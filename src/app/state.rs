use crate::models::{Guild, DmChannel, ChannelList};
use std::collections::HashMap;

pub fn get_channel_name(
    selected_channel: &Option<String>,
    dms: &[DmChannel],
    channel_cache: &HashMap<String, ChannelList>,
) -> Option<String> {
    selected_channel.as_ref().and_then(|id| {
        // Check if it's a DM
        if let Some(dm) = dms.iter().find(|dm| &dm.id == id) {
            return Some(dm.display_name());
        }
        
        // Check if it's a guild channel
        for channel_list in channel_cache.values() {
            if let Some(channel) = channel_list.channels.iter().find(|c| &c.id == id) {
                return Some(channel.name.clone());
            }
        }
        None
    })
}

pub fn get_guild_name(
    selected_channel: &Option<String>,
    dms: &[DmChannel],
    guilds: &[Guild],
    channel_cache: &HashMap<String, ChannelList>,
) -> Option<String> {
    selected_channel.as_ref().and_then(|channel_id| {
        // Check if it's a DM
        if dms.iter().any(|dm| &dm.id == channel_id) {
            return Some("Direct Messages".to_string());
        }
        
        // Check guild channels
        for (guild_id, channel_list) in channel_cache {
            if channel_list.channels.iter().any(|c| &c.id == channel_id) {
                return guilds.iter()
                    .find(|g| &g.id == guild_id)
                    .map(|g| g.name.clone());
            }
        }
        None
    })
}
