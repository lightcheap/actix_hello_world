use std::io::Result;
use actix_web::{App,
    HttpServer,
    // Responder,
    // HttpResponse,
    // get,
    web,
    middleware::Logger
};
use tera::Tera; // テンプレートエンジン Tera
use env_logger::Env;

mod handler;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        // Teraのインスタンス生成
        let mut tera = Tera::new("templates/**/*.html").unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .service(handler::index) // 一覧
            .service(handler::new) // 新規作成 show()よりも前に登録しないと動作しない
            .service(handler::create) //
            .service(handler::edit) // 編集
            .service(handler::update) // 更新
            .service(handler::destroy) // 削除
            .service(handler::show) // 詳細
            .default_service(web::to(handler::not_found)) // not found
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
