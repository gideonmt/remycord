use anyhow::Result;
use serenity::prelude::*;
use serenity::model::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::models::{
    Guild as AppGuild, 
    Channel as AppChannel,
    ChannelList,
    ChannelCategory,
    ChannelType, 
    Message as AppMessage,
    MessageAttachment as AppAttachment,
    DmChannel as AppDmChannel,
    DmUser,
};

#[derive(Debug, Clone)]
pub enum DiscordEvent {
    Ready(Vec<AppGuild>),
    Connected(String),
    DmChannels(Vec<AppDmChannel>),
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
            | GatewayIntents::DIRECT_MESSAGES
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
    
    pub async fn fetch_dms(&self) -> Result<Vec<AppDmChannel>> {
        let channels = self.http.get_user_dm_channels().await?;
        
        let dm_channels: Vec<AppDmChannel> = channels
            .into_iter()
            .filter_map(|c| {
                if let Some(recipient) = c.recipients.first() {
                    let discriminator = recipient.discriminator
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| "0".to_string());
                    
                    Some(AppDmChannel::new(
                        c.id.to_string(),
                        DmUser::new(
                            recipient.id.to_string(),
                            recipient.name.clone(),
                            discriminator,
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect();
        
        Ok(dm_channels)
    }
    
    pub async fn fetch_channels(&self, guild_id: &str) -> Result<ChannelList> {
        let guild_id = guild_id.parse::<GuildId>()?;
        let channels = self.http.get_channels(guild_id).await?;
        
        let mut channel_list = ChannelList::new();
        
        for channel in channels {
            match channel.kind {
                serenity::model::channel::ChannelType::Category => {
                    channel_list.categories.push(ChannelCategory::new(
                        channel.id.to_string(),
                        channel.name,
                        i32::from(channel.position),
                    ));
                }
                serenity::model::channel::ChannelType::Text => {
                    let mut ch = AppChannel::new(
                        channel.id.to_string(),
                        channel.name,
                        ChannelType::Text,
                        i32::from(channel.position),
                    );
                    if let Some(parent) = channel.parent_id {
                        ch = ch.with_parent(parent.to_string());
                    }
                    channel_list.channels.push(ch);
                }
                serenity::model::channel::ChannelType::Voice => {
                    let mut ch = AppChannel::new(
                        channel.id.to_string(),
                        channel.name,
                        ChannelType::Voice,
                        i32::from(channel.position),
                    );
                    if let Some(parent) = channel.parent_id {
                        ch = ch.with_parent(parent.to_string());
                    }
                    channel_list.channels.push(ch);
                }
                serenity::model::channel::ChannelType::News => {
                    let mut ch = AppChannel::new(
                        channel.id.to_string(),
                        channel.name,
                        ChannelType::Announcement,
                        i32::from(channel.position),
                    );
                    if let Some(parent) = channel.parent_id {
                        ch = ch.with_parent(parent.to_string());
                    }
                    channel_list.channels.push(ch);
                }
                serenity::model::channel::ChannelType::Stage => {
                    let mut ch = AppChannel::new(
                        channel.id.to_string(),
                        channel.name,
                        ChannelType::Stage,
                        i32::from(channel.position),
                    );
                    if let Some(parent) = channel.parent_id {
                        ch = ch.with_parent(parent.to_string());
                    }
                    channel_list.channels.push(ch);
                }
                serenity::model::channel::ChannelType::PublicThread
                | serenity::model::channel::ChannelType::PrivateThread
                | serenity::model::channel::ChannelType::NewsThread => {
                    continue;
                }
                _ => {
                    continue;
                }
            }
        }
        
        channel_list.sort();
        
        Ok(channel_list)
    }
    
    pub async fn fetch_messages(&self, channel_id: &str, limit: u8) -> Result<Vec<AppMessage>> {
        let channel_id = channel_id.parse::<ChannelId>()?;
        let messages = self.http.get_messages(channel_id, None, Some(limit)).await?;
        
        let mut app_messages: Vec<AppMessage> = messages
            .into_iter()
            .map(|m| {
                let timestamp = m.timestamp.format("%H:%M:%S").to_string();
                
                let attachments: Vec<AppAttachment> = m.attachments
                    .into_iter()
                    .map(|a| AppAttachment::new(
                        a.id.to_string(),
                        a.filename,
                        a.url,
                        a.proxy_url,
                        a.width,
                        a.height,
                        a.content_type,
                    ))
                    .collect();
                
                AppMessage::new(
                    m.id.to_string(),
                    channel_id.to_string(),
                    m.author.name,
                    m.author.id.to_string(),
                    m.author.avatar.map(|h| h.to_string()),
                    m.content,
                    timestamp,
                ).with_attachments(attachments)
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
            message.author.id.to_string(),
            message.author.avatar.map(|h| h.to_string()),
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
        
        let username = ready.user.name.clone();
        let _ = self.event_tx.send(DiscordEvent::Connected(username));
    }
    
    async fn message(&self, _ctx: Context, new_message: serenity::model::channel::Message) {
        let timestamp = new_message.timestamp.format("%H:%M:%S").to_string();
        
        let attachments: Vec<AppAttachment> = new_message.attachments
            .into_iter()
            .map(|a| AppAttachment::new(
                a.id.to_string(),
                a.filename,
                a.url,
                a.proxy_url,
                a.width,
                a.height,
                a.content_type,
            ))
            .collect();
        
        let app_message = AppMessage::new(
            new_message.id.to_string(),
            new_message.channel_id.to_string(),
            new_message.author.name,
            new_message.author.id.to_string(),
            new_message.author.avatar.map(|h| h.to_string()),
            new_message.content,
            timestamp,
        ).with_attachments(attachments);
        
        let _ = self.event_tx.send(DiscordEvent::NewMessage(app_message));
    }
}
