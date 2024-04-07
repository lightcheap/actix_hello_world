use std::io::Result;
use actix_web::{App,
    HttpServer,
    // Responder,
    // HttpResponse,
    // get,
    web,
    middleware::Logger
};

use env_logger::Env;

mod handler;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(handler::index) // 一覧

            .service(handler::new) // 新規作成 show()よりも前に登録しないと動作しない
            .service(handler::create) //
            .service(handler::show) // 詳細
            .default_service(web::to(handler::not_found))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
