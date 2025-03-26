use actix_web::{
    cookie::Cookie,
    get, HttpRequest, HttpResponse,
};

use crate::controllers::authentication::discord_callback as controller;

#[get("/auth/discord/callback")]
pub async fn discord_callback(req: HttpRequest) -> HttpResponse {
    match controller::discord_callback(req).await {
        Ok(cookies) => {
            let access_token = Cookie::build("access_token", cookies.access_token.clone())
                .path("/")
                .http_only(true)
                .secure(true)
                .finish();

            println!("Access Token: {}", &cookies.access_token);

            let user_id = Cookie::build("user_id", cookies.user_id.clone())
                .path("/")
                .http_only(true)
                .secure(true)
                .finish();

            println!("User ID: {}", &cookies.user_id);

            HttpResponse::Found()
                .cookie(access_token)
                .cookie(user_id)
                .append_header(("Location", "http://localhost:1420/"))
                .finish()
        }
        Err(error) => HttpResponse::InternalServerError().body(format!("Error: {:#?}", error)),
    }
}
