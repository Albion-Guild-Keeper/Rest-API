use actix_web::{put, web::Json, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{controllers::guild_create::guild_create as controller, utils::surreal_int::SurrealInt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    id: SurrealInt,
    name: String,
    owner_id: SurrealInt,
}

#[put("/guild_create")]
pub async fn guild_create(guild_data: Json<Guild>) -> HttpResponse {
    match controller(guild_data).await {
        Ok(response) => {
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            println!("Error occurred: {:?}", err);
            HttpResponse::InternalServerError().json(format!("Err {:?}", err))
        }
    }
}