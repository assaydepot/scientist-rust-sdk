#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let request = client.get("https://google.com");
    let result = request.send().await.unwrap();
    let body = result.bytes().await.unwrap();
    println!("{:?}", body);
}
