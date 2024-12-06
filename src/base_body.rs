use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatBody {
    pub(crate) model: String,
    pub(crate) prompt: String,
    pub(crate) stream: bool,
    pub(crate) options: Options,
    pub(crate) images: Option<Vec<String>>
}

#[derive(Serialize)]
pub struct Options {
    pub(crate) temperature: f64,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    model: String,
    create_at: Option<String>,
    pub(crate) response: Option<String>,
    done: bool,
    done_reason: Option<String>,
    context: Option<Vec<i64>>,
    total_duration: Option<i64>,
    load_duration: Option<i64>,
    prompt_eval_count: Option<i64>,
    prompt_eval_duration: Option<i64>,
    eval_count: Option<i64>,
    eval_duration: Option<i64>,
}

///
/// 发起请求,并返回全部结果
///
pub async fn send_and_return_all(client: Client, body: ChatBody) {
    // 发起请求,并返回全部结果
    let mut res = client.post("http://localhost:11434/api/generate")
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await.unwrap();
    println!("{:?}", res);
    let res_json = res.json::<ChatResponse>().await.unwrap();
    println!("{:?}", res_json.response);
}

///
/// 发起请求，且以stream返回
///
pub async fn send_and_return_stream(mut body: ChatBody, client: Client) {
    // 发起请求，且以stream返回
    body.stream = true;
    let mut res = client.post("http://localhost:11434/api/generate")
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await.unwrap();
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item.unwrap();
        // 拼装和返回
        let temp_res:Result<ChatResponse, serde_json::Error> = serde_json::from_slice(&item);
        match temp_res {
            Ok(chatRes) => {
                print!("{}", chatRes.response.unwrap());
            }
            Err(e) => {
            }
        }
    }
}