use actix_web::{cookie::Cookie, HttpResponse};

pub async fn metto_cookie() -> Result<Cookie<'static>, HttpResponse> {
    let cookie: Cookie<'_> = Cookie::build("tuamadre", "tua madre zoccola puttana")
        .path("/")
        .http_only(true)
        .finish();

    println!("{:#?}", cookie);

    Ok(cookie)
}