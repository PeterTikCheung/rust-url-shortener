//ensures that any macros defined in the imported crate can be used
use actix_web::{web, App, HttpServer};

mod db;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}