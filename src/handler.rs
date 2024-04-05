use log::info;
use actix_web::{Responder, HttpResponse, web, get, post};

// mod data;

#[get("/post")]
pub async fn post() ->impl Responder {
    let mut body_str: String = "".to_string();
    body_str += include_str!("../static/header.html");
    body_str += include_str!("../static/footer.html");

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body_str)
    // info!("Called post");
    // HttpResponse::Ok().body("called post")
}
