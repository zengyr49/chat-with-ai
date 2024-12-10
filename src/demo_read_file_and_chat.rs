use crate::{send_and_return_stream, ChatBody, Options};
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use substring::Substring;

pub async fn read_file_and_chat() {
    println!("Hello, world!");
    // 创建一个Client 实例
    let client = Client::new();

    // 读取文件
    let mut content = String::new();
    let mut file = File::open("/Users/zengyr7/Desktop/MideaFiles/日常任务/20241209年终总结/2024年md任务汇总.txt").unwrap();
    file.read_to_string(&mut content);

    println!("{}", content.substring(0, 100));

    let mut prompt = format!("下面的上下文是我2024年每日记录的工作内容，请帮我提炼出工作内容中，最有价值的前10件事情，并且帮我做下总结和给出价值体现。请使用简体中文回答
    上下文:
    \"\"\"
    {}
    \"\"\"
    ", content);

    let mut body = ChatBody {
        model: "llama3.1".to_string(),
        prompt: prompt,
        stream: true,
        options: Options {temperature: 0.8}, // temperature的默认值，0.8,值越大，回答越creative
        images: None,
    };

    // 发起请求，且以stream返回
    let mut stream_res = send_and_return_stream(body, client).await;
}

#[cfg(test)]
mod tests {
    use crate::demo_read_file_and_chat::read_file_and_chat;

    #[tokio::test]
    async fn test() {
        read_file_and_chat().await;
    }
}