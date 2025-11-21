mod theme;
mod keybinds;

pub use theme::{Theme, load_theme, create_default_theme, get_available_themes};
pub use keybinds::{Keybinds, KeyBind};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralSettings,
    pub images: ImageSettings,
    pub theme_name: String,
    pub keybinds: Keybinds,
    #[serde(skip)]
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub username: String,
    pub file_manager: String,
    pub show_timestamps: bool,
    pub show_typing_indicators: bool,
    pub message_scroll_speed: usize,
    pub max_input_lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSettings {
    pub enabled: bool,
    pub render_avatars: bool,
    pub render_emojis: bool,
    pub render_stickers: bool,
    pub render_attachments: bool,
    pub render_server_icons: bool,
    pub min_image_width: u16,
    pub max_image_width: u16,
    pub max_image_height: u16,
    pub aspect_ratio_correction: f32,
    pub auto_load_images: bool,
    pub cache_images_disk: bool,
    pub max_cache_size_mb: usize,
    pub image_quality: ImageQuality,
    pub cache_auto_clear: CacheAutoClear,
    pub cache_clear_on_exit: bool,
    pub cache_warn_threshold_percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheAutoClear {
    Never,
    WhenFull,
    Every30Minutes,
    EveryHour,
    EveryDay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageQuality {
    Low,
    Medium,
    High,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            images: ImageSettings::default(),
            theme_name: "oxocarbon-dark".to_string(),
            keybinds: Keybinds::default(),
            theme: Theme::default(),
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            username: "You".to_string(),
            file_manager: "fzf".to_string(),
            show_timestamps: true,
            show_typing_indicators: true,
            message_scroll_speed: 1,
            max_input_lines: 8,
        }
    }
}

impl Default for ImageSettings {
    fn default() -> Self {
        let in_kitty = std::env::var("TERM")
            .unwrap_or_default()
            .contains("kitty") || std::env::var("KITTY_WINDOW_ID").is_ok();
        
        Self {
            enabled: in_kitty,
            render_avatars: true,
            render_emojis: true,
            render_stickers: true,
            render_attachments: true,
            render_server_icons: true,
            min_image_width: 20,
            max_image_width: 40,
            max_image_height: 20,
            aspect_ratio_correction: 2.0,
            auto_load_images: true,
            cache_images_disk: true,
            max_cache_size_mb: 100,
            image_quality: ImageQuality::High,
            cache_auto_clear: CacheAutoClear::WhenFull,
            cache_clear_on_exit: false,
            cache_warn_threshold_percent: 80,
        }
    }
}

impl ImageQuality {
    pub fn to_filter_type(&self) -> image::imageops::FilterType {
        match self {
            ImageQuality::Low => image::imageops::FilterType::Nearest,
            ImageQuality::Medium => image::imageops::FilterType::Triangle,
            ImageQuality::High => image::imageops::FilterType::Lanczos3,
        }
    }
}

impl CacheAutoClear {
    pub fn duration_secs(&self) -> Option<u64> {
        match self {
            CacheAutoClear::Never => None,
            CacheAutoClear::WhenFull => None,
            CacheAutoClear::Every30Minutes => Some(30 * 60),
            CacheAutoClear::EveryHour => Some(60 * 60),
            CacheAutoClear::EveryDay => Some(24 * 60 * 60),
        }
    }
    
    pub fn next(&self) -> Self {
        match self {
            CacheAutoClear::Never => CacheAutoClear::WhenFull,
            CacheAutoClear::WhenFull => CacheAutoClear::Every30Minutes,
            CacheAutoClear::Every30Minutes => CacheAutoClear::EveryHour,
            CacheAutoClear::EveryHour => CacheAutoClear::EveryDay,
            CacheAutoClear::EveryDay => CacheAutoClear::Never,
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            CacheAutoClear::Never => "Never",
            CacheAutoClear::WhenFull => "When Full",
            CacheAutoClear::Every30Minutes => "Every 30 Minutes",
            CacheAutoClear::EveryHour => "Every Hour",
            CacheAutoClear::EveryDay => "Every Day",
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("remycord");
    
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.toml"))
}

pub fn get_themes_dir() -> Result<PathBuf> {
    let themes_dir = get_config_dir()?.join("themes");
    fs::create_dir_all(&themes_dir)?;
    Ok(themes_dir)
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    println!("Loading config from {:?}", config_path);
    
    if config_path.exists() {
        let contents = fs::read_to_string(&config_path)?;
        let mut config: Config = toml::from_str(&contents)?;
        
        config.theme = load_theme(&config.theme_name).unwrap_or_else(|_| {
            eprintln!("Warning: Could not load theme '{}', using default", config.theme_name);
            Theme::default()
        });
        
        Ok(config)
    } else {
        let mut config = Config::default();
        create_default_theme()?;
        config.theme = load_theme(&config.theme_name).unwrap_or_else(|_| Theme::default());
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let contents = toml::to_string_pretty(config)?;
    fs::write(config_path, contents)?;
    Ok(())
}
