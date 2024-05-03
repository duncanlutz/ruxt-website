use actix_files::Files;
use actix_web::{web, App, HttpServer};

pub mod pages;

fn get_app_scope() -> actix_web::Scope {
    web::scope("/static").service(Files::new("", "./src/public"))
}

#[ruxt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_app_scope()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
