use actix_web::{App, HttpServer};
use lib::server::{app_data, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        return App::new().configure(app_data).configure(routes);
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
