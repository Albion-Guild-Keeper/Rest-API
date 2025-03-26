use actix_web::{get, HttpRequest, HttpResponse};
use crate::controllers::authentication::get_guilds::get_guilds as controller;
use crate::middlewares::is_me_auth::is_me_auth;

#[get("/guilds/@me")]
pub async fn get_guilds(req: HttpRequest) -> HttpResponse {
        match is_me_auth(req).await {
    Ok(cookies) => {
        match controller(cookies.clone(), cookies.user_id.clone()).await {
            Ok(response) => {
                
                HttpResponse::Ok().json(response)
            },
                Err(err) => {
                    HttpResponse::InternalServerError().json(format!("Err {:?}", err))
                }
            }
        }
        Err(error) => HttpResponse::InternalServerError().body(format!("Error: {:#?}", error)),
    }
}