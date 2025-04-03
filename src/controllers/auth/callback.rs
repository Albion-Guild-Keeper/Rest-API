use reqwest::Client;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Record;
use thiserror::Error;

use crate::utils::database::connect::connect;

const CLIENT_ID: &str = "1248308695323115543";
const CLIENT_SECRET: &str = "LSXdbaUCwnHDuoeuIEUJc5G0HJZebXxg";
const REDIRECT_URI: &'static str = "http://localhost:8000/api/v1/auth/callback";

pub async fn callback(code: String) -> Result<String, CallBackError> {
    let token = ask_discord_for_token(code).await?;
    let user_info = ask_discord_for_user_info(&token).await?;
    let _ = create_user(user_info.clone()).await?;
    let _ = signin(user_info.clone()).await?;
    let _ = create_logs_into_db(user_info.clone()).await?;

    Ok("Logged in Successfully with Discord".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogsRecord {
    thing: String,
    created_at: String,
}
async fn create_logs_into_db(user_info: UserInfoResponse) -> Result<(), CallBackError> {
    let db = connect().await.map_err(|e| {
        CallBackError::DatabaseConnectionError(format!("CreateLogsIntoDB: {}", e.to_string()))
    })?;

    let log = LogsRecord {
        thing: "SignedIn".to_string(),
        created_at: "time::now()".to_string(),
    };

    let user_id = user_info.id;

    let query = format!(
        "CREATE logs SET thing = '{}', created_at = time::now(), user_id = type::thing('users', '{}')",
        log.thing, user_id
    );

    let _ = db.query(query).await.map_err(|e| {
        CallBackError::DatabaseQueryError(format!("CreateLogsIntoDB: {}", e.to_string()))
    })?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct BodyRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
    scope: String,
    token_type: String,
}

async fn ask_discord_for_token(code: String) -> Result<TokenResponse, CallBackError> {
    let body = BodyRequest {
        grant_type: "authorization_code".to_string(),
        client_id: CLIENT_ID.to_string(),
        client_secret: CLIENT_SECRET.to_string(),
        code,
        redirect_uri: REDIRECT_URI.to_string(),
    };

    let client = Client::new();
    let response = client
        .post("https://discord.com/api/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .map_err(|e| {
            CallBackError::TokenRequestFailed(format!("AskDiscordToken: {}", e.to_string()))
        })?;

    if response.status().is_success() {
        let text = response.text().await.map_err(|e| {
            CallBackError::TokenResponseParseError(format!("AskDiscordToken: {}", e.to_string()))
        })?;
        let token_response = serde_json::from_str::<TokenResponse>(&text).map_err(|e| {
            CallBackError::TokenResponseParseError(format!("AskDiscordToken: {}", e.to_string()))
        })?;
        Ok(token_response)
    } else {
        Err(CallBackError::TokenRequestFailed(format!(
            "AskDiscordToken: {}",
            response.status().to_string()
        )))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserInfoResponse {
    id: String,
    username: String,
    avatar: String,
    global_name: String,
    locale: String,
}

async fn ask_discord_for_user_info(
    token: &TokenResponse,
) -> Result<UserInfoResponse, CallBackError> {
    let client = Client::new();
    let response = client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .map_err(|e| {
            CallBackError::UserInfoRequestFailed(format!("AskDiscordUserInfo: {}", e.to_string()))
        })?;

    if response.status().is_success() {
        let text = response.text().await.map_err(|e| {
            CallBackError::TokenResponseParseError(format!("AskDiscordUserInfo: {}", e.to_string()))
        })?;
        let user_info = serde_json::from_str::<UserInfoResponse>(&text).map_err(|e| {
            CallBackError::TokenResponseParseError(format!("AskDiscordUserInfo: {}", e.to_string()))
        })?;
        Ok(user_info)
    } else {
        Err(CallBackError::UserInfoRequestFailed(format!(
            "AskDiscordUserInfo: {}",
            response.status().to_string()
        )))
    }
}

async fn create_user(user_info: UserInfoResponse) -> Result<(), CallBackError> {
    let db = connect().await.map_err(|e| {
        CallBackError::DatabaseConnectionError(format!("CreateUser: {}", e.to_string()))
    })?;

    let query = format!(
        "CREATE users SET id = '{}', username = '{}', avatar = '{}', global_name = '{}', local = '{}', panel = {}",
        user_info.id,
        user_info.username,
        user_info.avatar,
        user_info.global_name,
        user_info.locale,
        true,
    );

    let _execute = db
        .query(query)
        .await
        .map_err(|e| CallBackError::DatabaseQueryError(format!("CreateUser: {}", e.to_string())))?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct SignInRecord {
    user_id: String,
}
async fn signin(user_info: UserInfoResponse) -> Result<(), CallBackError> {
    let user_id = user_info.id.clone();

    let db = connect().await.map_err(|e| {
        CallBackError::DatabaseConnectionError(format!("SignIn: {}", e.to_string()))
    })?;

    let _ = db
        .signin(Record {
            namespace: "root",
            database: "root",
            access: "user",
            params: SignInRecord {
                user_id: user_id.clone(),
            },
        })
        .await
        .map_err(|e| CallBackError::DatabaseQueryError(format!("SignIn: {}", e.to_string())))?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum CallBackError {
    #[error("Token request failed: {0}")]
    TokenRequestFailed(String),
    #[error("Token response parse error: {0}")]
    TokenResponseParseError(String),
    #[error("User info request failed: {0}")]
    UserInfoRequestFailed(String),
    #[error("Database connection error: {0}")]
    DatabaseConnectionError(String),
    #[error("Database query error: {0}")]
    DatabaseQueryError(String),
}
