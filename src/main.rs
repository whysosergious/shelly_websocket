use std::{env, path::PathBuf};

use actix_files::NamedFile;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use log::{error, info};

use tokio::io::AsyncReadExt;
use tokio::process::Command;

#[get("/")]
async fn index() -> impl Responder {
    let mut path = PathBuf::from("index.html");
    if !path.exists() {
        path = PathBuf::from("mod.html");
    }

    NamedFile::open_async(path).await
}

/// spawn a process to execute shell command
async fn execute_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: binary input/uotput
    // TODO: look into keepalive for e.g. sqeel operations

    let output = Command::new("nu").arg("-c").arg(command).output().await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Nushell error: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

async fn ws_handler(req: HttpRequest, body: web::Payload) -> Result<HttpResponse, Error> {
    // Initiate the WebSocket handshake
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Received text message: {}", text);
                    match execute_command(&text).await {
                        Ok(output) => {
                            if let Err(e) = session.text(output).await {
                                error!("Error sending message: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error handling message: {}", e);
                            break;
                        }
                    }
                }
                Message::Binary(bin) => {
                    info!("Received binary message: {:?}", bin);
                    if let Err(e) = session.binary(bin).await {
                        error!("Error sending binary message: {}", e);
                        break;
                    }
                }
                Message::Close(reason) => {
                    info!("Received close message: {:?}", reason);
                    let _ = session.close(reason).await;
                    break;
                }
                Message::Ping(bytes) => {
                    info!("Received ping: {:?}", bytes);
                    if let Err(e) = session.pong(&bytes).await {
                        error!("Error sending pong: {}", e);
                        break;
                    }
                }
                Message::Pong(_) => {
                    info!("Received pong");
                }
                Message::Continuation(_) => {
                    info!("Received continuation message");
                }
                Message::Nop => {
                    info!("Received NOP");
                }
            }
        }
    });

    Ok(response)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    println!("Starting WebSocket server at ws://127.0.0.1:8080/ws/");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/ws/", web::get().to(ws_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
