use actix_web::{get, web, Scope};

#[get("")]
async fn get_user() -> String {
    "hello user".into()
}

pub fn scope() -> Scope {
    web::scope("/user").service(get_user)
}
