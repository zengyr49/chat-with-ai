use std::fs::File;
use std::io::{BufReader, Cursor};
use reqwest::Client;
use crate::base_body::{send_and_return_stream, ChatBody, Options};
use image::ImageReader;
use minio::s3::utils::b64encode;

pub async fn send_img_and_chat() {
    println!("Hello, world!");
    // 创建一个Client 实例
    let client = Client::new();

    println!("{}", "extracting picture......");
    // let pic_base64 = get_img_and_decode("/Users/zengyr7/Downloads/apple.jpeg".to_string());
    let pic_base64 = get_img_and_decode("/Users/zengyr7/Downloads/coffee.webp".to_string());
    println!("extract succeed");

    let mut prompt = format!("根据图片回答问题，请用简体中文简洁地回答。
    问题: {}
    ", "请描述本图，图中包含什么物体");

    // 用minicpm-v2.6
    let mut body = ChatBody {
        model: "minicpm-v2.6".to_string(),
        prompt: prompt,
        stream: true,
        options: Options {temperature: 0.8}, // temperature的默认值，0.8,值越大，回答越creative
        images:Some(vec![pic_base64])
    };

    // 发起请求，且以stream返回
    let mut stream_res = send_and_return_stream(body, client).await;
}

///
/// 获取本地img路径，和解析成base64格式
///
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

#[cfg(test)]
mod tests {
    use crate::demo_send_img_and_chat::get_img_and_decode;

    #[test]
    fn test_get_base64_img() {
        let string = get_img_and_decode("/Users/zengyr7/Downloads/apple.jpeg".to_string());
        println!("{}", string);
    }
}