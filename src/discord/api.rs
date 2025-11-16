use anyhow::Result;
use serenity::prelude::*;
use serenity::model::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::models::{Guild as AppGuild, Channel as AppChannel, ChannelType, Message as AppMessage};

#[derive(Debug, Clone)]
pub enum DiscordEvent {
    Ready(Vec<AppGuild>),
    GuildChannels(String, Vec<AppChannel>),
    Messages(String, Vec<AppMessage>),
    NewMessage(AppMessage),
    Error(String),
}

pub struct DiscordClient {
    http: Arc<serenity::http::Http>,
    event_tx: mpsc::UnboundedSender<DiscordEvent>,
}

impl DiscordClient {
    pub async fn new(token: String) -> Result<(Self, mpsc::UnboundedReceiver<DiscordEvent>)> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        
        let http = Arc::new(serenity::http::Http::new(&token));
        
        let client = Self {
            http,
            event_tx,
        };
        
        Ok((client, event_rx))
    }
    
    pub async fn start_gateway(&self, token: String) -> Result<()> {
        let intents = GatewayIntents::GUILDS 
            | GatewayIntents::GUILD_MESSAGES 
            | GatewayIntents::MESSAGE_CONTENT;
        
        let event_tx = self.event_tx.clone();
        let handler = Handler { event_tx };
        
        let mut client = Client::builder(token, intents)
            .event_handler(handler)
            .await?;
        
        tokio::spawn(async move {
            if let Err(why) = client.start().await {
                eprintln!("Client error: {:?}", why);
            }
        });
        
        Ok(())
    }
    
    pub async fn fetch_guilds(&self) -> Result<Vec<AppGuild>> {
        let guilds = self.http.get_guilds(None, None).await?;
        
        Ok(guilds.into_iter().map(|g| AppGuild::new(g.id.to_string(), g.name)).collect())
    }
    
    pub async fn fetch_channels(&self, guild_id: &str) -> Result<Vec<AppChannel>> {
        let guild_id = guild_id.parse::<GuildId>()?;
        let channels = self.http.get_channels(guild_id).await?;
        
        let mut app_channels: Vec<AppChannel> = channels
            .into_iter()
            .filter_map(|c| {
                match c.kind {
                    serenity::model::channel::ChannelType::Text => {
                        Some(AppChannel::new(c.id.to_string(), c.name, ChannelType::Text))
                    }
                    serenity::model::channel::ChannelType::Voice => {
                        Some(AppChannel::new(c.id.to_string(), c.name, ChannelType::Voice))
                    }
                    _ => None,
                }
            })
            .collect();
        
        app_channels.sort_by_key(|c| c.name.clone());
        Ok(app_channels)
    }
    
    pub async fn fetch_messages(&self, channel_id: &str, limit: u8) -> Result<Vec<AppMessage>> {
        let channel_id = channel_id.parse::<ChannelId>()?;
        let messages = self.http.get_messages(channel_id, None, Some(limit)).await?;
        
        let mut app_messages: Vec<AppMessage> = messages
            .into_iter()
            .map(|m| {
                let timestamp = m.timestamp.format("%H:%M:%S").to_string();
                AppMessage::new(
                    m.id.to_string(),
                    channel_id.to_string(),
                    m.author.name,
                    m.content,
                    timestamp,
                )
            })
            .collect();
        
        app_messages.reverse();
        Ok(app_messages)
    }
    
    pub async fn send_message(&self, channel_id: &str, content: &str) -> Result<AppMessage> {
        let channel_id = channel_id.parse::<ChannelId>()?;
        
        let message = channel_id.say(&self.http, content).await?;
        
        let timestamp = message.timestamp.format("%H:%M:%S").to_string();
        Ok(AppMessage::new(
            message.id.to_string(),
            channel_id.to_string(),
            message.author.name,
            message.content,
            timestamp,
        ))
    }
}

struct Handler {
    event_tx: mpsc::UnboundedSender<DiscordEvent>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as: {}", ready.user.name);
        
        let guilds: Vec<AppGuild> = ready.guilds
            .iter()
            .map(|g| AppGuild::new(g.id.to_string(), g.id.to_string()))
            .collect();
        
        let _ = self.event_tx.send(DiscordEvent::Ready(guilds));
        
        for guild in ready.guilds {
            if let Ok(full_guild) = ctx.http.get_guild(guild.id).await {
                let app_guild = AppGuild::new(full_guild.id.to_string(), full_guild.name);
                let _ = self.event_tx.send(DiscordEvent::Ready(vec![app_guild]));
            }
        }
    }
    
    async fn message(&self, _ctx: Context, new_message: serenity::model::channel::Message) {
        let timestamp = new_message.timestamp.format("%H:%M:%S").to_string();
        let app_message = AppMessage::new(
            new_message.id.to_string(),
            new_message.channel_id.to_string(),
            new_message.author.name,
            new_message.content,
            timestamp,
        );
        
        let _ = self.event_tx.send(DiscordEvent::NewMessage(app_message));
    }
}
