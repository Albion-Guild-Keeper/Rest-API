use actix_web::{get, HttpRequest, HttpResponse};

use crate::controllers::auth::discord_callback as controller;

#[get("/auth/discord/callback")]
pub async fn discord_callback(req: HttpRequest) -> HttpResponse {
    match controller::discord_callback(req).await {
        Ok(cookie) => HttpResponse::Ok().cookie(cookie).finish(),
        Err(error) => HttpResponse::InternalServerError().body(format!("Error: {:#?}", error)),
    }
}