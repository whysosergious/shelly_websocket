use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let mut path = PathBuf::from("index.html");
    if !path.exists() {
        path = PathBuf::from("mod.html");
    }

    NamedFile::open_async(path).await
}
