use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use log::{error, info};

use crate::cmd::nu::execute_command;

pub async fn handler(req: HttpRequest, body: web::Payload) -> Result<HttpResponse, Error> {
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
