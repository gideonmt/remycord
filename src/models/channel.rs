#[derive(Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: ChannelType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelType {
    Text,
    Voice,
}

impl Channel {
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: ChannelType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
        }
    }

    pub fn prefix(&self) -> &str {
        match self.kind {
            ChannelType::Text => "# ",
            ChannelType::Voice => "🔊 ",
        }
    }
}
