use actix_web::{get, HttpResponse};
use crate::controllers::test as controller;

#[get("/test")]
async fn test() -> Result<HttpResponse, actix_web::Error> {

    let config = ezoauth::OAuthConfig {
        auth_url: "https://discord.com/api/oauth2/authorize",
        token_url: "https://discord.com/api/oauth2/token",
        redirect_url: "http://localhost:8000/api/v1/auth/discord",
        client_id: "1248308695323115543",
        client_secret: "oR6kPICMVR6DQxDCJjfl_hcUuvo2z7vN",
        scopes: vec!["identify", "guilds", "guilds.members.read"],
    };
    

    let (rx, auth_url) = ezoauth::authenticate(config, "localhost:8000").map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("OAuth authentication error: {}", e))
    })?;

    println!("Browse to: {}\n", auth_url);

    let token = rx.recv().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Error receiving token: {}", e))
    })?;

    let token_value = token.unwrap();
    println!("Full Token: {:?}", &token_value);
    println!("Token Access: {:#?}", token_value.access_token());
    println!("Token Expire: {:#?}", token_value.expires_in());
    println!("Token Type: {:#?}", token_value.token_type());

    Ok(HttpResponse::Ok().json(token_value.access_token()))
}