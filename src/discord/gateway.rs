use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::time::{interval, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::models::*;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsWrite = futures_util::stream::SplitSink<WsStream, Message>;

pub struct DiscordGateway {
    token: String,
    user: Option<DiscordUser>,
}

impl DiscordGateway {
    pub async fn new(token: String) -> Result<Self> {
        Ok(Self {
            token,
            user: None,
        })
    }
    
    /// Connect to Discord Gateway and login
    pub async fn connect(&mut self) -> Result<()> {
        let gateway_url = self.get_gateway_url().await?;
        let ws_url = format!("{}/?v=10&encoding=json", gateway_url);
        
        println!("Connecting to Discord Gateway...");
        
        let (ws_stream, _) = connect_async(&ws_url)
            .await
            .context("Failed to connect to Discord Gateway")?;
        
        let (write, mut read) = ws_stream.split();
        let write = Arc::new(Mutex::new(write));
        let last_seq = Arc::new(Mutex::new(None::<u64>));
        
        if let Some(Ok(Message::Text(text))) = read.next().await {
            let payload: GatewayPayload = serde_json::from_str(&text)
                .context("Failed to parse HELLO payload")?;
            
            if payload.op == HELLO {
                println!("Received HELLO from Discord");
                
                let heartbeat_interval = payload.d
                    .as_ref()
                    .and_then(|d| d["heartbeat_interval"].as_u64())
                    .context("Missing heartbeat_interval")?;
                
                println!("Heartbeat interval: {}ms", heartbeat_interval);
                
                let write_clone = write.clone();
                let seq_clone = last_seq.clone();
                tokio::spawn(async move {
                    Self::heartbeat_task(write_clone, seq_clone, heartbeat_interval).await;
                });
                
                self.send_identify(&write).await?;
                
                while let Some(msg) = read.next().await {
                    match msg? {
                        Message::Text(text) => {
                            let payload: GatewayPayload = serde_json::from_str(&text)?;
                            
                            if let Some(s) = payload.s {
                                *last_seq.lock().await = Some(s);
                            }
                            
                            match payload.op {
                                DISPATCH => {
                                    if let Some(event_type) = &payload.t {
                                        if event_type == "READY" {
                                            self.handle_ready(payload.d.as_ref().unwrap()).await?;
                                            println!("\nSuccessfully logged in as: {}", 
                                                self.user.as_ref().unwrap().username);
                                            return Ok(());
                                        }
                                    }
                                }
                                HEARTBEAT_ACK => {
                                    // Heartbeat acknowledged
                                }
                                _ => {}
                            }
                        }
                        Message::Close(frame) => {
                            if let Some(frame) = frame {
                                anyhow::bail!("Gateway closed: {} - {}", frame.code, frame.reason);
                            } else {
                                anyhow::bail!("Gateway closed unexpectedly");
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        anyhow::bail!("Failed to receive HELLO event")
    }
    
    async fn get_gateway_url(&self) -> Result<String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://discord.com/api/v10/gateway")
            .send()
            .await
            .context("Failed to fetch gateway URL")?;
        
        let gateway: GatewayResponse = response
            .json()
            .await
            .context("Failed to parse gateway response")?;
        
        Ok(gateway.url)
    }
    
    async fn send_identify(&self, write: &Arc<Mutex<WsWrite>>) -> Result<()> {
        let identify = json!({
            "op": IDENTIFY,
            "d": {
                "token": self.token,
                "properties": {
                    "os": std::env::consts::OS,
                    "browser": "remycord",
                    "device": "remycord"
                },
                "compress": false,
                "large_threshold": 250,
                "intents": 33281  // GUILDS | GUILD_MESSAGES | MESSAGE_CONTENT
            }
        });
        
        println!("Sending IDENTIFY...");
        
        let mut write = write.lock().await;
        write.send(Message::Text(identify.to_string()))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send IDENTIFY: {}", e))?;
        
        Ok(())
    }
    
    async fn handle_ready(&mut self, data: &serde_json::Value) -> Result<()> {
        let ready: ReadyEvent = serde_json::from_value(data.clone())
            .context("Failed to parse READY event")?;
        
        println!("Received READY event");
        println!("User ID: {}", ready.user.id);
        println!("Username: {}#{}", ready.user.username, ready.user.discriminator);
        println!("Guilds: {}", ready.guilds.len());
        
        self.user = Some(ready.user);
        
        Ok(())
    }
    
    async fn heartbeat_task(
        write: Arc<Mutex<WsWrite>>,
        last_seq: Arc<Mutex<Option<u64>>>,
        heartbeat_interval: u64,
    ) {
        let mut ticker = interval(Duration::from_millis(heartbeat_interval));
        
        loop {
            ticker.tick().await;
            
            let seq = *last_seq.lock().await;
            let heartbeat = json!({
                "op": HEARTBEAT,
                "d": seq
            });
            
            let mut write = write.lock().await;
            if let Err(e) = write.send(Message::Text(heartbeat.to_string())).await {
                eprintln!("Heartbeat failed: {}", e);
                break;
            }
        }
    }
    
    pub fn get_user(&self) -> Option<&DiscordUser> {
        self.user.as_ref()
    }
}
