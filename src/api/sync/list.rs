use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub description: String,
}

pub fn list_models_sync(client: &Client, base_url: &str) -> Result<Vec<Model>, Error> {
    let url = format!("{}/api/tags", base_url);
    let response = client.get(&url).send()?.json::<Vec<Model>>()?;
    Ok(response)
}
