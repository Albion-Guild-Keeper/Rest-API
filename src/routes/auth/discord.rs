use crate::controllers::auth::discord as controller;
use actix_web::{get, HttpResponse};

#[get("auth/discord")]
pub async fn discord_auth() -> HttpResponse {
    let access_token = "12345".to_string();
    let expires_in = 604800;
    let scope = "identify".to_string();
    // @todo da rimuovere questi let fissi

    match controller::discord_auth(access_token, expires_in, scope).await {
        Ok(_) => {
            HttpResponse::SeeOther().append_header(("Location", "http://localhost:8080/")).finish()
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}