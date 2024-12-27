mod oss;
mod demo_get_oss_and_chat;
mod demo_send_img_and_chat;

mod demo_read_file_and_chat;
mod base_body;
mod img_chat;

use axum::Router;
use axum::routing::post;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use substring::Substring;
use tokio;
use img_chat::send_img_and_chat;

///
/// 主函数，暴露端点给到外部
///
///
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let app = Router::new()
        .route("/send_img_and_chat", post(send_img_and_chat::send_img_and_chat));

    // listener and server port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080".to_string()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}