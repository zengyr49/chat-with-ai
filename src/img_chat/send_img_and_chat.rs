use crate::base_body::{send_and_return_all, ChatBody, Options};
use axum::Json;
use image::ImageReader;
use minio::s3::utils::b64encode;
use reqwest::Client;
use std::fs::File;
use std::io::{BufReader, Cursor};
use serde::{Deserialize, Serialize};

/// 定义入参
#[derive(Serialize, Deserialize)]
pub struct ImgBody {
    pub model: Option<String>,
    pub prompt: String,
    pub image: String
}

impl Default for ImgBody {
    fn default() -> Self {
        ImgBody {
            model: Some("llava:13b".to_string()),
            prompt: "".to_string(),
            image: "".to_string()
        }
    }
}

pub async fn send_img_and_chat(img_body: Json<ImgBody>) -> Result<String, String> {
    println!("Hello, world!");
    // 创建一个Client 实例
    let client = Client::new();

    println!("{}", "extracting picture......");
    // let pic_base64 = get_img_and_decode("/Users/zengyr7/Downloads/apple.jpeg".to_string());
    let image_path = img_body.0.image;
    if image_path.is_empty() {
        return Ok("image is empty".to_string());
    }

    let pic_base64 = get_img_and_decode(image_path);
    println!("extract succeed");
    let model_opt = img_body.0.model;
    let mut model_name;
    if model_opt.is_none() {
        model_name = "llava:13b".to_string();
    } else {
        model_name = model_opt.unwrap();
    }

    // 构造body
    let mut body = ChatBody {
        model: model_name,
        prompt: img_body.0.prompt,
        stream: false,
        options: Options {temperature: 0.8}, // temperature的默认值，0.8,值越大，回答越creative
        images:Some(vec![pic_base64])
    };

    // 发起请求，且以stream返回
    let mut stream_res = send_and_return_all(client, body).await;
    Ok(stream_res)
}

pub fn get_img_and_decode(img_path:String) -> String {
    let img_file = File::open(img_path).unwrap();
    let mut img_reader = BufReader::new(img_file);

    let img = ImageReader::new(&mut img_reader).with_guessed_format().unwrap().decode().unwrap();
    let mut img_bytes = Cursor::new(vec![]);
    img.write_to(&mut img_bytes, image::ImageFormat::Png).unwrap();

    let img_bytes_vec = img_bytes.get_mut();

    let base64_encoded_img = b64encode(img_bytes_vec);
    base64_encoded_img
}