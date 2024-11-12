use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize)]
struct ChatBody {
    model: String,
    prompt: String,
    stream: bool,
    options: Options
}

#[derive(Serialize)]
struct Options {
    temperature: f64,
}

#[derive(Deserialize)]
struct ChatResponse {
    model: String,
    create_at: Option<String>,
    response: Option<String>,
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



#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    println!("Hello, world!");
    // 创建一个Client 实例
    let client = Client::new();

    // 从oss拉取文件，先尝试用http链接，后续TODO：用s3的方式；
    // let file_path_url = "https://oss-cn-foshan-2.biubiu.com/service-log/%E9%97%AE%E9%A2%98%E7%AD%94%E7%96%91.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=yt8fbg0lugdrtrw5wbyfmfyl%2F20241112%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20241112T022436Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&response-content-type=application%2Foctet-stream&X-Amz-Signature=c6eff8f8effb8fc1564e2d5b93bde453f65aa66b7824889b7a7ff4cd47c4d082";
    let file_path_url = "your_oss_or_other_remote_file_path";
    let result = reqwest::get(file_path_url).await;
    let mut content = "".to_string();
    match result {
        Ok(response) => {
            if response.status().is_success() {
                content = response.text().await.unwrap();
            } else {
                println!("error: {}", response.status());
                assert!(false, "error: {}", response.status());
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    let mut prompt = format!("根据上下文回答问题，请用简体中文简洁地回答。
    上下文:
    \"\"\"
    {}
    \"\"\"
    问题: {}
    ", content, "mgp-client 2.x版本读取discovery配置出错，可能的原因是什么");

    // "你的老板经营一家羽毛球馆，你是这家羽毛球馆的智能AI。球馆有7片场地，每天闲时0-17点场地费用50一小时，忙时18-24点费用60一小时。其他经营业务有：羽毛球教学、相关器具出售、羽毛球比赛承办。如果有不懂的地方请联系老板，老板的电话是1234567，姓牛。接下来的提问在我没有具体声明之前，请按照这个背景给出答案。如果有不确定的点，请避免给出一个模糊的或者编造的答案，比如你需要避免编造课程价格和场地费用价格或者器械价格，同时因为你是个智能AI，因此你也无法自行联系场馆，如有需要请告知提问者联系前述的场馆老板牛先生。".to_string()

    let mut body = ChatBody {
        model: "llama3.1".to_string(),
        prompt: prompt,
        stream: false,
        options: Options {temperature: 0.8} // temperature的默认值，0.8,值越大，回答越creative
    };

    // 发起请求
    let mut res = client.post("http://localhost:11434/api/generate")
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await?;
    println!("{:?}", res);
    let res_json = res.json::<ChatResponse>().await?;
    println!("{:?}", res_json.response);

    // 发起请求，且以stream返回
    body.stream = true;
    res = client.post("http://localhost:11434/api/generate")
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await?;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item?;
        // 拼装和返回
        let temp_res:Result<ChatResponse, serde_json::Error> = serde_json::from_slice(&item);
        match temp_res {
            Ok(chatRes) => {
                print!("{}", chatRes.response.unwrap());
            }
            Err(e) => {
            }
        }
        
        // print!("{}", temp_res.response.unwrap());
    }

    Ok(())
}