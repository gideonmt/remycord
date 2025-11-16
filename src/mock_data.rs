use crate::app::App;
use crate::models::{Guild, Channel, ChannelType, Message};

pub fn load(app: &mut App) {
    app.guilds = vec![
        Guild::new("1", "Server One"),
        Guild::new("2", "Server Two"),
        Guild::new("3", "Server Three"),
    ];

    app.channel_cache.insert("1".to_string(), vec![
        Channel::new("1-1", "general", ChannelType::Text),
        Channel::new("1-2", "random", ChannelType::Text),
        Channel::new("1-3", "voice-chat", ChannelType::Voice),
    ]);

    app.channel_cache.insert("2".to_string(), vec![
        Channel::new("2-1", "announcements", ChannelType::Text),
        Channel::new("2-2", "discussion", ChannelType::Text),
    ]);

    app.channel_cache.insert("3".to_string(), vec![
        Channel::new("3-1", "welcome", ChannelType::Text),
    ]);

    app.message_cache.insert("1-1".to_string(), vec![
        Message::new("m1", "1-1", "Alice", "Hello everyone!", "10:30:00"),
        Message::new("m2", "1-1", "Bob", "Hey Alice, how are you?", "10:31:15"),
        Message::new("m3", "1-1", "Charlie", "Good morning!", "10:32:30"),
        Message::new("m4", "1-1", "Alice", 
            "I'm doing great, thanks! Just working on this new project.", "10:33:00"),
    ]);

    app.message_cache.insert("1-2".to_string(), vec![
        Message::new("m5", "1-2", "David", "Anyone up for a game later?", "11:00:00"),
    ]);

    app.typing_users = vec!["Bob".to_string()];
}
