use anyhow::Result;
use image::{DynamicImage, imageops::FilterType, GenericImageView};
use ratatui_image::picker::{Picker, ProtocolType};
use ratatui_image::protocol::StatefulProtocol;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

pub struct ImageRenderer {
    picker: Picker,
    avatar_cache: HashMap<String, CachedAvatar>,
    attachment_cache: HashMap<String, CachedAttachment>,
}

struct CachedAvatar {
    image: DynamicImage,
    protocol: StatefulProtocol,
}

struct CachedAttachment {
    image: DynamicImage,
    protocol: StatefulProtocol,
    terminal_width: u16,
    terminal_height: u16,
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

    async fn save_processed_avatar_to_disk(&self, key: &str, img: &DynamicImage) -> Result<()> {
        let dir = Self::get_cache_dir()?;
        let path = dir.join(format!("{}.png", key));
        if !path.exists() {
            img.save(&path)?;
        }
        Ok(())
    }

    async fn process_and_cache_avatar(&mut self, url: &str, key: &str) -> Result<(DynamicImage, StatefulProtocol)> {
        let img = self.download_image(url).await?;
        let resized = img.resize_exact(128, 128, FilterType::Lanczos3);
        let masked = Self::circle_mask(resized);
        self.save_processed_avatar_to_disk(key, &masked).await?;
        let protocol = self.picker.new_resize_protocol(masked.clone());
        Ok((masked, protocol))
    }

    pub async fn load_avatar(&mut self, user_id: &str, avatar_hash: Option<&str>) -> Result<()> {
        if self.avatar_cache.contains_key(user_id) {
            return Ok(());
        }

        let url = Self::get_avatar_url(user_id, avatar_hash);
        let key = format!("avatar_{}", user_id);

        let (image, protocol) = if let Ok(img) = self.load_processed_avatar_from_disk(&key).await {
            let protocol = self.picker.new_resize_protocol(img.clone());
            (img, protocol)
        } else {
            self.process_and_cache_avatar(&url, &key).await?
        };

        self.avatar_cache.insert(user_id.to_string(), CachedAvatar { image, protocol });

        Ok(())
    }

    async fn load_processed_avatar_from_disk(&mut self, key: &str) -> Result<DynamicImage> {
        let dir = Self::get_cache_dir()?;
        let path = dir.join(format!("{}.png", key));
        if !path.exists() {
            return Err(anyhow::anyhow!("no processed avatar"));
        }
        let bytes = fs::read(&path).await?;
        let img = image::load_from_memory(&bytes)?;
        Ok(img)
    }

    pub async fn load_attachment(
        &mut self,
        attachment_id: &str,
        url: &str,
        min_size: (u32, u32),
        max_size: (u32, u32),
    ) -> Result<()> {
        if self.attachment_cache.contains_key(attachment_id) {
            return Ok(());
        }

        let (image, terminal_width, terminal_height) = if let Ok(result) = self.load_attachment_from_disk_cache(attachment_id, min_size, max_size).await {
            result
        } else {
            let img = self.download_image(url).await?;
            let (resized_img, terminal_width, terminal_height) = self.resize_attachment_image(img, min_size, max_size);
            let _ = self.save_attachment_to_disk_cache(attachment_id, &resized_img).await;
            (resized_img, terminal_width, terminal_height)
        };
        
        let protocol = self.picker.new_resize_protocol(image.clone());
        
        self.attachment_cache.insert(
            attachment_id.to_string(),
            CachedAttachment { image, protocol, terminal_width, terminal_height }
        );

        Ok(())
    }

    fn resize_attachment_image(
        &self,
        img: DynamicImage,
        min_size: (u32, u32),
        max_size: (u32, u32),
    ) -> (DynamicImage, u16, u16) {
        let (width, height) = img.dimensions();
        
        let (font_w, font_h) = self.picker.font_size();
        let min_w_px = min_size.0 * font_w as u32;
        let min_h_px = min_size.1 * font_h as u32;
        let max_w_px = max_size.0 * font_w as u32;
        let max_h_px = max_size.1 * font_h as u32;

        let (target_w, target_h) = if width < min_w_px || height < min_h_px {
            let scale_w = min_w_px as f32 / width as f32;
            let scale_h = min_h_px as f32 / height as f32;
            let scale = scale_w.max(scale_h);
            ((width as f32 * scale) as u32, (height as f32 * scale) as u32)
        } else if width > max_w_px || height > max_h_px {
            let scale_w = max_w_px as f32 / width as f32;
            let scale_h = max_h_px as f32 / height as f32;
            let scale = scale_w.min(scale_h);
            ((width as f32 * scale) as u32, (height as f32 * scale) as u32)
        } else {
            (width, height)
        };

        let resized = if target_w != width || target_h != height {
            img.resize_exact(target_w, target_h, FilterType::Lanczos3)
        } else {
            img
        };

        let terminal_width = ((target_w as f32 / font_w as f32).ceil() as u16).max(1);
        let terminal_height = ((target_h as f32 / font_h as f32).ceil() as u16).max(1);

        (resized, terminal_width, terminal_height)
    }

