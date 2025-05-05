use std::path::PathBuf;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, Responder};

const WEB_ROOT: &str = "web";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/", "./web").index_file("mod.html"));
}

#[get("/")]
async fn index() -> impl Responder {
    let mut path = PathBuf::from(WEB_ROOT).join("index.html");
    if !path.exists() {
        path = PathBuf::from(WEB_ROOT).join("mod.html");
    }

    NamedFile::open_async(path).await
}
