use crate::routes::ws::connect;
use actix_web::web;

pub fn init(app: &mut web::ServiceConfig) {
    app.service(web::resource("/ws").to(connect));
}
