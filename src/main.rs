use reqwest::*;

use scientist_api_sdk::response::SciInfo;

pub async fn sci_get(url: &str, token: &str) -> Result<Response> {
    Client::builder()
        .build()
        .unwrap()
        .get(url)
        .bearer_auth(token)
        .send()
        .await
}

#[tokio::main]
async fn main() {
    let url = "https://app-staging.scientist.com/api/v1/info.json";
    let token = "axveH-GzuLws2D5m1MYV";
    let result = sci_get(url, token)
        .await
        .unwrap()
        .json::<SciInfo>()
        .await
        .unwrap();

    println!("{:#?}", result);
}

#[tokio::main]
async fn a_parallel_main() {
    let mut tasks = vec![];

    for _ in 1..10 {
        tasks.push(tokio::spawn(async {
            let client = reqwest::Client::new();
            let result = client.get("https://google.com").send().await.unwrap();
            let body = result.bytes().await.unwrap();
            println!("body: {:?}", body);
        }));
    }

    futures::future::join_all(tasks).await;
}
