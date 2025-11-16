mod mode;
mod sidebar;
mod state;

pub use mode::AppMode;
pub use sidebar::SidebarItem;

use crate::config::{Config, Theme, load_theme};
use crate::models::{Guild, Channel, Message, AttachedFile};
use std::collections::HashMap;

pub struct App {
    pub mode: AppMode,
    pub guilds: Vec<Guild>,
    pub selected_sidebar_idx: usize,
    pub selected_channel: Option<String>,
    pub messages: Vec<Message>,
    pub message_scroll: usize,
    pub input: String,
    pub input_cursor: usize,
    pub attached_files: Vec<AttachedFile>,
    pub channel_cache: HashMap<String, Vec<Channel>>,
    pub message_cache: HashMap<String, Vec<Message>>,
    pub config: Config,
    pub typing_users: Vec<String>,
    pub settings_selected: usize,
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut app = Self {
            mode: AppMode::Sidebar,
            guilds: Vec::new(),
            selected_sidebar_idx: 0,
            selected_channel: None,
            messages: Vec::new(),
            message_scroll: 0,
            input: String::new(),
            input_cursor: 0,
            attached_files: Vec::new(),
            channel_cache: HashMap::new(),
            message_cache: HashMap::new(),
            config,
            typing_users: Vec::new(),
            settings_selected: 0,
        };
        
        app
    }

    pub fn theme(&self) -> &Theme {
        &self.config.theme
    }

    pub fn reload_theme(&mut self) {
        self.config.theme = load_theme(&self.config.theme_name)
            .unwrap_or_else(|_| Theme::default());
    }

    pub fn get_sidebar_items(&self) -> Vec<SidebarItem> {
        sidebar::get_items(&self.guilds, &self.channel_cache)
    }

    pub fn get_current_channel_name(&self) -> Option<String> {
        state::get_channel_name(&self.selected_channel, &self.channel_cache)
    }

    pub fn get_current_guild_name(&self) -> Option<String> {
        state::get_guild_name(&self.selected_channel, &self.guilds, &self.channel_cache)
    }
}
