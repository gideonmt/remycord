use anyhow::Result;
use image::DynamicImage;
use ratatui_image::picker::{Picker, ProtocolType};
use ratatui_image::protocol::StatefulProtocol;
use std::collections::HashMap;

pub struct ImageRenderer {
    picker: Picker,
    avatar_cache: HashMap<String, StatefulProtocol>,
    attachment_cache: HashMap<String, StatefulProtocol>,
}

impl ImageRenderer {
    pub fn new() -> Self {
        let picker = Picker::from_query_stdio()
            .unwrap_or_else(|_| Picker::from_fontsize((8, 12)));
        
        Self {
            picker,
            avatar_cache: HashMap::new(),
            attachment_cache: HashMap::new(),
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

    /// Get Discord avatar URL for a user
    pub fn get_avatar_url(user_id: &str, avatar_hash: Option<&str>) -> String {
        if let Some(hash) = avatar_hash {
            // User has a custom avatar
            format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", user_id, hash)
        } else {
            // Use default avatar based on discriminator/user ID
            let default_num = user_id
                .chars()
                .last()
                .and_then(|c| c.to_digit(10))
                .unwrap_or(0) % 5;
            
            format!("https://cdn.discordapp.com/embed/avatars/{}.png", default_num)
        }
    }

    /// Download and cache an avatar image
    pub async fn load_avatar(&mut self, user_id: &str, avatar_hash: Option<&str>) -> Result<()> {
        if self.avatar_cache.contains_key(user_id) {
            return Ok(());
        }

        let url = Self::get_avatar_url(user_id, avatar_hash);
        
        match self.download_and_process_image(&url, 32, 32).await {
            Ok(protocol) => {
                self.avatar_cache.insert(user_id.to_string(), protocol);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to load avatar for {}: {}", user_id, e);
                Err(e)
            }
        }
    }

    /// Download and cache an attachment image
    pub async fn load_attachment(
        &mut self, 
        attachment_id: &str, 
        url: &str,
        max_width: u16,
        max_height: u16,
    ) -> Result<()> {
        if self.attachment_cache.contains_key(attachment_id) {
            return Ok(());
        }

        match self.download_and_process_image(url, max_width as u32, max_height as u32).await {
            Ok(protocol) => {
                self.attachment_cache.insert(attachment_id.to_string(), protocol);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to load attachment {}: {}", attachment_id, e);
                Err(e)
            }
        }
    }

    /// Download image from URL and process it
    async fn download_and_process_image(
        &mut self,
        url: &str,
        max_width: u32,
        max_height: u32,
    ) -> Result<StatefulProtocol> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        let img = image::load_from_memory(&bytes)?;
        
        // Resize maintaining aspect ratio
        let resized = img.resize(max_width, max_height, image::imageops::FilterType::Lanczos3);
        let protocol = self.picker.new_resize_protocol(resized);
        
        Ok(protocol)
    }

    /// Get a cached avatar protocol for rendering
    pub fn get_avatar(&mut self, user_id: &str) -> Option<&mut StatefulProtocol> {
        self.avatar_cache.get_mut(user_id)
    }

    /// Get a cached attachment protocol for rendering
    pub fn get_attachment(&mut self, attachment_id: &str) -> Option<&mut StatefulProtocol> {
        self.attachment_cache.get_mut(attachment_id)
    }

    /// Clear all caches
    pub fn clear_cache(&mut self) {
        self.avatar_cache.clear();
        self.attachment_cache.clear();
    }

    /// Clear only avatar cache
    pub fn clear_avatar_cache(&mut self) {
        self.avatar_cache.clear();
    }

    /// Clear only attachment cache
    pub fn clear_attachment_cache(&mut self) {
        self.attachment_cache.clear();
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
    fn test_avatar_url_with_hash() {
        let url = ImageRenderer::get_avatar_url("123456789", Some("abcdef123456"));
        assert_eq!(url, "https://cdn.discordapp.com/avatars/123456789/abcdef123456.png?size=128");
    }

    #[test]
    fn test_avatar_url_without_hash() {
        let url = ImageRenderer::get_avatar_url("123456789", None);
        assert!(url.starts_with("https://cdn.discordapp.com/embed/avatars/"));
        assert!(url.ends_with(".png"));
    }

    #[test]
    fn test_default_avatar_number_range() {
        for i in 0..10 {
            let user_id = format!("{}", i);
            let url = ImageRenderer::get_avatar_url(&user_id, None);
            let num = url
                .trim_end_matches(".png")
                .chars()
                .last()
                .and_then(|c| c.to_digit(10))
                .unwrap();
            assert!(num <= 4);
        }
    }
}
