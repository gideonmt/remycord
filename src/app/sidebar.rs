use crate::models::{Guild, Channel, DmChannel};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SidebarItem {
    DmSection,
    DmChannel(DmChannel),
    ServerSection,
    Server(Guild),
    Channel(Channel),
}

pub fn get_items(
    dms: &[DmChannel],
    dm_section_expanded: bool,
    guilds: &[Guild],
    channel_cache: &HashMap<String, Vec<Channel>>,
) -> Vec<SidebarItem> {
    let mut items = Vec::new();
    
    // DMs section
    items.push(SidebarItem::DmSection);
    if dm_section_expanded {
        for dm in dms {
            items.push(SidebarItem::DmChannel(dm.clone()));
        }
    }
    
    // Servers section
    items.push(SidebarItem::ServerSection);
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
