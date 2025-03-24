use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};

use crate::utils::database;

const COOKIE_NAME: &str = "access_token";

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponse {
    result: String,
}

pub async fn is_me_auth(req: HttpRequest) -> AuthResult<String> {
    let db = database::connect()
        .await
        .map_err(|e| AuthError::DatabaseConnectionError(e.to_string()))?;

    let cookie = req.cookie(COOKIE_NAME).ok_or(AuthError::TokenNotFound)?;

    dbg!(&cookie);

    let query = format!("fn::verify_token('{}')", cookie.value());

    dbg!(&query);

    let mut res = db
        .query(query)
        .await
        .map_err(|e| AuthError::DatabaseQueryError(e.to_string()))?;

    dbg!(&res);

    let query_response: Vec<QueryResponse> = res
        .take(0)
        .map_err(|e| AuthError::DatabaseResponseError(e.to_string()))?;

    dbg!(&query_response);

    match query_response[0].result.as_str() {
        "NotFound" => return Err(AuthError::TokenNotFound),
        "Expired" => return Err(AuthError::TokenExpired),
        "Revoked" => return Err(AuthError::TokenRevoked),
        "Valid" => return Ok("User is authenticated".to_string()),
        _ => return Err(AuthError::TokenInvalid),
    }
}

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AuthError {
    TokenNotFound,
    TokenExpired,
    TokenInvalid,
    TokenRevoked,
    DatabaseConnectionError(String),
    DatabaseQueryError(String),
    DatabaseResponseError(String),
}
