use crate::schemas;
use actix_web::{post, web, HttpResponse, Scope};
use serde_json;

#[post("")]
async fn authenticate(payload: web::Bytes) -> HttpResponse {
    let body = String::from_utf8(payload.to_vec()).unwrap();
    let input = serde_json::from_str(&body).unwrap();
    let error = schemas::login::validate(&input);

    if let Err(error) = error {
        return HttpResponse::UnprocessableEntity().json(error);
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

pub fn scope() -> Scope {
    web::scope("/auth").service(authenticate)
}
