use actix_web::{put, HttpResponse};
use actix_web::web::{Json, Path};
use serde::Deserialize;
use crate::controllers::user as controller;
use crate::utils::surreal_int::SurrealInt;

#[derive(Deserialize)]
struct UserInput {
    username: String,
    joined_at: String,
    discord_id: SurrealInt,
}

#[put("/user/{user_id}")]
pub async fn join_user(user_id: Path<SurrealInt>, user_data: Json<UserInput>) -> HttpResponse {
    let username = user_data.username.clone();
    let joined_at = user_data.joined_at.clone();
    let discord_id = user_data.discord_id.clone();
    
    match controller::join_user(user_id.into_inner(), username, joined_at, discord_id).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}