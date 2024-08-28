use crate::constants::API_TAGS_ENDPOINT;
use std::error::Error;
use crate::api::client::Ollama;
use crate::structs::partialmodel::PartialModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelsResponse {
    pub models: Vec<PartialModel>,
}

/// Lists models from the API synchronously.
///
/// # Arguments
///
/// * `client` - An optional reference to the `Ollama` struct. If `None`, uses the default host and port.
///
/// # Returns
///
/// A result containing a vector of `PartialModel` instances or an error.
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
pub fn list(client: Option<&Ollama>) -> Result<Vec<PartialModel>, Box<dyn std::error::Error>> {
    use reqwest::blocking::Client as BlockingClient;

    let url = match client {
        Some(client) => format!("{}{}", client.base_url(), API_TAGS_ENDPOINT),
        None => format!("http://0.0.0.0:11434{}", API_TAGS_ENDPOINT),
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
/// * `client` - An optional reference to the `Ollama` struct. If `None`, uses the default host and port.
///
/// # Returns
///
/// A result containing a vector of `PartialModel` instances or an error.
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
pub async fn list(client: Option<&Ollama>) -> Result<Vec<PartialModel>, Box<dyn std::error::Error>> {
    let url = match client {
        Some(client) => format!("{}{}", client.base_url(), API_TAGS_ENDPOINT),
        None => format!("http://0.0.0.0:11434{}", API_TAGS_ENDPOINT),
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
    use crate::constants::TEST_ENDPOINT_HOST;
    use crate::constants::TEST_ENDPOINT_PORT;

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_list_sync_with_client() {
        let client = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let result = list(Some(&client));

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_list_sync_without_client() {
        let result = list(None);

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_list_async_with_client() {
        let client = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let result = list(Some(&client)).await;

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_list_async_without_client() {
        let result = list(None).await;

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }
}
