use actix_web::{put, HttpResponse, web::Json};
use crate::controllers::auth::sign_in as controller;
use crate::models::auth::sign_in::SignIn;

#[put("auth/sign_in")]
pub async fn sign_in(user_data: Json<SignIn>) -> HttpResponse {
    let email = user_data.email.clone();
    let password = user_data.password.clone();

    match controller::sign_in(email, password).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}