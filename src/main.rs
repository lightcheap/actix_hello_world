use actix_web::{web, App, HttpServer, HttpResponse, guard};

// 127.0.0.1:8080にアクセスすれば"localhost"と表示される
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Host("127.0.0.1"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("localhost") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
