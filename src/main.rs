mod oss;
mod demo_get_oss_and_chat;
mod base_body;
mod demo_send_img_and_chat;

use futures::StreamExt;
use reqwest::{Client};
use serde::{Deserialize, Serialize};
use substring::Substring;
use tokio;
use crate::oss::OssConfig;
use crate::base_body::{send_and_return_stream, ChatBody, ChatResponse, Options};
use crate::demo_get_oss_and_chat::get_oss_and_chat;
use crate::demo_send_img_and_chat::send_img_and_chat;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{

    // DEMO: get oss and chat
    // get_oss_and_chat().await;

    // DEMO: load pic, transfer to base64, and send to multimodal model
    send_img_and_chat().await;

    Ok(())
}