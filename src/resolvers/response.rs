use crate::resolvers::jsonapi;
use actix_web::http::{header, StatusCode};
use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

fn build<B: Serialize>(status_code: StatusCode, body: B) -> HttpResponse {
    HttpResponse::build(status_code)
        .content_type("application/json")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
        .json(body)
}

pub fn ok<B: Serialize>(body: B) -> HttpResponse {
    build(StatusCode::OK, body)
}

pub fn not_found<B: Serialize>(body: B) -> HttpResponse {
    build(StatusCode::NOT_FOUND, body)
}

pub fn unprocessable_entity<B: Serialize>(body: B) -> HttpResponse {
    build(StatusCode::UNPROCESSABLE_ENTITY, body)
}

pub fn internal_server_error<B: Serialize>(body: B) -> HttpResponse {
    build(StatusCode::INTERNAL_SERVER_ERROR, body)
}

pub fn validation_error(errors: validator::ValidationErrors) -> HttpResponse {
    let errors = jsonapi::map_errors(errors, None);
    unprocessable_entity(json!({ "errors": errors }))
}
