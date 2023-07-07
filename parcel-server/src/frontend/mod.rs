pub mod api;
mod error;
mod result;

use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse,
};
use rust_embed::{EmbeddedFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct FrontendFiles;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(actix_web::web::scope("api").configure(api::configure_endpoints))
        .service(frontend)
        .service(frontend_index);
}

#[get("{_:.*}")]
pub async fn frontend(path: web::Path<String>) -> HttpResponse {
    match FrontendFiles::get(&path) {
        Some(content) => response_from_embedded_file(&path, &content),
        None => {
            // Try index.html for SPA functionality
            const INDEX_PATH: &str = "index.html";
            let index_file = FrontendFiles::get(INDEX_PATH);

            match index_file {
                Some(content) => response_from_embedded_file(INDEX_PATH, &content),
                None => HttpResponse::NotFound().body("404 Not Found"),
            }
        }
    }
}

#[get("")]
pub async fn frontend_index() -> HttpResponse {
    match FrontendFiles::get("index.html") {
        Some(content) => response_from_embedded_file("index.html", &content),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn response_from_embedded_file(path: &str, file: &EmbeddedFile) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
        .body(file.data.to_vec())
}
