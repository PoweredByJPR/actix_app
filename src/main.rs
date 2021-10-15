#[macro_use]
extern crate log;
use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger::Env;

mod handlers;
mod resolvers;

async fn health() -> HttpResponse {
    HttpResponse::Ok().json("healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting server");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1")
                .service(web::resource("").to(health))
                .configure(handlers::user::config),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
