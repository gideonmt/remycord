use anyhow::Result;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;

use super::get_themes_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub author: Option<String>,
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    pub base0A: String,
    pub base0B: String,
    pub base0C: String,
    pub base0D: String,
    pub base0E: String,
    pub base0F: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self::oxocarbon_dark()
    }
}

impl Theme {
    pub fn oxocarbon_dark() -> Self {
        Self {
            name: "Oxocarbon Dark".to_string(),
            author: Some("shaunsingh/IBM".to_string()),
            base00: "#161616".to_string(),
            base01: "#262626".to_string(),
            base02: "#393939".to_string(),
            base03: "#525252".to_string(),
            base04: "#dde1e6".to_string(),
            base05: "#f2f4f8".to_string(),
            base06: "#ffffff".to_string(),
            base07: "#08bdba".to_string(),
            base08: "#3ddbd9".to_string(),
            base09: "#78a9ff".to_string(),
            base0A: "#ee5396".to_string(),
            base0B: "#33b1ff".to_string(),
            base0C: "#ff7eb6".to_string(),
            base0D: "#42be65".to_string(),
            base0E: "#be95ff".to_string(),
            base0F: "#82cfff".to_string(),
        }
    }

    pub fn get_color(&self, base: &str) -> Color {
        let hex = match base {
            "base00" => &self.base00,
            "base01" => &self.base01,
            "base02" => &self.base02,
            "base03" => &self.base03,
            "base04" => &self.base04,
            "base05" => &self.base05,
            "base06" => &self.base06,
            "base07" => &self.base07,
            "base08" => &self.base08,
            "base09" => &self.base09,
            "base0A" => &self.base0A,
            "base0B" => &self.base0B,
            "base0C" => &self.base0C,
            "base0D" => &self.base0D,
            "base0E" => &self.base0E,
            "base0F" => &self.base0F,
            _ => &self.base05,
        };
        hex_to_color(hex)
    }
}

pub fn load_theme(theme_name: &str) -> Result<Theme> {
    let themes_dir = get_themes_dir()?;
    let theme_path = themes_dir.join(format!("{}.yaml", theme_name));
    
    if theme_path.exists() {
        let contents = fs::read_to_string(&theme_path)?;
        let theme: Theme = serde_yaml::from_str(&contents)?;
        Ok(theme)
    } else {
        Ok(Theme::default())
    }
}

pub fn create_default_theme() -> Result<()> {
    let themes_dir = get_themes_dir()?;
    let theme_path = themes_dir.join("oxocarbon-dark.yaml");
    
    if !theme_path.exists() {
        let theme = Theme::oxocarbon_dark();
        let contents = serde_yaml::to_string(&theme)?;
        fs::write(theme_path, contents)?;
    }
    
    Ok(())
}

pub fn get_available_themes() -> Result<Vec<String>> {
    let themes_dir = get_themes_dir()?;
    let mut themes = Vec::new();
    
    if let Ok(entries) = fs::read_dir(themes_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".yaml") {
                    let theme_name = filename.trim_end_matches(".yaml").to_string();
                    themes.push(theme_name);
                }
            }
        }
    }
    
    themes.sort();
    Ok(themes)
}

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Color::White;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

    Color::Rgb(r, g, b)
}
