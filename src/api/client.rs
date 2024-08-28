use reqwest::Client as ReqwestClient;
use crate::api::list::list;
use crate::structs::partialmodel::PartialModel;

#[cfg(feature = "logging")]
use crate::logging::init_logger;
#[cfg(feature = "logging")]
use log::info;

/// A client for interacting with the API either synchronously or asynchronously.
#[derive(Clone)]
pub struct Ollama {
    host: String,
    port: Option<u16>,
    client: ReqwestClient,
}

impl Ollama {
    /// Creates a new API client with the default host and port.
    /// Defaults to "http://localhost" and port 11434 if not specified.
    pub fn new() -> Self {
        #[cfg(feature = "logging")]
        init_logger();

        #[cfg(feature = "logging")]
        info!("Creating new API client with default values.");

        Self {
            host: "http://localhost".to_string(),
            port: Some(11434),
            client: ReqwestClient::new(),
        }
    }

    /// Sets a custom host for the API client.
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Sets a custom port for the API client.
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Calculates the base URL based on the host and port.
    pub fn base_url(&self) -> String {
        match self.port {
            Some(port) => format!("{}:{}", self.host, port),
            None => self.host.clone(),
        }
    }

    /// Returns a reference to the `reqwest::Client` used for making requests.
    pub fn client(&self) -> &ReqwestClient {
        &self.client
    }

    /// Lists partial models from the API using the appropriate list function.
    #[cfg(not(feature = "async"))]
    pub fn list(&self) -> Result<Vec<PartialModel>, Box<dyn std::error::Error>> {
        list(Some(self))
    }

    /// Lists partial models from the API asynchronously using the appropriate list function.
    #[cfg(feature = "async")]
    pub async fn list(&self) -> Result<Vec<PartialModel>, Box<dyn std::error::Error>> {
        list(Some(self)).await
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Ollama::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_ENDPOINT_HOST;
    use crate::constants::TEST_ENDPOINT_PORT;

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_ollama_sync_list() {
        let ollama = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let result = ollama.list();

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_ollama_async_list() {
        let ollama = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let result = ollama.list().await;

        match result {
            Ok(models) => assert!(!models.is_empty(), "Model list should not be empty"),
            Err(e) => panic!("Failed to fetch models: {}", e),
        }
    }
}
