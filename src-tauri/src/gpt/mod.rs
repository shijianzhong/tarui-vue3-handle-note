/*
 * @Date: 2023-02-17 08:10:02
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-21 10:08:54
 * @FilePath: /vue-project/src-tauri/src/gpt/mod.rs
 */

pub mod openai {
    use hyper::{body::Buf, header, Body, Client, Request};
    use hyper_tls::HttpsConnector;
    use std::{env};
    use crate::models::open_ai_mod::{OpenAIResponse,OpenAIRequest};
    pub async fn qs(user_input:String) -> Result<OpenAIResponse, Box<dyn std::error::Error>> {
        let api_key = match env::var("OPENAI_KEY") {
            Ok(key) => key,
            Err(e) => {
                println!("{:#?}", e);
                println!("Error: please create an environment variable OPENAI_KEY");
                std::process::exit(1);
            }
        };
        let uri = "https://api.openai.com/v1/completions";
        let https = HttpsConnector::new();
        let client = Client::builder().build(https);
        let model = String::from("text-davinci-003");
        let stop = String::from("Text");
        println!("{}", api_key);
        let auth_header_val = format!("Bearer {}", api_key);
        // let default_prompt =
        //     "Given text, return 1 bash command. Text:list contents of a directory. Command:ls";
        let before_qs:String = String::from("");
        let openai_request = OpenAIRequest {
            model,
            prompt: format!("Text:{}. Human:", user_input),
            temperature:0.9,
            max_tokens: 2500,
            stop,
            top_p:1,
            frequency_penalty:0.0,
            presence_penalty:0.6
        };
        println!("执行我了&*（");
        let body = Body::from(serde_json::to_vec(&openai_request)?);

        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();

        let res = client.request(req).await?;

        let body = hyper::body::aggregate(res).await?;

        match serde_json::from_reader(body.reader()) {
            Ok(response) => Ok(response),
            Err(_) => {
                println!("Error");
                std::process::exit(1);
            }
        }
    }
}
