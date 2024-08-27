use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub description: String,
}

pub async fn list_models_async(client: &Client, base_url: &str) -> Result<Vec<Model>, Error> {
    let url = format!("{}/api/tags", base_url);
    let response = client.get(&url).send().await?.json::<Vec<Model>>().await?;
    Ok(response)
}
