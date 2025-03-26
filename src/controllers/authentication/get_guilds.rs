use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt;
use surrealdb::RecordId;

use crate::models::cookies::Cookies;
use crate::utils::database;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GuildInfo {
    pub id: String,
}

#[derive(Debug)]
pub enum GetGuildError {
    RequestGuildsError(String),
    DeserializeError(String),
    RequestFailed(u16),
    DatabaseQueryError(String),
    QueryParseError(String),
    DatabaseConnectionError(String),
}

impl fmt::Display for GetGuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetGuildError::RequestGuildsError(e) => {
                write!(f, "Failed to request guilds from Discord API: {}", e)
            }
            GetGuildError::DeserializeError(e) => {
                write!(f, "Failed to deserialize guild data: {}", e)
            }
            GetGuildError::RequestFailed(status) => write!(
                f,
                "Discord API returned an error with status code: {}",
                status
            ),
            GetGuildError::DatabaseQueryError(e) => write!(f, "Database query error: {}", e),
            GetGuildError::DatabaseConnectionError(e) => {
                write!(f, "Database connection error: {}", e)
            }
            GetGuildError::QueryParseError(e) => write!(f, "Query parse error: {}", e),
        }
    }
}

pub async fn get_guilds(
    cookies: Cookies,
    user_id: String,
) -> Result<Vec<String>, GetGuildError> {
    let client = Client::new();
    let response = client
        .get("https://discord.com/api/users/@me/guilds")
        .header("Authorization", format!("Bearer {}", cookies.access_token))
        .send()
        .await
        .map_err(|e| GetGuildError::RequestGuildsError(e.to_string()))?;

    if response.status() != StatusCode::OK {
        return Err(GetGuildError::RequestFailed(response.status().as_u16()));
    }

    let guild_infos: Vec<GuildInfo> = response
        .json()
        .await
        .map_err(|e| GetGuildError::DeserializeError(e.to_string()))?;

    let guild_ids: Vec<String> = guild_infos
        .clone()
        .into_iter()
        .map(|guild| guild.id)
        .collect();

    let guilds = get_guilds_from_db(guild_ids).await?;
    for guild_id in guilds.iter() {
        let guild_record_id: RecordId = ("guilds".to_string(), guild_id.clone()).into();
        let user_record_id: RecordId = ("user".to_string(), user_id.clone()).into();
        add_user_joined_guild_to_db(user_record_id, guild_record_id).await?;
    }

    Ok(guilds)
}

async fn get_guilds_from_db(guild_ids: Vec<String>) -> Result<Vec<String>, GetGuildError> {
    let db = database::connect()
        .await
        .map_err(|e| GetGuildError::DatabaseConnectionError(e.to_string()))?;

    let mut guilds: Vec<String> = Vec::new();

    #[derive(Deserialize, Debug)]
    struct QueryResponse {
        result: String,
    }

    for guild_id in guild_ids {
        let sql = format!("fn::get_guild({})", guild_id);

        let mut response = db
            .query(sql.as_str())
            .await
            .map_err(|e| GetGuildError::DatabaseQueryError(e.to_string()))?;

        let query_response: Vec<QueryResponse> = response
            .take(0)
            .map_err(|e| GetGuildError::QueryParseError(e.to_string()))?;

        match query_response[0].result.as_str() {
            "Found" => {
                guilds.push(guild_id.clone());
            }
            _ => {
            }
        }
    }

    Ok(guilds)
}

#[derive(Debug, Serialize, Deserialize)]
struct Joined {
    #[serde(rename = "in")]
    user_id: RecordId,
    #[serde(rename = "out")]
    guild_id: RecordId,
}

async fn add_user_joined_guild_to_db(
    user_id: RecordId,
    guild_id: RecordId,
) -> Result<Vec<Joined>, GetGuildError> {
    let db = database::connect()
        .await
        .map_err(|e| GetGuildError::DatabaseConnectionError(e.to_string()))?;

    let sql = format!("RELATE {}->joined->{} ", user_id, guild_id);
    db.query(sql.as_str())
        .await
        .map_err(|e| GetGuildError::DatabaseQueryError(e.to_string()))?;

    Ok(vec![])
}
