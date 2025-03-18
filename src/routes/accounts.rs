use actix_web::{get, HttpResponse};
use actix_web::web::Path;
use crate::controllers::accounts as controller;
use crate::utils::surreal_int::SurrealInt;

#[get("/accounts/{user_id}")]
pub async fn get_account(user_id: Path<SurrealInt>) -> HttpResponse {
    match controller::get_account(user_id.into_inner()).await {
        true => HttpResponse::Ok().json(true),
        false => HttpResponse::NotFound().json("12345"),
    }
}