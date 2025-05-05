use std::env;

use actix_web::{web, App, HttpServer};

// local modules
mod cmd;
mod http;
mod ws;

use http::routes::index;
use ws::connection::handler;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    println!("Starting WebSocket server at ws://127.0.0.1:8080/ws/");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/ws/", web::get().to(handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
