use log::info;
use actix_web::{
    Responder,
    HttpResponse,
    web,
    get,
    post
};
use serde::{Deserialize};
use chrono::{DateTime, Local};
// テンプレートエンジンTeraを使う
use tera::Context;

// データのモジュールであるdataの取り込みの指定
mod data;

/* ------------
    ハンドラー：その他のFWで言うところのControllerみたいなもん？
*/

// ページが見つかりません
pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page Not found!")
}

// TOP画面
#[get("/")]
pub async fn home(tmpl: web::Data<tera::Tera>) ->impl Responder {
    info!("Called home");
    let context = Context::new();
    let body_str: String = tmpl.render("home.html", &context).unwrap();

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
}

// 一覧画面
#[get("/posts")]
pub async fn index(tmpl: web::Data<tera::Tera>) ->impl Responder {
    info!("Called Index");
    // 全投稿データを取り込み Message型
    let posts = data::get_all();
    let mut context = Context::new();
    context.insert("posts", &posts);
    let body_str = tmpl.render("index.html", &context).unwrap();

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)

}

// 投稿表示
#[get("/posts/{id}")]
pub async fn show(tmpl: web::Data<tera::Tera>, info: web::Path<i32>) -> impl Responder {
    info!("Called show");
    let info = info.into_inner();
    let posts = data::get(info);
    let mut context = Context::new();
    context.insert("post", &posts);
    let body_str = tmpl.render("show.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
}

// 投稿新規作成フォーム画面
#[get("/posts/new")]
pub async fn new(tmpl: web::Data<tera::Tera>) -> impl Responder {
    info!("Called new");
    let mut context = Context::new();
    let post = data::Message {id:0, sender:"".to_string(), content:"".to_string(), posted:"".to_string()};
    context.insert("action", "create");
    context.insert("post", &post);
    context.insert("button", "投稿");
    let body_str = tmpl.render("form.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
}

// 編集画面
#[get("/posts/{id}/edit")]
pub async fn edit(tmpl: web::Data<tera::Tera>, info: web::Path<i32>) -> impl Responder {
    info!("Called edit");
    let info = info.into_inner();
    let post = data::get(info);
    let mut context = Context::new();
    context.insert("action", "update");
    context.insert("post", &post);
    context.insert("button", "更新");
    let body_str = tmpl.render("form.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
}

#[derive(Deserialize, Debug)]
pub struct CreateForm {
    id: i32,
    posted: String,
    sender: String,
    content: String,
}

// 新規投稿登録
#[post("/posts/create")]
pub async fn create(params: web::Form<CreateForm>) -> impl Responder {
    info!("Called create");
    let now: DateTime<Local> = Local::now();
    let mut message = data::Message {
        id: 0,
        posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone()
    };
    message = data::create(message);
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
    //web::Redirect::to("/posts").see_other()
}

// 更新
#[post("/posts/update")]
pub async fn update(params: web::Form<CreateForm>) -> impl Responder {
    info!("Called update");
    let message = data::Message {
        id: params.id,
        posted: params.posted.clone(),
        sender: params.sender.clone(),
        content: params.content.clone()
    };
    data::update(&message);
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
    //web::Redirect::to("/posts").see_other()
}

// 削除
#[get("/posts/{id}/delete")]
pub async fn destroy(info: web::Path<i32>) -> impl Responder {
    info!("Called destroy");
    let info = info.into_inner();
    data::remove(info);
    web::Redirect::to("/posts").see_other()
}
