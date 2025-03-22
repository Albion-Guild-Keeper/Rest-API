use actix_web::{put, HttpResponse};
use actix_web::web::{Json, Path};
use crate::utils::surreal_int::SurrealInt;
use serde::Deserialize;
use crate::controllers::discord as controllers;


#[derive(Deserialize)]
struct DiscordInput {
    discord_name: String,
    joined_at: String,
}

#[put("/discord/{discord_id}")]
pub async fn join_discord(discord_id: Path<SurrealInt>, discord_data: Json<DiscordInput>) -> HttpResponse {
    let discord_name = discord_data.discord_name.clone();
    let joined_at = discord_data.joined_at.clone();
    
    let result = controllers::join_discord(discord_id.into_inner(), discord_name, joined_at).await;

    println!("result: {:?}", result);   // @todo qui per qualche motivo da errore
    // app] result: Err(Db(Serialization("failed to deserialize; expected a 64-bit signed integer, found $surrealdb::private::sql::Thing { tb: \"discords\", id: Id::Number(1348909995638390834i64) }")))

    match result {
        Ok(Some(_)) => HttpResponse::Ok().body("success record already exists"),
        Ok(None) => HttpResponse::Ok().body("success created a new record"),    
        Err(_) => HttpResponse::InternalServerError().body("aaaa"),
    }
}
