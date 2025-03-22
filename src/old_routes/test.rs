use std::fmt::format;

use actix_web::cookie::Cookie;
use actix_web::web::{Json, Path};
use actix_web::{get, put, HttpRequest, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::controllers::test as controller;
use crate::database::connect as database;
use crate::utils::surreal_int::SurrealInt;

struct Body {
    token_url: &'static str,
    redirect_url: &'static str,
    client_id: &'static str,
    client_secret: &'static str,
    code: String,
    scopes: Vec<&'static str>,
}
struct Headers {
    header: &'static str,
    value: &'static str,
}

#[derive(Deserialize, Serialize)]
struct Token {
    access_token: String,
    expires_in: SurrealInt,
    refresh_token: String,
    scope: String,
    token_type: String,
}

#[get("/test/callback")]
pub async fn test(req: HttpRequest) -> HttpResponse {
    let code = req
        .query_string()
        .to_string()
        .split("=")
        .collect::<Vec<&str>>()[1]
        .to_string();

    let body = Body {
        token_url: "https://discord.com/api/oauth2/token",
        redirect_url: "http://localhost:8000/api/v1/callback",
        client_id: "1248308695323115543",
        client_secret: "t2DWinSWnDhbAAw6kpJbvDjVyRfCDoN4",
        code: code.clone(),
        scopes: vec!["identify", "guilds", "guilds.members.read"],
    };

    let headers = Headers {
        header: "Content-Type",
        value: "application/x-www-form-urlencoded",
    };

    let req_oauth = Client::new()
        .post(body.token_url)
        .header(headers.header, headers.value)
        .body(format!("client_id={}&client_secret={}&grant_type=authorization_code&code={}&redirect_uri={}&scope={}", body.client_id, body.client_secret, body.code, body.redirect_url, body.scopes.join(" ")))
        .send()
        .await
        .unwrap();

    let token: Token = req_oauth.json().await.unwrap();

    let req_test = Client::new()
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .unwrap();

    let user_info: serde_json::Value = req_test.json().await.unwrap();

    let db = database::connect().await.unwrap();

    let auth = db.authenticate(token.access_token.clone()).await.unwrap();

    println!("{:#?}", auth);

    HttpResponse::Ok().json(user_info)
}

#[get("/test")]
pub async fn cookies() -> Result<HttpResponse, actix_web::Error> {
    // match controller::metto_cookie().await {
    //     Ok(cookie) => Ok(HttpResponse::Ok().cookie(cookie).finish()),
    //     Err(_) => Ok(HttpResponse::InternalServerError().body("Cookies failed")),
    // }

    let db = database::connect().await.unwrap();
    let query = format!("RETURN $auth");
    let result = db.query(query).await.unwrap();

    println!("Query eseguita con successo: {:#?}", result);
    Ok(HttpResponse::Ok().body("Query executed successfully"))
}

#[get("/test/{cookie_name}")]
pub async fn prendo_cookie(
    req: HttpRequest,
    cookie_name: Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(cookie) = req.cookie(cookie_name.into_inner().as_str()) {
        println!("Cookie value: {}", cookie.value());
    } else {
        println!("Cookie not found");
    }
    Ok(HttpResponse::Ok().body("test"))
}
