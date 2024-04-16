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
// テンプレートエンジン Tera
use tera::Tera;
// session
use actix_web::cookie::{Key}; // cookieのキーを使えるようにするための構造体
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
// フラッシュメッセージ
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::storage::SessionMessageStore;
// use actix_web_flash_messages::storage::CookieMessageStore;

mod handler;

// クッキーベースのセッションを使うための関数
fn build_cookie_session_middleware(key: Key)-> SessionMiddleware<CookieSessionStore> {
    // インスタンスを返す
    SessionMiddleware::builder(CookieSessionStore::default(), key).build()
}


#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // クッキーのキーの生成
    let key = Key::generate();
    // メッセージストアにクッキーベースをセッションを使う場合
    let message_store = SessionMessageStore::default();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    HttpServer::new(move|| {
        // Teraのインスタンス生成
        let tera = Tera::new("templates/**/*.html").unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .service(handler::home) // top
            .service(handler::index) // 一覧
            .service(handler::new) // 新規作成 show()よりも前に登録しないと動作しない
            .service(handler::create) //
            .service(handler::edit) // 編集
            .service(handler::update) // 更新
            .service(handler::destroy) // 削除
            .service(handler::show) // 詳細
            .default_service(web::to(handler::not_found)) // not found
            .wrap(Logger::default())
            .wrap(message_framework.clone())
            .wrap(build_cookie_session_middleware(key.clone()))
        })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
