use crate::models::{Guild, Channel};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SidebarItem {
    Server(Guild),
    Channel(Channel),
}

pub fn get_items(guilds: &[Guild], channel_cache: &HashMap<String, Vec<Channel>>) -> Vec<SidebarItem> {
    let mut items = Vec::new();
    
    for guild in guilds {
        items.push(SidebarItem::Server(guild.clone()));
        
        if guild.expanded {
            if let Some(channels) = channel_cache.get(&guild.id) {
                for channel in channels {
                    items.push(SidebarItem::Channel(channel.clone()));
                }
            }
        }
    }
    
    items
}
