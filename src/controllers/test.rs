use actix_web::HttpResponse;

use crate::middlewares;

pub async fn test() -> HttpResponse {
    let mw = middlewares::is_authenticated_middleware::is_authenticated()
        .await
        .unwrap();

    HttpResponse::Ok().json(mw)
}