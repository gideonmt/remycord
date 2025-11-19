use anyhow::Result;
use image::DynamicImage;
use ratatui_image::picker::{Picker, ProtocolType};
use ratatui_image::protocol::StatefulProtocol;
use std::collections::HashMap;

pub struct ImageRenderer {
    picker: Picker,
    avatar_cache: HashMap<String, StatefulProtocol>,
}

impl ImageRenderer {
    pub fn new() -> Self {
        let picker = Picker::from_query_stdio()
            .unwrap_or_else(|_| Picker::from_fontsize((8, 12)));
        
        Self {
            picker,
            avatar_cache: HashMap::new(),
        }
    }

    pub fn is_supported(&self) -> bool {
        !matches!(self.picker.protocol_type(), ProtocolType::Halfblocks)
    }

    pub fn protocol_name(&self) -> &str {
        match self.picker.protocol_type() {
            ProtocolType::Halfblocks => "Halfblocks",
            ProtocolType::Sixel => "Sixel",
            ProtocolType::Kitty => "Kitty",
            ProtocolType::Iterm2 => "iTerm2",
        }
    }

    /// Get a default Discord avatar URL for a user
    pub fn get_default_avatar_url(user_id: &str) -> String {
        // Use last digit of user ID to determine which default avatar (0-5)
        let avatar_num = user_id
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0) % 6;
        
        format!("https://cdn.discordapp.com/embed/avatars/{}.png", avatar_num)
    }

    /// Download and cache an avatar image
    pub async fn load_avatar(&mut self, user_id: &str) -> Result<()> {
        if self.avatar_cache.contains_key(user_id) {
            return Ok(());
        }

        let url = Self::get_default_avatar_url(user_id);
        let response = reqwest::get(&url).await?;
        let bytes = response.bytes().await?;
        let img = image::load_from_memory(&bytes)?;
        
        // Resize to a reasonable size for terminal display
        let resized = img.resize(32, 32, image::imageops::FilterType::Lanczos3);
        let protocol = self.picker.new_resize_protocol(resized);
        
        self.avatar_cache.insert(user_id.to_string(), protocol);
        
        Ok(())
    }

    /// Get a cached avatar protocol for rendering
    pub fn get_avatar(&mut self, user_id: &str) -> Option<&mut StatefulProtocol> {
        self.avatar_cache.get_mut(user_id)
    }

    /// Clear the avatar cache
    pub fn clear_cache(&mut self) {
        self.avatar_cache.clear();
    }

    /// Load an image from bytes
    pub fn load_image(&mut self, bytes: &[u8]) -> Result<StatefulProtocol> {
        let img = image::load_from_memory(bytes)?;
        let protocol = self.picker.new_resize_protocol(img);
        Ok(protocol)
    }

    /// Load an image from a DynamicImage
    pub fn load_dynamic_image(&mut self, img: DynamicImage) -> StatefulProtocol {
        self.picker.new_resize_protocol(img)
    }
}

impl Default for ImageRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_avatar_url() {
        let url = ImageRenderer::get_default_avatar_url("123456789");
        assert!(url.starts_with("https://cdn.discordapp.com/embed/avatars/"));
        assert!(url.ends_with(".png"));
    }

    #[test]
    fn test_avatar_number_range() {
        for i in 0..10 {
            let user_id = format!("{}", i);
            let url = ImageRenderer::get_default_avatar_url(&user_id);
            let num = url
                .trim_end_matches(".png")
                .chars()
                .last()
                .and_then(|c| c.to_digit(10))
                .unwrap();
            assert!(num <= 5);
        }
    }
}
