#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: String,
    pub author_id: String,
    pub author_avatar: Option<String>,
    pub content: String,
    pub timestamp: String,
    pub attachments: Vec<MessageAttachment>,
}

#[derive(Debug, Clone)]
pub struct MessageAttachment {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub proxy_url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub content_type: Option<String>,
}

impl Message {
    pub fn new(
        id: impl Into<String>,
        channel_id: impl Into<String>,
        author: impl Into<String>,
        author_id: impl Into<String>,
        author_avatar: Option<String>,
        content: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            channel_id: channel_id.into(),
            author: author.into(),
            author_id: author_id.into(),
            author_avatar,
            content: content.into(),
            timestamp: timestamp.into(),
            attachments: Vec::new(),
        }
    }

    pub fn with_attachments(mut self, attachments: Vec<MessageAttachment>) -> Self {
        self.attachments = attachments;
        self
    }
}

impl MessageAttachment {
    pub fn new(
        id: impl Into<String>,
        filename: impl Into<String>,
        url: impl Into<String>,
        proxy_url: impl Into<String>,
        width: Option<u32>,
        height: Option<u32>,
        content_type: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            filename: filename.into(),
            url: url.into(),
            proxy_url: proxy_url.into(),
            width,
            height,
            content_type,
        }
    }

    pub fn is_image(&self) -> bool {
        if let Some(content_type) = &self.content_type {
            content_type.starts_with("image/")
        } else {
            let ext = self.filename.rsplit('.').next().unwrap_or("");
            matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "gif" | "webp")
        }
    }
}
