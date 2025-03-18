use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscordAuth {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
}  