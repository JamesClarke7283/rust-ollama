use serde::{Deserialize, Serialize};
use serde_json::{self, Result as JsonResult};
use crate::api::show::ShowResponse;

/// Represents the details of a model, including metadata such as format, family, and size.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ModelDetails {
    pub parent_model: Option<String>,
    pub format: Option<String>,
    pub family: Option<String>,
    pub families: Option<Vec<String>>,
    pub parameter_size: Option<String>,
    pub quantization_level: Option<String>,
}

/// Represents a model returned by the API, including its metadata and associated details.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Model {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub parameters: Option<String>,
    pub template: Option<String>,
    pub details: ModelDetails,
    pub model_info: Option<serde_json::Value>,
}

impl Model {
    /// Creates a new instance of `Model` from a `ShowResponse`.
    pub fn from_show_response(response: ShowResponse) -> Self {
        Model {
            name: response.details.family.clone().unwrap_or_default(),
            model: response.details.family.clone().unwrap_or_default(),
            modified_at: response.details.parameter_size.clone().unwrap_or_default(),
            size: response.details.parameter_size.clone().unwrap_or_default().parse().unwrap_or(0),
            digest: response.details.quantization_level.clone().unwrap_or_default(),
            parameters: Some(response.parameters),
            template: Some(response.template),
            details: response.details,
            model_info: response.model_info,
        }
    }

    /// Serializes the `Model` instance to a JSON string.
    pub fn json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
