use crate::routes::{echo, hello, manual_hello};
use actix_web::web;

pub struct AppState {
    pub app_name: String,
}

pub fn app_data(cfg: &mut web::ServiceConfig) {
    cfg.data(AppState {
        app_name: String::from("Testing!"),
    });
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello));
}
