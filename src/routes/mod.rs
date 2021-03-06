pub mod user;
pub mod ws;

use crate::server::AppState;
use actix_web::{get, post, web, web::Data, HttpResponse, Responder};

#[get("/")]
pub async fn hello(app: Data<AppState>) -> impl Responder {
    let app_name = &app.app_name;
    println!("App name: {}", app_name);
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


