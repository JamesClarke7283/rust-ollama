use reqwest::Client as ReqwestClient;
#[cfg(feature = "logging")]
use crate::logging::init_logger;
#[cfg(feature = "logging")]
use log::info;

/// A client for interacting with the API asynchronously.
///
/// This client allows you to specify a base URL for the API and provides
/// a pre-configured `reqwest::Client` instance to make asynchronous
/// HTTP requests.
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
/// The example above creates a new `Client` for making asynchronous API calls.
#[derive(Clone)]
pub struct Client {
    base_url: String,
    client: ReqwestClient,
}

impl Client {
    /// Creates a new asynchronous API client with the given base URL.
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
        info!("Creating new asynchronous API client with base URL: {}", base_url);

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

    /// Returns a reference to the `reqwest::Client` used for making requests.
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
}
