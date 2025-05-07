use std::env;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
// local modules
mod cmd;
mod http;
mod ws;

use http::routes::index;
use ws::connection::{handler, Clients, Tx};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    env_logger::init();

    println!("Starting WebSocket server at ws://127.0.0.1:8080/ws/");

    HttpServer::new(move || {
        App::new()
            //.configure(configure)
            .service(Files::new("/web", "./web"))
            .service(index)
            .wrap(Logger::default()) // ← logs requests & errors :contentReference[oaicite:0]{index=0}
            .app_data(web::Data::new(clients.clone())) // ← inject shared Clients :contentReference[oaicite:1]{index=1}
            .route("/ws/", web::get().to(handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
