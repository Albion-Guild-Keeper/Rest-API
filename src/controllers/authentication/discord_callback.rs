use actix_web::HttpRequest;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::utils::database as database;
use crate::models::cookies::Cookies;

const CLIENT_ID: &'static str = "1248308695323115543";
const CLIENT_SECRET: &'static str = "t2DWinSWnDhbAAw6kpJbvDjVyRfCDoN4";
const REDIRECT_URI: &'static str = "http://localhost:8000/api/v1/auth/discord/callback";

pub async fn discord_callback(req: HttpRequest) -> Result<Cookies, CallBackError> {
    let query_string = req.query_string().to_string();

    let code = get_code_from_query_string(query_string).await?;
    let token = ask_discord_for_token(code).await?;
    let user_info = ask_discord_for_user_info(&token).await?;
    let _ = create_user(user_info.clone()).await?;
    let _ = save_cookie_to_database(&token).await?;

    let cookies = Cookies {
        access_token: token.access_token.clone(),
        user_id: user_info.id.clone(),
    };

    Ok(cookies)
}

async fn get_code_from_query_string(query_string: String) -> Result<String, CallBackError> {
    let code = query_string
        .split('=')
        .nth(1)
        .map(|s| s.to_string())
        .ok_or(CallBackError::MissingCode("Missing code".to_string()))?;

    Ok(code)
}

#[derive(Debug, Serialize, Deserialize)]
struct BodyRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
        .map_err(|e| CallBackError::TokenRequestFailed(e.to_string()))?;

    if response.status().is_success() {
        let text = response.text().await.map_err(|e| CallBackError::TokenResponseParseError(e.to_string()))?;
        let token_response = serde_json::from_str::<TokenResponse>(&text).map_err(|e| CallBackError::TokenResponseParseError(e.to_string()))?;
        Ok(token_response)
    } else {
        Err(CallBackError::TokenRequestFailed(
            response.status().to_string(),
        ))
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

async fn ask_discord_for_user_info(token: &TokenResponse) -> Result<UserInfoResponse, CallBackError> {
    let client = Client::new();
    let response = client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .map_err(|e| CallBackError::UserInfoRequestFailed(e.to_string()))?;

    if response.status().is_success() {
        let text = response.text().await.map_err(|e| CallBackError::TokenResponseParseError(e.to_string()))?;
        let user_info = serde_json::from_str::<UserInfoResponse>(&text).map_err(|e| CallBackError::TokenResponseParseError(e.to_string()))?;
        Ok(user_info)
    } else {
        Err(CallBackError::UserInfoRequestFailed(
            response.status().to_string(),
        ))
    }
}

async fn create_user(user_info: UserInfoResponse) -> Result<(), CallBackError> {
    let db = database::connect()
        .await
        .map_err(|e| CallBackError::DatabaseConnectionError(e.to_string()))?;

    let query = format!(
        "CREATE users SET id = '{}', username = '{}', avatar = '{}', global_name = '{}', local = '{}', panel = {}",
        user_info.id,
        user_info.username,
        user_info.avatar,
        user_info.global_name,
        user_info.locale,
        true,
    );

    let _execute = db.query(query)
        .await
        .map_err(|e| CallBackError::DatabaseQueryError(e.to_string()))?;

    Ok(())
}

async fn save_cookie_to_database(token: &TokenResponse) -> Result<(), CallBackError> {
    let db = database::connect()
        .await
        .map_err(|e| CallBackError::DatabaseConnectionError(e.to_string()))?;

    let query = format!(
        "CREATE tokens SET access_token = '{}', expires_in = {}, refresh_token = '{}'",
        token.access_token, token.expires_in, token.refresh_token
    );
    dbg!(&query);

    let _res = db.query(query.to_string())
        .await
        .map_err(|err| CallBackError::DatabaseQueryError(err.to_string()))?;

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CallBackError {
    MissingCode(String),
    TokenRequestFailed(String),
    TokenResponseParseError(String),
    DatabaseConnectionError(String),
    DatabaseQueryError(String),
    UserInfoRequestFailed(String),
}