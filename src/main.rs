use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use log::{error, info};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};

async fn spawn_nushell() -> Result<(ChildStdin, BufReader<ChildStdout>), Box<dyn std::error::Error>>
{
    let mut child = Command::new("nu")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let reader = BufReader::new(stdout);

    Ok((stdin, reader))
}

async fn handle_ws_message(
    msg: String,
    mut nushell_stdin: ChildStdin,
    mut nushell_stdout: BufReader<ChildStdout>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Write the command to Nushell
    nushell_stdin.write_all(msg.as_bytes()).await?;
    nushell_stdin.write_all(b"\n").await?;
    nushell_stdin.flush().await?;

    // Read the output from Nushell
    let mut output = String::new();
    nushell_stdout.read_line(&mut output).await?;

    Ok(output)
}

async fn ws_handler(req: HttpRequest, body: web::Payload) -> Result<HttpResponse, Error> {
    // Initiate the WebSocket handshake
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    // Spawn a task to handle incoming messages
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Received text message: {}", text);
                    if let Err(e) = session.text(format!("Echo: {}", text)).await {
                        error!("Error sending message: {}", e);
                        break;
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

    // Return the response to establish the WebSocket connection
    Ok(response)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Starting WebSocket server at ws://127.0.0.1:8080/ws/");

    HttpServer::new(|| App::new().route("/ws/", web::get().to(ws_handler)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
