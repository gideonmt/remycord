mod mode;
mod sidebar;
mod state;

pub use mode::AppMode;
pub use sidebar::SidebarItem;

use crate::config::{Config, Theme, load_theme, CacheAutoClear};
use crate::models::{Guild, Message, AttachedFile, DmChannel, Notification, ChannelList};
use crate::discord::DiscordClient;
use crate::ui::image::{ImageRenderer, CacheStats};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct App {
    pub mode: AppMode,
    pub dms: Vec<DmChannel>,
    pub dm_section_expanded: bool,
    pub guilds: Vec<Guild>,
    pub selected_sidebar_idx: usize,
    pub selected_channel: Option<String>,
    pub messages: Vec<Message>,
    pub message_scroll: usize,
    pub input: String,
    pub input_cursor: usize,
    pub attached_files: Vec<AttachedFile>,
    pub channel_cache: HashMap<String, ChannelList>,
    pub expanded_categories: HashMap<String, HashSet<String>>,
    pub message_cache: HashMap<String, Vec<Message>>,
    pub config: Config,
    pub typing_users: Vec<String>,
    pub settings_selected: usize,
    pub discord_client: Option<Arc<Mutex<DiscordClient>>>,
    pub loading_channels: bool,
    pub loading_messages: bool,
    pub loading_dms: bool,
    pub notifications: Vec<Notification>,
    pub image_renderer: ImageRenderer,
    pub cache_stats: Option<CacheStats>,
    pub last_cache_check: Instant,
    pub last_cache_clear: Instant,
    pub cache_warning_shown: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            mode: AppMode::Sidebar,
            dms: Vec::new(),
            dm_section_expanded: true,
            guilds: Vec::new(),
            selected_sidebar_idx: 0,
            selected_channel: None,
            messages: Vec::new(),
            message_scroll: 0,
            input: String::new(),
            input_cursor: 0,
            attached_files: Vec::new(),
            channel_cache: HashMap::new(),
            expanded_categories: HashMap::new(),
            message_cache: HashMap::new(),
            config,
            typing_users: Vec::new(),
            settings_selected: 0,
            discord_client: None,
            loading_channels: false,
            loading_messages: false,
            loading_dms: false,
            notifications: Vec::new(),
            image_renderer: ImageRenderer::new(),
                    cache_stats: None,
            last_cache_check: Instant::now(),
            last_cache_clear: Instant::now(),
            cache_warning_shown: false,
        }
    }
    
    pub fn toggle_category(&mut self, guild_id: &str, category_id: &str) {
        let guild_categories = self.expanded_categories
            .entry(guild_id.to_string())
            .or_insert_with(HashSet::new);
        
        if guild_categories.contains(category_id) {
            guild_categories.remove(category_id);
        } else {
            guild_categories.insert(category_id.to_string());
        }
    }
    
    pub fn is_category_expanded(&self, guild_id: &str, category_id: &str) -> bool {
        self.expanded_categories
            .get(guild_id)
            .map(|cats| cats.contains(category_id))
            .unwrap_or(false)
    }
    
    pub fn get_guild_channels(&self, guild_id: &str) -> Option<&ChannelList> {
        self.channel_cache.get(guild_id)
    }

    pub fn get_sidebar_items(&self) -> Vec<SidebarItem> {
        sidebar::get_items(
            &self.dms,
            self.dm_section_expanded,
            &self.guilds,
            &self.channel_cache,
            &self.expanded_categories,
        )
    }

    pub fn get_current_guild_name(&self) -> Option<String> {
        state::get_guild_name(
            &self.selected_channel,
            &self.dms,
            &self.guilds,
            &self.channel_cache,
        )
    }

    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn clear_expired_notifications(&mut self) {
        self.notifications.retain(|n| !n.is_expired());
    }

    pub fn theme(&self) -> &Theme {
        &self.config.theme
    }

    pub fn reload_theme(&mut self) {
        self.config.theme = load_theme(&self.config.theme_name)
            .unwrap_or_else(|_| Theme::default());
    }

    pub fn get_current_channel_name(&self) -> Option<String> {
        state::get_channel_name(&self.selected_channel, &self.dms, &self.channel_cache)
    }

    pub fn set_discord_client(&mut self, client: DiscordClient) {
        self.discord_client = Some(Arc::new(Mutex::new(client)));
    }

    pub async fn update_cache_stats(&mut self) {
        if let Ok(stats) = ImageRenderer::get_cache_stats().await {
            self.cache_stats = Some(stats);
        }
    }

    pub async fn check_cache_health(&mut self) {
        self.update_cache_stats().await;
        
        if let Some(stats) = &self.cache_stats {
            let max_size_bytes = (self.config.images.max_cache_size_mb * 1024 * 1024) as u64;
            let usage_percent = (stats.total_size_bytes as f64 / max_size_bytes as f64 * 100.0) as u8;
            
            if usage_percent >= self.config.images.cache_warn_threshold_percent && !self.cache_warning_shown {
                self.add_notification(Notification::warning(
                    format!("Cache is {}% full ({}/{}MB). Consider clearing cache in settings.", 
                        usage_percent, 
                        stats.total_size_mb() as usize,
                        self.config.images.max_cache_size_mb)
                ));
                self.cache_warning_shown = true;
            }
            
            if self.config.images.cache_auto_clear == CacheAutoClear::WhenFull 
                && usage_percent >= 95 {
                self.add_notification(Notification::info("Cache full, auto-clearing..."));
                let _ = ImageRenderer::clear_cache().await;
                self.image_renderer.clear_memory_cache();
                self.update_cache_stats().await;
                self.cache_warning_shown = false;
            }
            
            if usage_percent < self.config.images.cache_warn_threshold_percent {
                self.cache_warning_shown = false;
            }
        }
        
        self.last_cache_check = Instant::now();
    }
    
    pub async fn check_scheduled_cache_clear(&mut self) {
        if let Some(duration_secs) = self.config.images.cache_auto_clear.duration_secs() {
            if self.last_cache_clear.elapsed().as_secs() >= duration_secs {
                self.add_notification(Notification::info("Scheduled cache clear..."));
                let _ = ImageRenderer::clear_cache().await;
                self.image_renderer.clear_memory_cache();
                self.update_cache_stats().await;
                self.last_cache_clear = Instant::now();
            }
        }
    }
}
