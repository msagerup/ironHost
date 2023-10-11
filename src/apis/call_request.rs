use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call Large Language Model.
pub async fn call_gtp(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API key info.
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in env.");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in env.");

    // Confirm endpoint.
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers.
    let mut headers: HeaderMap = HeaderMap::new();

    // Create api key header.
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Create org key header.
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Call client
    let client: Client = Client::builder()
    .default_headers(headers)
    .build()
    .unwrap();

    // Create chat completion 
    let chat_completion: ChatCompletion  = ChatCompletion {
        model: "gtp-4".to_string(),
        messages,
        temperature: 0.1
    };

    // Troubleshooting 
    let res_raw = client
    .post(url)
    .json(&chat_completion)
    .send()
    .await
    .unwrap();

    dbg!(res_raw.text().await.unwrap());


}
