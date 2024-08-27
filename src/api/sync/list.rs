use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::constants::API_TAGS_ENDPOINT;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub description: Option<String>,
    // Add other fields as per the API response
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelsResponse {
    pub models: Vec<Model>,
}

/// Lists models from the given base URL synchronously.
///
/// # Arguments
///
/// * `client` - A reference to the reqwest blocking Client.
/// * `base_url` - The base URL of the API.
///
/// # Returns
///
/// A result containing a vector of `Model` instances or an `Error`.
///
/// # Example
///
/// ```
/// use ollama::prelude::*;
/// use reqwest::blocking::Client;
///
/// let client = Client::new();
/// let base_url = "http://0.0.0.0:11434"; // Use your actual API base URL here
/// let result = list_models(&client, base_url);
/// assert!(result.is_ok());
/// ```
pub fn list_models(client: &Client, base_url: &str) -> Result<Vec<Model>, Box<dyn Error>> {
    let url = format!("{}{}", base_url, API_TAGS_ENDPOINT);
    let response = client.get(&url).send()?;  // Synchronously send the request

    // Print the raw response for debugging
    let raw_body = response.text()?;
    println!("Raw response body: {}", raw_body);

    // Attempt to parse the JSON after printing the raw body
    let models_response: ModelsResponse = serde_json::from_str(&raw_body)?;
    Ok(models_response.models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::blocking::Client;
    use crate::constants::TEST_ENDPOINT;

    #[test]
    fn test_list_models_sync() {
        let client = Client::new();
        let base_url = TEST_ENDPOINT;

        match list_models(&client, base_url) {
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
