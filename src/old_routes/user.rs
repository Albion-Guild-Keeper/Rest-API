use actix_web::{get as GET, HttpRequest, HttpResponse};

use crate::controllers::user as controller;
use crate::utils::middleware::{middleware, MiddlewareError::*};

#[GET("/user/@me")]
pub async fn get_user(req: HttpRequest) -> HttpResponse {
    let middleware_result = middleware(&req).await;
    match middleware_result {
        Ok(middleware) => match middleware {
            TokenValid => match controller::get_user(&req).await {
                Ok(response) => HttpResponse::Ok().body(response),
                Err(e) => HttpResponse::InternalServerError().body(e),
            },
            TokenMissing => HttpResponse::Unauthorized().body("Token missing"),
            TokenExpired => HttpResponse::Unauthorized().body("Token expired"),
            TokenInvalid => HttpResponse::Unauthorized().body("Token invalid"),
            UnexpectedError => HttpResponse::InternalServerError().body("Unexpected error"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}