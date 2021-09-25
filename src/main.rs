use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod schemas;
mod services;

#[get("/")]
async fn info() -> impl Responder {
    HttpResponse::Ok().body("Actix App")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(services::user::scope())
            .service(services::auth::scope())
            .service(info)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
