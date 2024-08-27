use reqwest::blocking::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub description: String,
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
/// let base_url = "http://0.0.0.0:11434";
/// let result = list_models(&client, base_url);
/// assert!(result.is_ok());
/// ```
pub fn list_models(client: &Client, base_url: &str) -> Result<Vec<Model>, Error> {
    let url = format!("{}/api/tags", base_url);
    let response = client.get(&url).send()?;  // Synchronously send the request
    let models = response.json::<Vec<Model>>()?;  // Synchronously parse the JSON
    Ok(models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::blocking::Client;

    #[test]
    fn test_list_models_sync() {
        let client = Client::new();
        let base_url = "http://0.0.0.0:11434";

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
