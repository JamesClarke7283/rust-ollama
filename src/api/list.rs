use crate::constants::API_TAGS_ENDPOINT;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::api::client::Client;

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
/// * `client` - An optional reference to the `Client` struct. If `None`, uses the default base URL.
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
/// let result = list(None);
/// assert!(result.is_ok());
/// ```
#[cfg(not(feature = "async"))]
pub fn list(client: Option<&Client>) -> Result<Vec<Model>, Box<dyn Error>> {
    use reqwest::blocking::Client as BlockingClient;

    let url = match client {
        Some(client) => format!("{}{}", client.base_url(), API_TAGS_ENDPOINT),
        None => format!("{}{}", crate::constants::TEST_ENDPOINT, API_TAGS_ENDPOINT),
    };

    #[cfg(feature = "logging")]
    log::info!("Sending synchronous request to URL: {}", url);

    let response = BlockingClient::new()
        .get(&url)
        .send()?
        .error_for_status()?;

    let raw_body = response.text()?;

    #[cfg(feature = "logging")]
    log::info!("Received response: {}", raw_body);

    let models_response: ModelsResponse = serde_json::from_str(&raw_body)?;
    Ok(models_response.models)
}

/// Lists models from the API asynchronously.
///
/// # Arguments
///
/// * `client` - An optional reference to the `Client` struct. If `None`, uses the default base URL.
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
///     let result = list(None).await;
///     assert!(result.is_ok());
/// }
/// ```
#[cfg(feature = "async")]
pub async fn list(client: Option<&Client>) -> Result<Vec<Model>, Box<dyn Error>> {
    let url = match client {
        Some(client) => format!("{}{}", client.base_url(), API_TAGS_ENDPOINT),
        None => format!("{}{}", crate::constants::TEST_ENDPOINT, API_TAGS_ENDPOINT),
    };

    #[cfg(feature = "logging")]
    log::info!("Sending asynchronous request to URL: {}", url);

    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .error_for_status()?;

    let raw_body = response.text().await?;

    #[cfg(feature = "logging")]
    log::info!("Received response: {}", raw_body);

    let models_response: ModelsResponse = serde_json::from_str(&raw_body)?;
    Ok(models_response.models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_ENDPOINT;

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_list_sync_with_client() {
        let client = Client::new(TEST_ENDPOINT);

        match list(Some(&client)) {
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

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_list_sync_without_client() {
        match list(None) {
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

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_list_async_with_client() {
        let client = Client::new(TEST_ENDPOINT);

        match list(Some(&client)).await {
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

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_list_async_without_client() {
        match list(None).await {
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
