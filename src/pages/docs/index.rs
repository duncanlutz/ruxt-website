use actix_web::{web, Responder};

use crate::util::render_doc;

pub async fn get(tera: web::Data<tera::Tera>) -> Result<impl Responder, actix_web::Error> {
    render_doc(&tera, "Getting Started", "docs/getting-started.html")
}
