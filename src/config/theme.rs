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
        // Minimal fallback - should never be used in practice
        Self {
            name: "Fallback".to_string(),
            author: None,
            base00: "#000000".to_string(),
            base01: "#111111".to_string(),
            base02: "#222222".to_string(),
            base03: "#333333".to_string(),
            base04: "#cccccc".to_string(),
            base05: "#dddddd".to_string(),
            base06: "#eeeeee".to_string(),
            base07: "#ffffff".to_string(),
            base08: "#ff0000".to_string(),
            base09: "#ff8800".to_string(),
            base0A: "#ffff00".to_string(),
            base0B: "#00ff00".to_string(),
            base0C: "#00ffff".to_string(),
            base0D: "#0088ff".to_string(),
            base0E: "#8800ff".to_string(),
            base0F: "#ff00ff".to_string(),
        }
    }
}

impl Theme {

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
        // Write the YAML content directly
        let yaml_content = "name: \"Oxocarbon Dark\"
author: \"shaunsingh/IBM\"
base00: \"#161616\"
base01: \"#262626\"
base02: \"#393939\"
base03: \"#525252\"
base04: \"#dde1e6\"
base05: \"#f2f4f8\"
base06: \"#ffffff\"
base07: \"#08bdba\"
base08: \"#3ddbd9\"
base09: \"#78a9ff\"
base0A: \"#ee5396\"
base0B: \"#33b1ff\"
base0C: \"#ff7eb6\"
base0D: \"#42be65\"
base0E: \"#be95ff\"
base0F: \"#82cfff\"
";
        fs::write(theme_path, yaml_content)?;
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
