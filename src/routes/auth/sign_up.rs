use actix_web::{put, HttpResponse, web::Json};
use crate::models::auth::sign_up::SignUp;
use crate::controllers::auth::sign_up as controller;

#[put("auth/sign_up")]
pub async fn sign_up(user_data: Json<SignUp>) -> HttpResponse {
    let username = user_data.username.clone();
    let email = user_data.email.clone();
    let password = user_data.password.clone();
    
    match controller::sign_up(username, email, password).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}