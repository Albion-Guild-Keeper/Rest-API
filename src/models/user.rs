use serde::{Deserialize, Serialize};

use crate::utils::surreal_int::SurrealInt;

// Definisce una struttura chiamata Discord che contiene informazioni su un utente Discord.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: SurrealInt, 
    pub server_name: String, 
    pub username: String,
    pub joined_at: String
}