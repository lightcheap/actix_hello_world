use log::info;
use actix_web::{
    Responder,
    HttpResponse,
    web,
    get,
    post
};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, Duration};

// データのモジュールであるdataの取り込みの指定
mod data;

// ページが見つかりません
pub async fn not_found() -> impl Responder {
    info!("called not found");
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += "<h2>Page Not Found</h2>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::NotFound().content_type("text/html; charset=utf-8").body(body_str)
}

// 一覧画面
#[get("/posts")]
pub async fn index() ->impl Responder {
    info!("Called Post!!");
    // 全投稿データを取り込み Message型
    let posts = data::get_all();
    let mut body_str: String = "".to_string();
    // 表示内容（文字列）の読み込み
    // ヘッダー読み込み
    body_str += include_str!("../static/header.html");
    for item in &posts {
        body_str += &format!("<div><a href=\"/posts/{}\">", item.id);
        body_str += &format!("<div>{} {}</div>", item.sender, item.posted);
        body_str += &format!("<div><p>{}</p></div>",
            item.content.replace("\n", "<br />"));
        body_str += "</a></div>";
    }
    body_str += "<div><a href=\"/posts/new\">作成</a></div>";
    // フッター読み込み
    body_str += include_str!("../static/footer.html");

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
    // info!("Called post");
    // HttpResponse::Ok().body("called post")
}

// 投稿表示
#[get("/posts/{id}")]
pub async fn show(info: web::Path<i32>) -> impl Responder {
    info!("Called show");
    let info = info.into_inner();
    let posts = data::get(info);
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += "<div>";
    if posts.id != 0 {
        body_str += &format!("<div>投稿者：{}</div>", posts.sender);
        body_str += &format!("<div>投稿日時：{}</div>", posts.posted);
        body_str += &format!("<div>投稿内容：<br />{}</div>", posts.content.replace("\n", "<br />"));
        body_str += &format!("<div><a href=\"/posts/{}/edit\">編集</a>&nbsp;", info);
        body_str += &format!("<a href=\"/posts/{}/delete\">削除</a><div>", info);
    } else {
        body_str += "見つかりません。";
    }
    body_str += "</div>";
    body_str += "<div><a href=\"/posts\">一覧へ</a></div>";
    body_str += include_str!("../static/footer.html");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
}

#[get("/posts/new")]
pub async fn new() -> impl Responder {
    info!("Called new");
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += include_str!("../static/form.html");
    body_str += include_str!("../static/footer.html");
    body_str = body_str.replace("{{action}}", "create");
    body_str = body_str.replace("{{id}}", "0");
    body_str = body_str.replace("{{posted}}", "");
    body_str = body_str.replace("{{sender}}", "");
    body_str = body_str.replace("{{content}}", "");
    body_str = body_str.replace("{{button}}", "登録");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
}

#[get("/posts/{id}/edit")]
pub async fn edit(info: web::Path<i32>) -> impl Responder {
    info!("Called edit");
    let info = info.into_inner();
    let posts = data::get(info);
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += include_str!("../static/form.html");
    body_str += include_str!("../static/footer.html");
    body_str = body_str.replace("{{action}}", "update");
    body_str = body_str.replace("{{id}}", &posts.id.to_string());
    body_str = body_str.replace("{{posted}}", &posts.posted);
    body_str = body_str.replace("{{sender}}", &posts.sender);
    body_str = body_str.replace("{{content}}", &posts.content);
    body_str = body_str.replace("{{button}}", "更新");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
}

#[derive(Deserialize, Debug)]
pub struct CreateForm {
    id: i32,
    posted: String,
    sender: String,
    content: String,
}

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
}

#[post("/posts/update")]
pub async fn update(params: web::Form<CreateForm>) -> impl Responder {
    info!("Called update");
    let mut message = data::Message {
        id: params.id,
        posted: params.posted.clone(),
        sender: params.sender.clone(),
        content: params.content.clone()
    };
    data::update(&message);
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}

#[get("/posts/{id}/delete")]
pub async fn destroy(info: web::Path<i32>) -> impl Responder {
    info!("Called destroy");
    let info = info.into_inner();
    data::remove(info);
    web::Redirect::to("/posts").see_other()
}
