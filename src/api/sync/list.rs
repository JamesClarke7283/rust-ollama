use crate::constants::API_TAGS_ENDPOINT;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::api::sync::client::Client;
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

/// Lists models from the API synchronously.
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
///
/// let client = Client::new("http://0.0.0.0:11434"); // Use your actual API base URL here
/// let result = list(&client);
/// assert!(result.is_ok());
/// ```
pub fn list(client: &Client) -> Result<Vec<Model>, Box<dyn Error>> {
    let url = format!("{}{}", client.base_url(), API_TAGS_ENDPOINT);

    #[cfg(feature = "logging")]
    info!("Sending synchronous request to URL: {}", url);

    let response = client.client().get(&url).send()?;  // Synchronously send the request

    let raw_body = response.text()?;
    
    #[cfg(feature = "logging")]
    info!("Received response: {}", raw_body);

    let models_response: ModelsResponse = serde_json::from_str(&raw_body)?;
    Ok(models_response.models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_ENDPOINT;

    #[test]
    fn test_list_sync() {
        let client = Client::new(TEST_ENDPOINT);

        match list(&client) {
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
