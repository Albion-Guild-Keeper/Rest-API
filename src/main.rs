use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, get},
    HttpResponse,
};
use serde_json::json;
use shuttle_actix_web::ShuttleActixWeb;

mod controllers;
mod database;
mod models;
mod routes;
mod utils;

use crate::routes::{
    auth::sign_in::sign_in, auth::sign_up::sign_up, discord::join_discord, test::test,
    user::join_user, accounts::get_account, auth::discord::discord_auth,
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
                .service(join_discord)
                .service(test)
                .service(join_user)
                .service(sign_in)
                .service(sign_up)
                .service(get_account)
                .service(discord_auth),  
                
        );
    };

    println!("Starting server at http://localhost:8000");
    Ok(config.into())
}
