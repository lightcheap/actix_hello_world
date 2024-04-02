use log::info;
use actix_web::{Responder, HttpResponse, get};

#[get("/post")]
pub async fn post() ->impl Responder {
    info!("Called post");
    HttpResponse::Ok().body("called post")
}
