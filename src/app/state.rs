use crate::models::{Guild, Channel};
use std::collections::HashMap;

pub fn get_channel_name(
    selected_channel: &Option<String>,
    channel_cache: &HashMap<String, Vec<Channel>>,
) -> Option<String> {
    selected_channel.as_ref().and_then(|id| {
        for channels in channel_cache.values() {
            if let Some(channel) = channels.iter().find(|c| &c.id == id) {
                return Some(channel.name.clone());
            }
        }
        None
    })
}

pub fn get_guild_name(
    selected_channel: &Option<String>,
    guilds: &[Guild],
    channel_cache: &HashMap<String, Vec<Channel>>,
) -> Option<String> {
    selected_channel.as_ref().and_then(|channel_id| {
        for (guild_id, channels) in channel_cache {
            if channels.iter().any(|c| &c.id == channel_id) {
                return guilds.iter()
                    .find(|g| &g.id == guild_id)
                    .map(|g| g.name.clone());
            }
        }
        None
    })
}
