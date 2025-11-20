#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: ChannelType,
    pub position: i32,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChannelCategory {
    pub id: String,
    pub name: String,
    pub position: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelType {
    Text,
    Voice,
    Announcement,
    Stage,
    Forum,
    Category,
}

impl Channel {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: ChannelType,
        position: i32,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            position,
            parent_id: None,
        }
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn prefix(&self) -> &str {
        match self.kind {
            ChannelType::Text => "# ",
            ChannelType::Voice => "🔊 ",
            ChannelType::Announcement => "📢 ",
            ChannelType::Stage => "🎙️ ",
            ChannelType::Forum => "💬 ",
            ChannelType::Category => "",
        }
    }

    pub fn is_text_based(&self) -> bool {
        matches!(
            self.kind,
            ChannelType::Text | ChannelType::Announcement | ChannelType::Forum
        )
    }
}

impl ChannelCategory {
    pub fn new(id: impl Into<String>, name: impl Into<String>, position: i32) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            position,
        }
    }
}

/// Organized channel structure with categories
#[derive(Debug, Clone)]
pub struct ChannelList {
    pub categories: Vec<ChannelCategory>,
    pub channels: Vec<Channel>,
}

impl ChannelList {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            channels: Vec::new(),
        }
    }

    /// Sort channels by position and organize by category
    pub fn sort(&mut self) {
        self.categories.sort_by_key(|c| c.position);
        self.channels.sort_by_key(|c| c.position);
    }

    /// Get channels without a category (top-level)
    pub fn uncategorized_channels(&self) -> Vec<&Channel> {
        self.channels
            .iter()
            .filter(|c| c.parent_id.is_none())
            .collect()
    }

    /// Get channels in a specific category
    pub fn channels_in_category(&self, category_id: &str) -> Vec<&Channel> {
        self.channels
            .iter()
            .filter(|c| c.parent_id.as_deref() == Some(category_id))
            .collect()
    }

    /// Get all text-based channels (for message viewing)
    pub fn text_channels(&self) -> Vec<&Channel> {
        self.channels
            .iter()
            .filter(|c| c.is_text_based())
            .collect()
    }
}

impl Default for ChannelList {
    fn default() -> Self {
        Self::new()
    }
}
