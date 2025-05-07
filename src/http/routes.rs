use std::fs;

use actix_web::{get, HttpResponse, Responder};

//const WEB_ROOT: &str = "web";

#[get("/")]
async fn index() -> impl Responder {
    match fs::read_to_string("web/main.html") {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().body("Could not load main.html"),
    }

    //let mut path = PathBuf::from(WEB_ROOT).join("index.html");
    //if !path.exists() {
    //    path = PathBuf::from(WEB_ROOT).join("mod.html");
    //}
    //
    //NamedFile::open_async(path).await
}
