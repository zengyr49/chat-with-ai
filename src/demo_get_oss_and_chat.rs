use reqwest::Client;
use substring::Substring;
use crate::base_body::{send_and_return_stream, ChatBody, Options};
use crate::{oss};
use crate::oss::OssConfig;

pub async fn get_oss_and_chat() {
    println!("Hello, world!");
    // 创建一个Client 实例
    let client = Client::new();

    // 从oss拉取文件,S3的方式
    let oss_config = OssConfig {
        access_key: "".to_string(),
        secret_key: "".to_string(),
        suffix: "oss-cn-foshan-2.company.com".to_string()
    };
    let object_name = "问题答疑.txt";
    let oss_client = oss::oss_client(oss_config).unwrap();
    let content = oss::cat_object(&oss_client, "service-log", object_name).await.unwrap();
    println!("{}", content.substring(0, 100));

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
        stream: true,
        options: Options {temperature: 0.8}, // temperature的默认值，0.8,值越大，回答越creative
        images: None,
    };

    // 发起请求，且以stream返回
    let mut stream_res = send_and_return_stream(body, client).await;
}

#[cfg(test)]
mod tests {
    use crate::demo_get_oss_and_chat::get_oss_and_chat;

    #[tokio::test]
    async fn test() {
        get_oss_and_chat().await;
    }
}