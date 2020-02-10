#[macro_use]
extern crate serde;

#[derive(Deserialize, Debug)]
struct SciInfo {
    version: String,
    message: Option<String>,
    api_version: String,
    environment: String,
}

#[tokio::main]
async fn main() {
    let url = "https://app-staging.scientist.com/api/v1/info.json";
    let result = reqwest::Client::builder()
        .build()
        .unwrap()
        .get(url)
        .bearer_auth("axveH-GzuLws2D5m1MYV")
        .send()
        .await
        .unwrap()
        .json::<SciInfo>()
        .await
        .unwrap();
    //    let body = result.bytes().await.unwrap();

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
