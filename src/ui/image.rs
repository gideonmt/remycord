use anyhow::Result;
use image::{DynamicImage, imageops::FilterType};
use ratatui_image::picker::{Picker, ProtocolType};
use ratatui_image::protocol::StatefulProtocol;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

pub struct ImageRenderer {
    picker: Picker,
    avatar_cache: HashMap<String, StatefulProtocol>,
    attachment_cache: HashMap<String, CachedAttachment>,
}

struct CachedAttachment {
    protocol: StatefulProtocol,
    original_width: u32,
    original_height: u32,
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

    pub fn get_avatar_url(user_id: &str, avatar_hash: Option<&str>) -> String {
        if let Some(hash) = avatar_hash {
            format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", user_id, hash)
        } else {
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
        
        if let Ok(protocol) = self.load_from_disk_cache(&format!("avatar_{}", user_id)).await {
            self.avatar_cache.insert(user_id.to_string(), protocol);
            return Ok(());
        }
        
        match self.download_and_process_image(&url, 32, 32).await {
            Ok(protocol) => {
                let _ = self.save_to_disk_cache(&format!("avatar_{}", user_id), &url).await;
                self.avatar_cache.insert(user_id.to_string(), protocol);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to load avatar for {}: {}", user_id, e);
                Err(e)
            }
        }
    }

    /// Download and cache an attachment image with proper sizing
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

        if let Ok((protocol, width, height)) = self.load_attachment_from_disk_cache(attachment_id).await {
            self.attachment_cache.insert(attachment_id.to_string(), CachedAttachment {
                protocol,
                original_width: width,
                original_height: height,
            });
            return Ok(());
        }

        match self.download_image(url).await {
            Ok(img) => {
                let original_width = img.width();
                let original_height = img.height();
                
                // Calculate optimal size maintaining aspect ratio
                let (target_width, target_height) = self.calculate_target_dimensions(
                    original_width,
                    original_height,
                    max_width as u32,
                    max_height as u32,
                );
                
                // Resize with high-quality filter
                let resized = img.resize_exact(
                    target_width,
                    target_height,
                    FilterType::Lanczos3
                );
                
                let protocol = self.picker.new_resize_protocol(resized);
                
                // Save to disk cache
                let _ = self.save_attachment_to_disk_cache(
                    attachment_id,
                    url,
                    original_width,
                    original_height
                ).await;
                
                self.attachment_cache.insert(attachment_id.to_string(), CachedAttachment {
                    protocol,
                    original_width,
                    original_height,
                });
                
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to load attachment {}: {}", attachment_id, e);
                Err(e)
            }
        }
    }

