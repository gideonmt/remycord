use anyhow::Result;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageProtocol {
    Kitty,
    None,
}

pub struct ImageRenderer {
    protocol: ImageProtocol,
}

impl ImageRenderer {
    pub fn new() -> Self {
        Self {
            protocol: Self::detect_protocol(),
        }
    }

    fn detect_protocol() -> ImageProtocol {
        if std::env::var("TERM").unwrap_or_default().contains("kitty") {
            return ImageProtocol::Kitty;
        }
        
        if std::env::var("KITTY_WINDOW_ID").is_ok() {
            return ImageProtocol::Kitty;
        }

        ImageProtocol::None
    }

    pub fn is_supported(&self) -> bool {
        self.protocol != ImageProtocol::None
    }

    pub fn protocol(&self) -> ImageProtocol {
        self.protocol
    }

    /// Render image at specific terminal position using Kitty graphics protocol
    pub fn render_image(&self, image_data: &[u8], x: u16, y: u16, width: u16, height: u16) -> Result<()> {
        match self.protocol {
            ImageProtocol::Kitty => self.render_kitty(image_data, x, y, width, height),
            ImageProtocol::None => Ok(()),
        }
    }

    fn render_kitty(&self, image_data: &[u8], x: u16, y: u16, width: u16, height: u16) -> Result<()> {
        let encoded = BASE64.encode(image_data);
        
        // Kitty graphics protocol escape sequence
        // Format: \x1b_G<control_data>;<payload>\x1b\\
        // a=T: transmit and display
        // f=100: PNG format (can also be 24 for RGB, 32 for RGBA)
        // t=d: direct transmission (data inline)
        // c=<cols>: width in columns
        // r=<rows>: height in rows
        // X=<x>: x position
        // Y=<y>: y position
        
        let control = format!("a=T,f=100,t=d,c={},r={},X={},Y={}", width, height, x, y);
        let escape_seq = format!("\x1b_G{};{}\x1b\\", control, encoded);
        
        std::io::stdout().write_all(escape_seq.as_bytes())?;
        std::io::stdout().flush()?;
        
        Ok(())
    }

    /// Download and render image from URL
    pub async fn render_url(&self, url: &str, x: u16, y: u16, width: u16, height: u16) -> Result<()> {
        if !self.is_supported() {
            return Ok(());
        }

        let response = reqwest::get(url).await?;
        let image_data = response.bytes().await?;
        
        self.render_image(&image_data, x, y, width, height)
    }

    /// Clear image at position
    pub fn clear_image(&self, image_id: u32) -> Result<()> {
        match self.protocol {
            ImageProtocol::Kitty => {
                let escape_seq = format!("\x1b_Ga=d,d=I,i={}\x1b\\", image_id);
                std::io::stdout().write_all(escape_seq.as_bytes())?;
                std::io::stdout().flush()?;
                Ok(())
            }
            ImageProtocol::None => Ok(()),
        }
    }

    /// Clear all images
    pub fn clear_all_images(&self) -> Result<()> {
        match self.protocol {
            ImageProtocol::Kitty => {
                let escape_seq = "\x1b_Ga=d,d=A\x1b\\";
                std::io::stdout().write_all(escape_seq.as_bytes())?;
                std::io::stdout().flush()?;
                Ok(())
            }
            ImageProtocol::None => Ok(()),
        }
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
    fn test_protocol_detection() {
        let renderer = ImageRenderer::new();
        let _ = renderer.protocol();
    }
}
