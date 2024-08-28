use serde::{Deserialize, Serialize};
use crate::api::client::Ollama;
use crate::structs::model::Model;
use std::error::Error;
use crate::prelude::show;

/// Represents a partial model returned by the `/api/tags` endpoint.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PartialModel {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
}

impl PartialModel {
    /// Converts a `PartialModel` to a full `Model` by calling the `show` API.
    ///
    /// # Arguments
    ///
    /// * `client` - An optional reference to a `Ollama` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Model` instance or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama::prelude::*;
    /// use tokio; // Import the Tokio runtime for the async test
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Ollama::new().with_host("http://0.0.0.0").with_port(11434);
    ///     let partial_model = PartialModel {
    ///         name: "llama3.1:8b-instruct-q6_K".to_string(),
    ///         model: "llama3.1:8b-instruct-q6_K".to_string(),
    ///         modified_at: "2024-08-26T13:02:58.883873254+01:00".to_string(),
    ///         size: 2490902249,
    ///         digest: "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2".to_string(),
    ///     };
    ///     let model = partial_model.to_model(Some(&client)).await.unwrap();
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn to_model(&self, client: Option<&Ollama>) -> Result<Model, Box<dyn Error>> {
        let response = show(client, &self.model, Some(true)).await?;
        let mut model = Model::from_show_response(response);
        model.name = self.name.clone();  // Ensure the name matches the PartialModel name
        Ok(model)
    }

    /// Converts a `PartialModel` to a full `Model` by calling the `show` API.
    ///
    /// # Arguments
    ///
    /// * `client` - An optional reference to a `Ollama` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Model` instance or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama::prelude::*;
    ///
    /// let client = Ollama::new().with_host("http://0.0.0.0").with_port(11434);
    /// let partial_model = PartialModel {
    ///     name: "llama3.1:8b-instruct-q6_K".to_string(),
    ///     model: "llama3.1:8b-instruct-q6_K".to_string(),
    ///     modified_at: "2024-08-26T13:02:58.883873254+01:00".to_string(),
    ///     size: 2490902249,
    ///     digest: "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2".to_string(),
    /// };
    /// let model = partial_model.to_model(Some(&client)).unwrap();
    /// ```
    #[cfg(not(feature = "async"))]
    pub fn to_model(&self, client: Option<&Ollama>) -> Result<Model, Box<dyn Error>> {
        let response = show(client, &self.model, Some(true))?;
        let mut model = Model::from_show_response(response);
        model.name = self.name.clone();  // Ensure the name matches the PartialModel name
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::client::Ollama;
    use crate::constants::TEST_ENDPOINT_HOST;
    use crate::constants::TEST_ENDPOINT_PORT;

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_to_model_sync() {
        let client = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let partial_model = PartialModel {
            name: "llama3.1:8b-instruct-q6_K".to_string(),
            model: "llama3.1:8b-instruct-q6_K".to_string(),
            modified_at: "2024-08-26T13:02:58.883873254+01:00".to_string(),
            size: 2490902249,
            digest: "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2".to_string(),
        };
        let model = partial_model.to_model(Some(&client)).unwrap();
        assert_eq!(model.name, partial_model.name);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_to_model_async() {
        let client = Ollama::new().with_host(TEST_ENDPOINT_HOST).with_port(TEST_ENDPOINT_PORT);
        let partial_model = PartialModel {
            name: "llama3.1:8b-instruct-q6_K".to_string(),
            model: "llama3.1:8b-instruct-q6_K".to_string(),
            modified_at: "2024-08-26T13:02:58.883873254+01:00".to_string(),
            size: 2490902249,
            digest: "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2".to_string(),
        };
        let model = partial_model.to_model(Some(&client)).await.unwrap();
        assert_eq!(model.name, partial_model.name);
    }
}
