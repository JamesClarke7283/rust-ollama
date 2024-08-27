use crate::constants::API_TAGS_ENDPOINT;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::api::r#async::client::Client;
#[cfg(feature = "logging")]
use log::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelsResponse {
    pub models: Vec<Model>,
}

/// Lists models from the API asynchronously.
///
/// # Arguments
///
/// * `client` - A reference to the `Client` struct.
///
/// # Returns
///
/// A result containing a vector of `Model` instances or an error.
///
/// # Examples
///
/// ```
/// use ollama::prelude::*;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("http://0.0.0.0:11434"); // Use your actual API base URL here
///     let result = list(&client).await;
///     assert!(result.is_ok());
/// }
/// ```
pub async fn list(client: &Client) -> Result<Vec<Model>, Box<dyn Error>> {
    let url = format!("{}{}", client.base_url(), API_TAGS_ENDPOINT);

    #[cfg(feature = "logging")]
    info!("Sending asynchronous request to URL: {}", url);

    let response = client.client().get(&url).send().await?;

    let raw_body = response.text().await?;
    
    #[cfg(feature = "logging")]
    info!("Received response: {}", raw_body);

    let models_response: ModelsResponse = serde_json::from_str(&raw_body)?;
    Ok(models_response.models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_ENDPOINT;

    #[tokio::test]
    async fn test_list_async() {
        let client = Client::new(TEST_ENDPOINT);

        match list(&client).await {
            Ok(models) => {
                println!("Successfully retrieved models: {:?}", models);
                assert!(!models.is_empty(), "Model list should not be empty");
            }
            Err(e) => {
                eprintln!("Error fetching models: {}", e);
                assert!(false, "Failed to fetch models");
            }
        }
    }
}
