use crate::controllers::auth::callback as controller;
use actix_web::{get, web::Query, HttpResponse};

#[derive(serde::Deserialize)]
struct QueryParams {
    code: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/callback",
    responses(
        (status = 200, description = "Callback", body = String),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    ),
    tags = ["Auth"],
    params(
        ("code" = String, Query, description = "Authorization code from Discord"),
    ),
)]
#[get("/auth/callback")]
async fn callback(query: Query<QueryParams>) -> HttpResponse {
    match controller::callback(query.code.clone()).await {
        Ok(_) => HttpResponse::Found()
            .append_header(("Location", "http://localhost:8000/"))
            .finish(),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}