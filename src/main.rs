use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    get,
    web::{self},
    HttpResponse,
};
use serde_json::json;
use shuttle_actix_web::ShuttleActixWeb;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod controllers;
mod middlewares;
mod models;
mod routes;
mod utils;

use crate::routes::{auth::callback::callback, test::test};

#[get("/")]
async fn admin_check() -> HttpResponse {
    let mw = middlewares::is_authenticated_middleware::is_authenticated().await;

    println!("Middleware result: {:?}", mw);

    match mw {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Authenticated"})),
        Err(_) => HttpResponse::Unauthorized().json(json!({"message": "Unauthorized"})),
    }
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Test", description = "Test related Tags endpoints"),
        (name = "Auth", description = "Auth related Tags endpoints"),
    ),
    paths(
        routes::test::test,
        routes::auth::callback::callback,
    ),
)]
struct ApiDoc;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let openapi = ApiDoc::openapi();

    let config = move |cfg: &mut web::ServiceConfig| {
        let cors = Arc::new(
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header(),
        );

        // * Service for `/api/v1` endpoints
        cfg.service(
            web::scope("/api/v1")
                .wrap(cors.clone())
                .service(test)
                .service(callback)
                .service(web::scope("/admin").wrap(cors.clone()).service(admin_check)),
        );

        // * Swagger Service
        cfg.service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        );
    };

    println!("Starting server at http://localhost:8000");
    Ok(config.into())
}
