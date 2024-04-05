use log::info;
use actix_web::{Responder, HttpResponse, web, get, post};
// データのモジュールであるdataの取り込みの指定
mod data;

#[get("/post")]
pub async fn post() ->impl Responder {
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
