use scientist_api_sdk::response::SciInfo;
use scientist_api_sdk::Client;

#[tokio::main]
async fn main() {
    let url = "https://app-staging.scientist.com/api/v1/info.json";
    let token = "axveH-GzuLws2D5m1MYV";

    let client = Client::new(token);
    let get_res = client.get(url).await.unwrap();
    let sci_info = get_res.json::<SciInfo>().await.unwrap();

    println!("{:#?}", sci_info);
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
