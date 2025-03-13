use actix_cors::Cors;
use actix_web::{get, web::{self}, HttpResponse};
use serde_json::json;
use shuttle_actix_web::ShuttleActixWeb;

mod controllers;
mod database;
mod models;
mod routes;
mod utils;

use crate::routes::{
    auth::login::login, discord::join_discord, test::test,
    user::join_user, auth::auth::auth as is_auth,
};

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
                .service(login)
                .service(join_discord)
                .service(test)
                .service(join_user)
                .service(is_auth),
        );
    };
    
    println!("Starting server at http://localhost:8000");
    Ok(config.into())
}
