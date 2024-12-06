use minio::s3::args::GetPresignedObjectUrlArgs;
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::error::Error;
use minio::s3::http::BaseUrl;

pub struct OssConfig {
    pub access_key: String,
    pub secret_key: String,
    pub suffix: String,
}

pub fn oss_client(oss_config: OssConfig) -> Result<Client, Error> {
    let mut base_url = format!("http://{}", oss_config.suffix).parse::<BaseUrl>()?;
    base_url.region = "region-one".to_string();
    let provider = StaticProvider::new(&oss_config.access_key, &oss_config.secret_key, None);
    let client = Client::new(base_url, Some(Box::new(provider)), None, None)?;
    Ok(client)
}

///
/// 将oss文件内容存到本地变量中
///
pub async fn cat_object(client: &Client, bucket_name:&str, object_name:&str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let encoded_name:String = url::form_urlencoded::byte_serialize(object_name.as_bytes()).collect();

    // 通过预签名获取url
    let args = GetPresignedObjectUrlArgs {
        extra_query_params: None,
        region: Some("region-one"),
        bucket: bucket_name,
        object: &encoded_name,
        version_id: None,
        method: Default::default(),
        expiry_seconds: Some(60 * 60 * 24 * 7),
        request_time: None
    };
    let response = client.get_presigned_object_url(&args).await.unwrap();
    println!("{}", response.url);
    // 根据url下载文件存到本地
    let result = reqwest::get(response.url).await;
    let mut res_content = "".to_string();
    match result {
        Ok(response) => {
            if response.status().is_success() {
                res_content = response.text().await.unwrap()
            } else {
                println!("error: {}", response.status());
                assert!(false, "error: {}", response.status())
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    Ok(res_content)
}

#[cfg(test)]
mod tests {
    use crate::oss::{cat_object, oss_client, OssConfig};
    use substring::Substring;

    #[tokio::test]
    pub async fn test() {
        // init oss client
        let oss_config = OssConfig {
            suffix: "".to_string(),
            access_key: "".to_string(),
            secret_key: "".to_string()
        };
        let client = oss_client(oss_config).unwrap();

        // cat object
        // let encoded_filename:String = url::form_urlencoded::byte_serialize("ttt.txt".as_bytes()).collect();
        let object_content = cat_object(&client, "service-log", "问题答疑.txt").await.unwrap();
        println!("{}", object_content.substring(0, 300));
    }
}