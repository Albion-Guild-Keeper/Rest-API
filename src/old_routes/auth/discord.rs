use actix_web::http::header::LOCATION;
use actix_web::{get, HttpRequest, HttpResponse};

use crate::controllers::auth::discord as controller;
use crate::controllers::auth::discord::CallBackError;

#[get("/auth/discord/callback")]
pub async fn discord_callback(req: HttpRequest) -> HttpResponse {
    match controller::discord_callback(req).await {
        Ok(cookie) => {
            HttpResponse::Ok().cookie(cookie).append_header((LOCATION, "http://localhost:8080/")).finish()
        },
        Err(error) => match error { 
            CallBackError::MissingCode => HttpResponse::BadRequest().body("Missing code"),
            CallBackError::TokenRequestFailed => {
                HttpResponse::InternalServerError().body(format!("Token request failed {:#?}", error))
            }
            CallBackError::HttpResponseError => {
                HttpResponse::InternalServerError().body(format!("HTTP response error {:#?}", error))
            }
            CallBackError::TokenResponseParseError => {
                HttpResponse::InternalServerError().body(format!("Token response parse error {:#?}", error))
            }
            CallBackError::DatabaseConnectionError => {
                HttpResponse::InternalServerError().body(format!("Database connection error: {:#?}", error))
            }
            CallBackError::DatabaseQueryError(_) => {
                HttpResponse::InternalServerError().body(format!("Database query error: {:#?}", error))
            }
            CallBackError::ParsingError => {
                HttpResponse::InternalServerError().body(format!("Parsing error {:#?}", error))
            }
            CallBackError::InvalidTokenError => HttpResponse::Unauthorized().body(format!("Invalid token {:#?}", error))
        },
    }
}
