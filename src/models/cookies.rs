use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookies {
    pub access_token: String,
    pub user_id: String,
}