    /// Download image from URL
    async fn download_image(&self, url: &str) -> Result<DynamicImage> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        let img = image::load_from_memory(&bytes)?;
        Ok(img)
    }

    /// Download image from URL and process it
    async fn download_and_process_image(
        &mut self,
        url: &str,
        max_width: u32,
        max_height: u32,
    ) -> Result<StatefulProtocol> {
        let img = self.download_image(url).await?;
        
        let (target_width, target_height) = self.calculate_target_dimensions(
            img.width(),
            img.height(),
            max_width,
            max_height,
        );
        
        let resized = img.resize_exact(target_width, target_height, FilterType::Lanczos3);
        let protocol = self.picker.new_resize_protocol(resized);
        
        Ok(protocol)
    }

    /// Calculate target dimensions maintaining aspect ratio
    fn calculate_target_dimensions(
        &self,
        original_width: u32,
        original_height: u32,
        max_width: u32,
        max_height: u32,
    ) -> (u32, u32) {
        let aspect_ratio = original_width as f32 / original_height as f32;
        let max_ratio = max_width as f32 / max_height as f32;

        let (w, h) = if aspect_ratio > max_ratio {
            let w = max_width;
            let h = (w as f32 / aspect_ratio) as u32;
            (w, h)
        } else {
            let h = max_height;
            let w = (h as f32 * aspect_ratio) as u32;
            (w, h)
        };

        (w, h)
    }


    /// Get cache directory
    fn get_cache_dir() -> Result<PathBuf> {
        crate::config::get_image_cache_dir()
    }

    /// Load image from disk cache
    async fn load_from_disk_cache(&mut self, key: &str) -> Result<StatefulProtocol> {
        let cache_dir = Self::get_cache_dir()?;
        let cache_path = cache_dir.join(format!("{}.png", key));
        
        if cache_path.exists() {
            let bytes = fs::read(&cache_path).await?;
            let img = image::load_from_memory(&bytes)?;
            let protocol = self.picker.new_resize_protocol(img);
            Ok(protocol)
        } else {
            Err(anyhow::anyhow!("Cache file not found"))
        }
    }

    /// Load attachment from disk cache with metadata
    async fn load_attachment_from_disk_cache(&mut self, key: &str) -> Result<(StatefulProtocol, u32, u32)> {
        let cache_dir = Self::get_cache_dir()?;
        let cache_path = cache_dir.join(format!("{}.png", key));
        let meta_path = cache_dir.join(format!("{}.meta", key));
        
        if cache_path.exists() && meta_path.exists() {
            let bytes = fs::read(&cache_path).await?;
            let meta = fs::read_to_string(&meta_path).await?;
            
            let dimensions: Vec<u32> = meta
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            
            if dimensions.len() >= 2 {
                let img = image::load_from_memory(&bytes)?;
                let protocol = self.picker.new_resize_protocol(img);
                Ok((protocol, dimensions[0], dimensions[1]))
            } else {
                Err(anyhow::anyhow!("Invalid metadata"))
            }
        } else {
            Err(anyhow::anyhow!("Cache file not found"))
        }
    }

    /// Save image to disk cache
    async fn save_to_disk_cache(&self, key: &str, url: &str) -> Result<()> {
        let cache_dir = Self::get_cache_dir()?;
        let cache_path = cache_dir.join(format!("{}.png", key));
        
        if !cache_path.exists() {
            let response = reqwest::get(url).await?;
            let bytes = response.bytes().await?;
            fs::write(&cache_path, &bytes).await?;
        }
        
        Ok(())
    }

    /// Save attachment to disk cache with metadata
    async fn save_attachment_to_disk_cache(
        &self,
        key: &str,
        url: &str,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let cache_dir = Self::get_cache_dir()?;
        let cache_path = cache_dir.join(format!("{}.png", key));
        let meta_path = cache_dir.join(format!("{}.meta", key));
        
        if !cache_path.exists() {
            let response = reqwest::get(url).await?;
            let bytes = response.bytes().await?;
            fs::write(&cache_path, &bytes).await?;
            fs::write(&meta_path, format!("{},{}", width, height)).await?;
        }
        
        Ok(())
    }

    /// Get a cached avatar protocol for rendering
    pub fn get_avatar(&mut self, user_id: &str) -> Option<&mut StatefulProtocol> {
        self.avatar_cache.get_mut(user_id)
    }

    /// Get a cached attachment protocol for rendering
    pub fn get_attachment(&mut self, attachment_id: &str) -> Option<&mut StatefulProtocol> {
        self.attachment_cache.get_mut(attachment_id).map(|cached| &mut cached.protocol)
    }

    /// Get original dimensions of cached attachment
    pub fn get_attachment_dimensions(&self, attachment_id: &str) -> Option<(u32, u32)> {
        self.attachment_cache.get(attachment_id).map(|cached| {
            (cached.original_width, cached.original_height)
        })
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

    /// Clear disk cache
    pub async fn clear_disk_cache() -> Result<()> {
        let cache_dir = Self::get_cache_dir()?;
        if cache_dir.exists() {
            fs::remove_dir_all(&cache_dir).await?;
            fs::create_dir_all(&cache_dir).await?;
        }
        Ok(())
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
    fn test_calculate_target_dimensions() {
        let renderer = ImageRenderer::new();
        
        // Landscape image
        let (w, h) = renderer.calculate_target_dimensions(1920, 1080, 30, 15);
        assert!(w <= 30 && h <= 15);
        assert!((w as f32 / h as f32 - 1920.0 / 1080.0).abs() < 0.1);
        
        // Portrait image
        let (w, h) = renderer.calculate_target_dimensions(1080, 1920, 30, 15);
        assert!(w <= 30 && h <= 15);
        assert!((w as f32 / h as f32 - 1080.0 / 1920.0).abs() < 0.1);
        
        // Square image
        let (w, h) = renderer.calculate_target_dimensions(1000, 1000, 30, 15);
        assert!(w <= 30 && h <= 15);
        assert_eq!(w, h);
    }
}
