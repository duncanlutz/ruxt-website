use actix_files::Files;
use actix_web::{web, App, HttpServer};

pub mod pages;
pub mod util;

#[ruxt::main]
async fn main() -> std::io::Result<()> {
    let tera = tera::Tera::new("src/templates/**/*").unwrap();
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(Files::new("/static", "./src/public"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
