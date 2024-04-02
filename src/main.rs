use std::io::Result;
use actix_web::{get, App, HttpServer, Responder, HttpResponse, middleware::Logger};
use env_logger::Env;

mod handler;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix web!")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(handler::post)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
