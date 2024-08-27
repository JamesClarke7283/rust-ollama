use reqwest::Client;
use serde::{Deserialize, Serialize};

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

/// Lists models from the given base URL asynchronously.
///
/// # Arguments
///
/// * `client` - A reference to the reqwest Client.
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
/// use reqwest::Client;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new();
///     let base_url = "http://0.0.0.0:11434";
///     let result = list_models(&client, base_url).await;
///     assert!(result.is_ok());
/// }
/// ```
pub async fn list_models(client: &Client, base_url: &str) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    let url = format!("{}/api/tags", base_url);
    let response = client.get(&url).send().await?;

    // Print the raw response for debugging
    let raw_body = response.text().await?;
    println!("Raw response body: {}", raw_body);

    // Attempt to parse the JSON after printing the raw body
    let models_response: ModelsResponse = serde_json::from_str(&raw_body)?;
    Ok(models_response.models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_list_models_async() {
        let client = Client::new();
        let base_url = "http://0.0.0.0:11434";

        match list_models(&client, base_url).await {
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
