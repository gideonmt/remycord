#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: String,
    pub content: String,
    pub timestamp: String,
}

impl Message {
    pub fn new(
        id: impl Into<String>,
        channel_id: impl Into<String>,
        author: impl Into<String>,
        content: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            channel_id: channel_id.into(),
            author: author.into(),
            content: content.into(),
            timestamp: timestamp.into(),
        }
    }
}
