use reqwest::*;

pub struct Client {
    token: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(token: &str) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        Client {
            token: token.to_string(),
            client,
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        self.client.get(url).bearer_auth(&self.token).send().await
    }
}
