use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};

use crate::utils::database;
use crate::models::cookies::Cookies;

const COOKIE_NAME_ACCESS_TOKEN: &str = "access_token";
const COOKIE_NAME_USER_ID: &str = "user_id";

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponse {
    result: String,
}

pub async fn is_me_auth(req: HttpRequest) -> AuthResult<Cookies> {
    let db = database::connect()
        .await
        .map_err(|e| AuthError::DatabaseConnectionError(e.to_string()))?;

    let access_token_cookie = req.cookie(COOKIE_NAME_ACCESS_TOKEN).ok_or(AuthError::TokenNotFound)?;
    let user_id_cookie = req.cookie(COOKIE_NAME_USER_ID).ok_or(AuthError::UserNotFound)?;

    let query = format!("fn::verify_token('{}')", access_token_cookie.value());

    let mut res = db
        .query(query)
        .await
        .map_err(|e| AuthError::DatabaseQueryError(e.to_string()))?;

    let query_response: Vec<QueryResponse> = res
        .take(0)
        .map_err(|e| AuthError::DatabaseResponseError(e.to_string()))?;

    match query_response[0].result.as_str() {
        "NotFound" => return Err(AuthError::TokenNotFound),
        "Expired" => return Err(AuthError::TokenExpired),
        "Revoked" => return Err(AuthError::TokenRevoked),
        "Valid" => return Ok( Cookies {
            access_token: access_token_cookie.value().to_string(),
            user_id: user_id_cookie.value().to_string(),
        }
        ),
        _ => return Err(AuthError::TokenInvalid),
    }
}

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AuthError {
    TokenNotFound,
    TokenExpired,
    TokenInvalid,
    TokenRevoked,
    DatabaseConnectionError(String),
    DatabaseQueryError(String),
    DatabaseResponseError(String),
    UserNotFound,
}