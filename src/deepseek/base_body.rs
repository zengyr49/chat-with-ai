use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::ops::Add;

//////////// request ///////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub frequency_penalty: i32,
    pub max_tokens: i32,
    pub presence_penalty: i32,
    pub response_format: ResponseFormat,
    pub stop: Option<String>,
    pub stream: bool,
    pub stream_options: Option<String>,
    pub temperature: i32,
    pub top_p: i32,
    pub tools: Option<String>,
    pub tool_choice: String,
    pub logprobs: bool,
    pub top_logprobs: Option<String>,
}
////////////////////

///////// response //////////
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    pub system_fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: i32,
    pub message: ResMessage,
    pub logprobs: Option<serde_json::Value>, // 使用 Option 表示可能为 null
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub prompt_tokens_details: PromptTokensDetails,
    pub prompt_cache_hit_tokens: i32,
    pub prompt_cache_miss_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptTokensDetails {
    pub cached_tokens: i32,
}
////////////////////////////

const DEEPSEEK_API_KEY : &str = "sk-xxx";

///
/// send request to deepseek and get return
///
pub async fn send_and_return_all(client:Client, body:ChatRequest) -> String {
    let auth_token = &("Bearer ".to_string().add(DEEPSEEK_API_KEY));
    println!("{:?}", auth_token);

    let mut res = client.post("https://api.deepseek.com/chat/completions")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", auth_token)
        .json(&body)
        .send().await.unwrap();
    if !res.status().is_success() {
        return res.text().await.unwrap();
    }

    println!("{:?}", res);
    let res_json = res.json::<ChatResponse>().await.unwrap();
    println!("{:?}", res_json);
    let mut res_str = "".to_string();
    let choices = res_json.choices;
    if !choices.is_empty() {
        for choice in choices {
            let output = choice.message.content;
            res_str.push_str(&output);
            println!("{}", &output);
        }
    }

    res_str
}

#[cfg(test)]
mod test {
    use crate::deepseek::base_body::{send_and_return_all, ChatRequest, Message, ResponseFormat};
    use reqwest::Client;

    #[tokio::test]
    async fn test_chat_deepseek() {
        let messages_sys = Message {
            content: "你是个得力助手".to_string(),
            role: "system".to_string(),
        };
        let messages_user = Message {
            content: "你好".to_string(),
            role: "user".to_string(),
        };

        let request = ChatRequest {
            messages: vec![messages_sys, messages_user],
            model: "deepseek-chat".to_string(),
            frequency_penalty: 0,
            max_tokens: 2048,
            presence_penalty: 0,
            response_format: ResponseFormat { type_: "text".to_string() },
            stop: None,
            stream: false,
            stream_options: None,
            temperature: 1,
            top_p: 1,
            tools: None,
            tool_choice: "none".to_string(),
            logprobs: false,
            top_logprobs: None,
        };
        let client = Client::new();
        let res = send_and_return_all(client, request).await;
    }
}