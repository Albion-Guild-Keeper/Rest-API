use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignIn {
    pub email: String,
    pub password: String,
}  