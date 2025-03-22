use actix_web::{get, HttpRequest, HttpResponse};

use crate::middlewares::is_me_auth;

#[get("/test")]
pub async fn test(req: HttpRequest) -> HttpResponse {
    match is_me_auth::is_me_auth(req).await {
        Ok(response) => {
            dbg!(&response);
            // * Call other function
            let additional_response = "some_other_function(response)".to_string();
            HttpResponse::Ok().json(additional_response)
        },
        Err(error) => HttpResponse::InternalServerError().body(format!("Error: {:#?}", error)),
    }
}
