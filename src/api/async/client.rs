// ./src/api/async/client.rs
use reqwest::Client as ReqwestClient;

#[derive(Clone)]
pub struct Client {
    base_url: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: ReqwestClient::new(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn client(&self) -> &ReqwestClient {
        &self.client
    }
}
