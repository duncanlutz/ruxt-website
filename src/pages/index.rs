use actix_web::HttpResponse;

pub async fn get() -> Result<HttpResponse, actix_web::Error> {
    let tera = tera::Tera::new("src/templates/**/*").unwrap();
    let ctx = tera::Context::new();

    let rendered = tera.render("index.html", &ctx).map_err(|e| {
        eprintln!("Error rendering template: {}", e);
        actix_web::error::ErrorInternalServerError("Error rendering template")
    })?;

    Ok(HttpResponse::Ok().body(rendered))
}
