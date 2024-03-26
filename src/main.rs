use actix_web::{get, web, App, HttpServer, Responder};

// この構造体は状態を表します
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // get app_name
    format!("Hello my name is {app_name}!")   // response
}

// アプリの初期化時に状態を渡し、アプリケーションを起動する
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("actix web!!!"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
