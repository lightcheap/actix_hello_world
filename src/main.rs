use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello rust and actix web")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("へい　そこ")
}

async fn manual_hoge() -> impl Responder {
    HttpResponse::Ok().body("ほげふが")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/hoge", web::get().to(manual_hoge))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
