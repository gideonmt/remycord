use crate::models::{Guild, Channel, DmChannel, ChannelList, ChannelCategory};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum SidebarItem {
    DmSection,
    DmChannel(DmChannel),
    ServerSection,
    Server(Guild),
    Category { guild_id: String, category: ChannelCategory, expanded: bool },
    Channel { guild_id: String, channel: Channel },
}

pub fn get_items(
    dms: &[DmChannel],
    dm_section_expanded: bool,
    guilds: &[Guild],
    channel_cache: &HashMap<String, ChannelList>,
    expanded_categories: &HashMap<String, HashSet<String>>,
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
        
        if !guild.expanded {
            continue;
        }
        
        let Some(channel_list) = channel_cache.get(&guild.id) else {
            continue;
        };
        
        let guild_expanded_cats = expanded_categories
            .get(&guild.id)
            .cloned()
            .unwrap_or_default();
        
        for channel in channel_list.uncategorized_channels() {
            items.push(SidebarItem::Channel {
                guild_id: guild.id.clone(),
                channel: channel.clone(),
            });
        }
        
        for category in &channel_list.categories {
            let is_expanded = guild_expanded_cats.contains(&category.id);
            
            items.push(SidebarItem::Category {
                guild_id: guild.id.clone(),
                category: category.clone(),
                expanded: is_expanded,
            });
            
            if is_expanded {
                for channel in channel_list.channels_in_category(&category.id) {
                    items.push(SidebarItem::Channel {
                        guild_id: guild.id.clone(),
                        channel: channel.clone(),
                    });
                }
            }
        }
    }
    
    items
}
