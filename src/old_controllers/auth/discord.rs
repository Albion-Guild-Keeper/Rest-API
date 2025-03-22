use actix_web::cookie::{time::Duration, Cookie};
use actix_web::{HttpRequest, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::{Credentials, Record};

use crate::database::connect as database;
use crate::utils::surreal_int::SurrealInt;

struct Body {
    token_url: Option<&'static str>,
    redirect_url: Option<&'static str>,
    client_id: Option<&'static str>,
    client_secret: Option<&'static str>,
    code: Option<String>,
    scopes: Option<Vec<&'static str>>,
}

struct Headers {
    header: Option<&'static str>,
    value: Option<&'static str>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Token {
    access_token: Option<String>,
    expires_in: Option<SurrealInt>,
    refresh_token: Option<String>,
    scope: Option<String>,
    token_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserInfo {
    id: Option<String>,
    username: Option<String>,
    avatar: Option<String>,
    global_name: Option<String>,
    locale: Option<String>,
}

pub async fn discord_callback<'a>(req: HttpRequest) -> Result<Cookie<'static>, CallBackError> {
    println!("Inizio della funzione discord_callback");

    let query_string = req.query_string().to_string();
    println!("Query string ricevuta: {}", query_string);

    // ! TROVO CODE

    let code = query_string
        .split('=')
        .nth(1)
        .map(|s| s.to_string())
        .ok_or(CallBackError::MissingCode)?;
    println!("Codice estratto: {}", code);

    let body = Body {
        token_url: Some("https://discord.com/api/oauth2/token"),
        redirect_url: Some("http://localhost:8000/api/v1/auth/discord/callback"),
        client_id: Some("1248308695323115543"),
        client_secret: Some("t2DWinSWnDhbAAw6kpJbvDjVyRfCDoN4"),
        code: Some(code),
        scopes: Some(vec!["identify", "guilds", "guilds.members.read"]),
    };
    println!("Corpo della richiesta creato");

    let headers = Headers {
        header: Some("Content-Type"),
        value: Some("application/x-www-form-urlencoded"),
    };
    println!("Headers creati");

    let req = Client::new()
        .post(body.token_url.unwrap())
        .header(headers.header.unwrap(), headers.value.unwrap())
        .body(format!(
            "client_id={}&client_secret={}&grant_type=authorization_code&code={}&redirect_uri={}&scope={}",
            body.client_id.unwrap(), body.client_secret.unwrap(), body.code.unwrap(), body.redirect_url.unwrap(), body.scopes.unwrap().join(" ")
        ))
        .send()
        .await
        .map_err(|_| CallBackError::TokenRequestFailed)?;
    println!("Richiesta token inviata");

    // ! RICHIEDO TOKEN

    if !req.status().is_success() {
        println!("Errore nella risposta HTTP: {}", req.status());
        return Err(CallBackError::HttpResponseError);
    }
    println!("Risposta HTTP ricevuta con successo");

    let token_response: Token = req
        .json()
        .await
        .map_err(|_| CallBackError::TokenResponseParseError)?;
    println!("Token ricevuto: {:?}", token_response);

    // ! RICEVO TOKEN

    let req_test = Client::new()
        .get("https://discord.com/api/users/@me")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                token_response
                    .access_token
                    .as_ref()
                    .ok_or(CallBackError::MissingCode)?
            ),
        )
        .send()
        .await
        .map_err(|_| CallBackError::HttpResponseError)?;
    println!("Richiesta informazioni utente inviata");

    // ! RICHEIDO USER INFO

    let user_info: UserInfo = req_test
        .json()
        .await
        .map_err(|_| CallBackError::TokenResponseParseError)?;
    println!("Informazioni utente ricevute: {:?}", user_info);

    let db = database::connect()
        .await
        .map_err(|_| CallBackError::DatabaseConnectionError)?;

    println!("Connessione al database stabilita");

    #[derive(Serialize, Debug)]
    struct Credentials<'a> {
        id: &'a str,
        username: &'a str,
        avatar: &'a str,
        global_name: &'a str,
        locale: &'a str,
    }

    let credentials = Credentials {
        id: user_info.id.as_ref().unwrap(),
        username: user_info.username.as_ref().unwrap(),
        avatar: user_info.avatar.as_ref().unwrap(),
        global_name: user_info.global_name.as_ref().unwrap(),
        locale: user_info.locale.as_ref().unwrap(),
    };

    println!("Credentials: {:?}", credentials);

    let signin = db
        .signin(Record {
            namespace: "root",
            database: "root",
            access: "tokens",
            params: &credentials,
        })
        .await
        .map_err(|e| CallBackError::DatabaseQueryError(e.to_string()))?;

    println!("Signin eseguito con successo: {:#?}", signin);

    dbg!(signin.as_insecure_token());

    let test = db.query("SELECT * FROM accounts").await.map_err(|e| CallBackError::DatabaseQueryError(e.to_string()))?;

    println!("Query eseguita con successo: {:#?}", test);

    // let auth = db
    //     .authenticate(token_response.access_token.clone().unwrap())
    //     .await
    //     .unwrap();
    // println!("{:#?}", auth);

    // let query = format!(
    //     "fn::new_account({}, '{}', '{}', '{}', '{}')",
    //     SurrealInt::from(
    //         user_info
    //             .id
    //             .as_ref()
    //             .unwrap()
    //             .parse::<SurrealInt>()
    //             .map_err(|_| CallBackError::ParsingError)?
    //     ),
    //     user_info.username.as_ref().unwrap(),
    //     user_info.avatar.as_ref().unwrap(),
    //     user_info.global_name.as_ref().unwrap(),
    //     user_info.locale.as_ref().unwrap()
    // );

    // println!("Query creata: {:#?}", query);

    // let res = db
    //     .query(query)
    //     .await
    //     .map_err(|_| CallBackError::DatabaseQueryError)?;

    // println!("Query eseguita con successo: {:#?}", res);

    // let access_token: &str = token_response
    //     .access_token
    //     .as_ref()
    //     .map(|s| s.as_str())
    //     .ok_or(CallBackError::MissingCode)?;

    // println!("Access token received: {}", access_token);

    // // let query_token = format!("fn::new_token('{}')", access_token);
    // // let res_token = db.query(query_token).await.map_err(|_| CallBackError::DatabaseQueryError)?;
    // // println!("Token query executed successfully: {:#?}", res_token);

    // let query_authenticate = format!("fn::authenticate('{}')", access_token);
    // let res_authenticate = db.query(query_authenticate).await.map_err(|_| CallBackError::DatabaseQueryError)?;

    // println!("Authentication query executed successfully: {:#?}", res_authenticate);

    // let cookie = Cookie::build("access_token", access_token.to_owned())
    //     .path("/")
    //     .http_only(true)
    //     .secure(true)
    //     .max_age(Duration::seconds(token_response.expires_in.unwrap().0))
    //     .finish();

    // println!("Cookie created successfully: {:#?}", cookie);

    Ok(Cookie::build("access_token", "test")
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(Duration::days(1))
        .finish())
}

#[derive(Debug)]
pub enum CallBackError {
    MissingCode,
    TokenRequestFailed,
    HttpResponseError,
    TokenResponseParseError,
    DatabaseConnectionError,
    DatabaseQueryError(String),
    ParsingError,
    InvalidTokenError,
}