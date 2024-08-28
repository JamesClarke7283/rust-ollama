use reqwest::Client as ReqwestClient;
use crate::api::list::list;
use crate::structs::partialmodel::PartialModel; // Adjusted import

#[cfg(feature = "logging")]
use crate::logging::init_logger;
#[cfg(feature = "logging")]
use log::info;

/// A client for interacting with the API either synchronously or asynchronously.
///
/// This client allows you to specify a base URL for the API and provides
/// a pre-configured `reqwest::Client` or `reqwest::blocking::Client` instance
/// to make HTTP requests depending on whether the `async` feature is enabled.
///
/// # Examples
///
/// ```
/// use ollama::prelude::*;
///
/// let client = Client::new("http://0.0.0.0:11434");
/// let base_url = client.base_url();
/// let reqwest_client = client.client();
/// ```
///
/// The example above creates a new `Client` for making either synchronous or asynchronous API calls.
#[derive(Clone)]
pub struct Client {
    base_url: String,
    client: ReqwestClient,
}

impl Client {
    /// Creates a new API client with the given base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama::prelude::*;
    ///
    /// let client = Client::new("http://0.0.0.0:11434");
    /// ```
    pub fn new(base_url: &str) -> Self {
        #[cfg(feature = "logging")]
        init_logger();

        #[cfg(feature = "logging")]
        info!("Creating new API client with base URL: {}", base_url);

        Self {
            base_url: base_url.to_string(),
            client: ReqwestClient::new(),
        }
    }

    /// Returns the base URL of the API.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama::prelude::*;
    ///
    /// let client = Client::new("http://0.0.0.0:11434");
    /// assert_eq!(client.base_url(), "http://0.0.0.0:11434");
    /// ```
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns a reference to the `reqwest::Client` or `reqwest::blocking::Client` used for making requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama::prelude::*;
    ///
    /// let client = Client::new("http://0.0.0.0:11434");
    /// let reqwest_client = client.client();
    /// ```
    pub fn client(&self) -> &ReqwestClient {
        &self.client
    }

    /// Lists partial models from the API using the appropriate list function.
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
    /// let client = Client::new("http://0.0.0.0:11434");
    /// let result = client.list();
    /// assert!(result.is_ok());
    /// ```
    #[cfg(not(feature = "async"))]
    pub fn list(&self) -> Result<Vec<PartialModel>, Box<dyn std::error::Error>> { // Updated return type
        list(Some(self))
    }

    /// Lists partial models from the API asynchronously using the appropriate list function.
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
    ///     let client = Client::new("http://0.0.0.0:11434");
    ///     let result = client.list().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn list(&self) -> Result<Vec<PartialModel>, Box<dyn std::error::Error>> { // Updated return type
        list(Some(self)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_ENDPOINT;

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_client_sync_list() {
        let client = Client::new(TEST_ENDPOINT);
        let result = client.list();

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_client_async_list() {
        let client = Client::new(TEST_ENDPOINT);
        let result = client.list().await;

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }
}
