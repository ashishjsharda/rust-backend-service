use actix_web::{App, HttpServer};
use tracing_subscriber;

mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        App::new()
            .configure(routes::user::init)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
