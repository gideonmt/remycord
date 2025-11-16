use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayResponse {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Option<serde_json::Value>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadyEvent {
    pub user: DiscordUser,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnavailableGuild {
    pub id: String,
}

pub const DISPATCH: u8 = 0;
pub const HEARTBEAT: u8 = 1;
pub const IDENTIFY: u8 = 2;
pub const HELLO: u8 = 10;
pub const HEARTBEAT_ACK: u8 = 11;
