#[derive(Debug, Clone)]
pub struct DmChannel {
    pub id: String,
    pub recipient: DmUser,
}

#[derive(Debug, Clone)]
pub struct DmUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
}

impl DmChannel {
    pub fn new(id: impl Into<String>, recipient: DmUser) -> Self {
        Self {
            id: id.into(),
            recipient,
        }
    }

    pub fn display_name(&self) -> String {
        if self.recipient.discriminator == "0" {
            self.recipient.username.clone()
        } else {
            format!("{}#{}", self.recipient.username, self.recipient.discriminator)
        }
    }
}

impl DmUser {
    pub fn new(id: impl Into<String>, username: impl Into<String>, discriminator: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            discriminator: discriminator.into(),
        }
    }
}
