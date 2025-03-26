use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::{routes::guild_create::Guild, utils::database};

pub async fn guild_create(guild_data: Json<Guild>) -> Result<String, GuildCreateError> {
    let db = database::connect()
        .await
        .map_err(|e| GuildCreateError::DatabaseConnectionError(e.to_string()))?;

    let content = guild_data.into_inner();

    let result: Option<Guild> = db
        .create("guilds")
        .content(content)
        .await
        .map_err(|e| GuildCreateError::DatabaseQueryError(e.to_string()))?;

    // @todo Err DatabaseQueryError(\"Serialization error: failed to deserialize; expected a 64-bit signed integer, found $surrealdb::private::sql::Thing { tb: \\\"guilds\\\", id: Id::Number(1348909995638390834i64) }\")"

    match result {
        Some(_) => Ok("Guild created".to_string()),
        None => Err(GuildCreateError::GuildCreateError),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GuildCreateError {
    DatabaseConnectionError(String),
    DatabaseQueryError(String),
    GuildCreateError,
}