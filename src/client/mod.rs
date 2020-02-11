use reqwest::*;

pub async fn sci_get(url: &str, token: &str) -> Result<Response> {
    Client::builder()
        .build()
        .unwrap()
        .get(url)
        .bearer_auth(token)
        .send()
        .await
}
