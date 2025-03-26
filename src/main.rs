use actix_cors::Cors;
use actix_web::{
    get,
    web::{self},
    HttpResponse,
};
use serde_json::json;
use shuttle_actix_web::ShuttleActixWeb;

mod controllers;
mod middlewares;
mod errors;
mod models;
mod routes;
mod utils;

use crate::routes::{test::test, authentication::discord_callback::discord_callback, guild_create::guild_create, authentication::get_guilds::get_guilds};

#[get("/")]
async fn admin_check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({ "is_admin": true }))
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        cfg.service(
            web::scope("/api/v1")
                .wrap(cors)
                .service(admin_check)
                .service(discord_callback)
                .service(guild_create)
                .service(get_guilds)
                .service(test),
                
        );
    };

    println!("Starting server at http://localhost:8000");
    Ok(config.into())
}
