use crate::constants::SHOW_ENDPOINT;
use crate::structs::model::ModelDetails;
use crate::api::client::Ollama;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Struct representing the request body for the `show` API call.
///
/// The `name` field is mandatory, and `verbose` is optional.
/// If `verbose` is set to `Some(true)`, the API will return a more detailed response.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ShowRequest {
    pub name: String,
    pub verbose: Option<bool>,
}

/// Struct representing the response from the `show` API call.
///
/// The response includes details like the modelfile, parameters, template, and model details.
/// The `model_info` field may contain additional model-specific metadata if requested with `verbose: true`.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ShowResponse {
    pub modelfile: String,
    pub parameters: String,
    pub template: String,
    pub details: ModelDetails,
    pub model_info: Option<serde_json::Value>,
}

/// Synchronously sends a request to the `show` endpoint to retrieve detailed information about a model.
///
/// # Arguments
///
/// * `client` - An optional reference to a `Ollama` instance.
/// * `name` - The name of the model to retrieve information about.
/// * `verbose` - An optional boolean flag to request more detailed information.
///
/// # Returns
///
/// A `Result` containing a `ShowResponse` with the model details, or an error if the request fails.
///
/// # Examples
///
/// ```
/// use ollama::prelude::*;
///
/// let ollama = Ollama::new().with_host("http://0.0.0.0").with_port(11434);
/// let response = show(Some(&ollama), "llama3.1:8b-instruct-q6_K", Some(true)).unwrap();
/// assert!(response.modelfile.contains("llama3.1"));
/// ```
///
/// # Errors
///
/// This function returns an error if the HTTP request fails or if the response cannot be deserialized.
#[cfg(not(feature = "async"))]
pub fn show(client: Option<&Ollama>, name: &str, verbose: Option<bool>) -> Result<ShowResponse, Box<dyn Error>> {
    use reqwest::blocking::Client as BlockingClient;

    let url = match client {
        Some(client) => format!("{}{}", client.base_url(), SHOW_ENDPOINT),
        None => format!("{}{}", crate::constants::TEST_ENDPOINT_HOST, SHOW_ENDPOINT),
    };

    #[cfg(feature = "logging")]
    log::info!("Sending synchronous request to URL: {}", url);

    let request_body = ShowRequest {
        name: name.to_string(),
        verbose,
    };

    let response = BlockingClient::new()
        .post(&url)
        .json(&request_body)
        .send()?
        .error_for_status()?;

    let raw_body = response.text()?;

    #[cfg(feature = "logging")]
    log::info!("Received response: {}", raw_body);

    let show_response: ShowResponse = serde_json::from_str(&raw_body)?;
    Ok(show_response)
}

/// Asynchronously sends a request to the `show` endpoint to retrieve detailed information about a model.
///
/// # Arguments
///
/// * `client` - An optional reference to a `Ollama` instance.
/// * `name` - The name of the model to retrieve information about.
/// * `verbose` - An optional boolean flag to request more detailed information.
///
/// # Returns
///
/// A `Result` containing a `ShowResponse` with the model details, or an error if the request fails.
///
/// # Examples
///
/// ```
/// use ollama::prelude::*;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let ollama = Ollama::new().with_host("http://0.0.0.0").with_port(11434);
///     let response = show(Some(&ollama), "llama3.1:8b-instruct-q6_K", Some(true)).await.unwrap();
///     assert!(response.modelfile.contains("llama3.1"));
/// }
/// ```
///
/// # Errors
///
/// This function returns an error if the HTTP request fails or if the response cannot be deserialized.
#[cfg(feature = "async")]
pub async fn show(client: Option<&Ollama>, name: &str, verbose: Option<bool>) -> Result<ShowResponse, Box<dyn Error>> {
    let url = match client {
        Some(client) => format!("{}{}", client.base_url(), SHOW_ENDPOINT),
        None => format!("{}:{}{}", crate::constants::TEST_ENDPOINT_HOST, crate::constants::TEST_ENDPOINT_PORT, SHOW_ENDPOINT),
    };

    #[cfg(feature = "logging")]
    log::info!("Sending asynchronous request to URL: {}", url);

    let request_body = ShowRequest {
        name: name.to_string(),
        verbose,
    };

    let response = reqwest::Client::new()
        .post(&url)
        .json(&request_body)
        .send()
        .await?
        .error_for_status()?;

    let raw_body = response.text().await?;

    #[cfg(feature = "logging")]
    log::info!("Received response: {}", raw_body);

    let show_response: ShowResponse = serde_json::from_str(&raw_body)?;
    Ok(show_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_ENDPOINT_HOST;
    use crate::constants::TEST_ENDPOINT_PORT;

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_show_sync_with_client() {
        let client = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let result = show(Some(&client), "llama3.1:8b-instruct-q6_K", Some(true));

        match result {
            Ok(response) => {
                assert!(response.modelfile.contains("llama3.1"));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                assert!(false, "Failed to fetch model details");
            }
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_show_async_with_client() {
        let client = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let result = show(Some(&client), "llama3.1:8b-instruct-q6_K", Some(true)).await;

        match result {
            Ok(response) => {
                assert!(response.modelfile.contains("llama3.1"));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                assert!(false, "Failed to fetch model details");
            }
        }
    }
}