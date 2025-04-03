use actix_web::{get, HttpResponse};
use crate::controllers::test as controller;

#[utoipa::path(
    get,
    path = "api/v1/test",
    responses(
        (status = 200, description = "Test endpoint", body = String),
    ),
    tags = ["Test"],
)]
#[get("/test")]
async fn test() -> HttpResponse {
    controller::test().await
}