    async fn download_image(&self, url: &str) -> Result<DynamicImage> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        Ok(image::load_from_memory(&bytes)?)
    }

    fn circle_mask(img: DynamicImage) -> DynamicImage {
        let mut out = image::RgbaImage::new(img.width(), img.height());
        let (w, h) = (img.width(), img.height());
        let cx = w as f32 / 2.0;
        let cy = h as f32 / 2.0;
        let r2 = (w.min(h) as f32 / 2.0).powi(2);

        for y in 0..h {
            for x in 0..w {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let mut p = img.get_pixel(x, y);
                if dx*dx + dy*dy > r2 {
                    p.0[3] = 0;
                }
                out.put_pixel(x, y, p);
            }
        }

        DynamicImage::ImageRgba8(out)
    }

    fn get_cache_dir() -> Result<PathBuf> {
        let dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?
            .join("remycord")
            .join("images");
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    pub async fn get_cache_stats() -> Result<CacheStats> {
        let cache_dir = Self::get_cache_dir()?;
        let mut total_size = 0u64;
        let mut file_count = 0usize;
        let mut avatar_count = 0usize;
        let mut attachment_count = 0usize;

        if let Ok(mut entries) = fs::read_dir(&cache_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_file() {
                        total_size += metadata.len();
                        file_count += 1;
                        if let Some(name) = entry.file_name().to_str() {
                            if name.starts_with("avatar_") {
                                avatar_count += 1;
                            } else {
                                attachment_count += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(CacheStats {
            total_size_bytes: total_size,
            total_files: file_count,
            avatar_files: avatar_count,
            attachment_files: attachment_count,
            cache_path: cache_dir,
        })
    }

    pub async fn clear_cache() -> Result<()> {
        let dir = Self::get_cache_dir()?;
        if dir.exists() {
            fs::remove_dir_all(&dir).await?;
            fs::create_dir_all(&dir).await?;
        }
        Ok(())
    }

    pub async fn clear_avatar_cache() -> Result<()> {
        let dir = Self::get_cache_dir()?;
        if let Ok(mut entries) = fs::read_dir(&dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("avatar_") {
                        let _ = fs::remove_file(entry.path()).await;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn clear_attachment_cache() -> Result<()> {
        let dir = Self::get_cache_dir()?;
        if let Ok(mut entries) = fs::read_dir(&dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(name) = entry.file_name().to_str() {
                    if !name.starts_with("avatar_") {
                        let _ = fs::remove_file(entry.path()).await;
                    }
                }
            }
        }
        Ok(())
    }

    async fn load_attachment_from_disk_cache(
        &mut self,
        key: &str,
        min_size: (u32, u32),
        max_size: (u32, u32),
    ) -> Result<(DynamicImage, u16, u16)> {
        let dir = Self::get_cache_dir()?;
        let img_path = dir.join(format!("{}.png", key));
        if img_path.exists() {
            let bytes = fs::read(&img_path).await?;
            let img = image::load_from_memory(&bytes)?;
            let (resized_img, terminal_width, terminal_height) = self.resize_attachment_image(img, min_size, max_size);
            return Ok((resized_img, terminal_width, terminal_height));
        }
        Err(anyhow::anyhow!("Attachment cache not found"))
    }

    async fn save_attachment_to_disk_cache(&self, key: &str, img: &DynamicImage) -> Result<()> {
        let dir = Self::get_cache_dir()?;
        let img_path = dir.join(format!("{}.png", key));
        if !img_path.exists() {
            img.save(&img_path)?;
        }
        Ok(())
    }

    pub fn get_avatar(&mut self, user_id: &str) -> Option<&mut StatefulProtocol> {
        self.avatar_cache.get_mut(user_id).map(|c| &mut c.protocol)
    }

    pub fn get_attachment(&mut self, attachment_id: &str) -> Option<StatefulProtocol> {
        self.attachment_cache.get(attachment_id).map(|cached| {
            self.picker.new_resize_protocol(cached.image.clone())
        })
    }
    
    pub fn get_attachment_height(&self, attachment_id: &str) -> Option<u16> {
        self.attachment_cache.get(attachment_id).map(|c| c.terminal_height)
    }

    pub fn get_attachment_width(&self, attachment_id: &str) -> Option<u16> {
        self.attachment_cache.get(attachment_id).map(|c| c.terminal_width)
    }

    pub fn clear_memory_cache(&mut self) {
        self.avatar_cache.clear();
        self.attachment_cache.clear();
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_size_bytes: u64,
    pub total_files: usize,
    pub avatar_files: usize,
    pub attachment_files: usize,
    pub cache_path: PathBuf,
}

impl CacheStats {
    pub fn total_size_mb(&self) -> f64 {
        self.total_size_bytes as f64 / (1024.0 * 1024.0)
    }
}

impl Default for ImageRenderer {
    fn default() -> Self {
        Self::new()
    }
}
