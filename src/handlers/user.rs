use crate::resolvers::{jsonapi::Model, response};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone)]
struct User {
    #[validate(length(min = 3, message = "username should atleast 3 characters"))]
    #[validate(required)]
    username: Option<String>,
    #[validate(length(min = 3, message = "password should atleast 3 characters"))]
    #[validate(required)]
    password: Option<String>,
    #[validate(range(min = 18, message = "you must atleast 18 years old"))]
    #[validate(required)]
    age: Option<i16>,
    #[validate]
    friends: Option<Vec<User>>,
}

async fn new_user(input: web::Json<Model<User>>) -> HttpResponse {
    let result = input.validate();

    if let Err(errors) = result {
        return response::validation_error(errors);
    }

    let data = input.clone().attributes();

    response::ok(User {
        username: data.username.to_owned(),
        password: data.password.to_owned(),
        age: data.age,
        friends: None,
    })
}

async fn get_user() -> String {
    "hello user".into()
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::resource("/user")
            .route(web::get().to(get_user))
            .route(web::post().to(new_user)),
    );
}
