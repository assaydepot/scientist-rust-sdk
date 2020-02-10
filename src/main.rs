#[macro_use]
extern crate serde;

#[derive(Deserialize, Debug)]
struct IP {
    ip: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let result = reqwest::get("http://ip.jsontest.com")
        .await
        .unwrap()
        .json::<IP>()
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
