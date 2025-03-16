use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub password: String,
}