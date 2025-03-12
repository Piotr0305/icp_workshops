use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use ic_cdk::api::management_canister::http_request::HttpHeader;
use ic_cdk::println;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    translation_text: String
}

#[ic_cdk::update]
async fn translate() -> String {
    let token = "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";
    let arg = CanisterHttpRequestArgument {
        url: "https://api-inference.huggingface.co/models/google-t5/t5-base".to_string(),
        max_response_bytes: None,
        method: HttpMethod::GET,
        headers: vec![
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {}", token).to_string(),
            }
        ],
        body: Some(r#"{"inputs": "hello"}"#.into()),
        transform: None,
    };
    let res = http_request(arg,1_700_000_000).await.unwrap();

    println!("{:?}", String::from_utf8(res.0.body.clone()));

    let formated_res: (Response,) = serde_json::from_slice(&res.0.body).unwrap();

    String::new()
}