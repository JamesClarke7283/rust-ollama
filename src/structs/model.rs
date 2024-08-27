use serde::{Deserialize, Serialize};
use serde_json::{self, Result as JsonResult};

/// Represents the details of a model, including metadata such as format, family, and size.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ModelDetails {
    /// The name of the parent model, if applicable.
    pub parent_model: Option<String>,
    
    /// The format of the model, e.g., "gguf".
    pub format: Option<String>,
    
    /// The primary family of the model, e.g., "starcoder2".
    pub family: Option<String>,
    
    /// A list of families the model belongs to, e.g., ["starcoder2"].
    pub families: Option<Vec<String>>,
    
    /// The size of the model's parameters, e.g., "3B".
    pub parameter_size: Option<String>,
    
    /// The level of quantization applied to the model, e.g., "Q6_K".
    pub quantization_level: Option<String>,
}

impl ModelDetails {
    /// Creates a new instance of `ModelDetails`.
    pub fn new(
        parent_model: Option<String>,
        format: Option<String>,
        family: Option<String>,
        families: Option<Vec<String>>,
        parameter_size: Option<String>,
        quantization_level: Option<String>,
    ) -> Self {
        ModelDetails {
            parent_model,
            format,
            family,
            families,
            parameter_size,
            quantization_level,
        }
    }
}

/// Represents a model returned by the API, including its metadata and associated details.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Model {
    /// The name of the model, e.g., "starcoder2:3b-q6_K".
    pub name: String,
    
    /// The identifier of the model, e.g., "starcoder2:3b-q6_K".
    pub model: String,
    
    /// The timestamp when the model was last modified, e.g., "2024-08-27T12:02:13.295534973+01:00".
    pub modified_at: String,
    
    /// The size of the model in bytes.
    pub size: u64,
    
    /// The digest of the model file, used for integrity checking.
    pub digest: String,
    
    /// Additional details about the model.
    pub details: ModelDetails,
}

impl Model {
    /// Creates a new instance of `Model`.
    pub fn new(
        name: String,
        model: String,
        modified_at: String,
        size: u64,
        digest: String,
        details: ModelDetails,
    ) -> Self {
        Model {
            name,
            model,
            modified_at,
            size,
            digest,
            details,
        }
    }

    /// Serializes the `Model` instance to a JSON string.
    pub fn json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_creation() {
        let details = ModelDetails::new(
            Some("".to_string()),
            Some("gguf".to_string()),
            Some("starcoder2".to_string()),
            Some(vec!["starcoder2".to_string()]),
            Some("3B".to_string()),
            Some("Q6_K".to_string()),
        );

        let model = Model::new(
            "starcoder2:3b-q6_K".to_string(),
            "starcoder2:3b-q6_K".to_string(),
            "2024-08-27T12:02:13.295534973+01:00".to_string(),
            2490902249,
            "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2".to_string(),
            details,
        );

        assert_eq!(model.name, "starcoder2:3b-q6_K");
        assert_eq!(model.model, "starcoder2:3b-q6_K");
        assert_eq!(model.modified_at, "2024-08-27T12:02:13.295534973+01:00");
        assert_eq!(model.size, 2490902249);
        assert_eq!(model.digest, "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2");
        assert_eq!(model.details.family, Some("starcoder2".to_string()));
    }

    #[test]
    fn test_model_serialization() {
        let details = ModelDetails::new(
            Some("".to_string()),
            Some("gguf".to_string()),
            Some("starcoder2".to_string()),
            Some(vec!["starcoder2".to_string()]),
            Some("3B".to_string()),
            Some("Q6_K".to_string()),
        );

        let model = Model::new(
            "starcoder2:3b-q6_K".to_string(),
            "starcoder2:3b-q6_K".to_string(),
            "2024-08-27T12:02:13.295534973+01:00".to_string(),
            2490902249,
            "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2".to_string(),
            details,
        );

        let json_output = model.json().unwrap();
        let expected_json = r#"{"name":"starcoder2:3b-q6_K","model":"starcoder2:3b-q6_K","modified_at":"2024-08-27T12:02:13.295534973+01:00","size":2490902249,"digest":"a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2","details":{"parent_model":"","format":"gguf","family":"starcoder2","families":["starcoder2"],"parameter_size":"3B","quantization_level":"Q6_K"}}"#;

        assert_eq!(json_output, expected_json);
    }

    #[test]
    fn test_model_deserialization() {
        let json_input = r#"{
            "name": "starcoder2:3b-q6_K",
            "model": "starcoder2:3b-q6_K",
            "modified_at": "2024-08-27T12:02:13.295534973+01:00",
            "size": 2490902249,
            "digest": "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2",
            "details": {
                "parent_model": "",
                "format": "gguf",
                "family": "starcoder2",
                "families": ["starcoder2"],
                "parameter_size": "3B",
                "quantization_level": "Q6_K"
            }
        }"#;

        let model: Model = serde_json::from_str(json_input).unwrap();

        assert_eq!(model.name, "starcoder2:3b-q6_K");
        assert_eq!(model.model, "starcoder2:3b-q6_K");
        assert_eq!(model.modified_at, "2024-08-27T12:02:13.295534973+01:00");
        assert_eq!(model.size, 2490902249);
        assert_eq!(model.digest, "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2");
        assert_eq!(model.details.family, Some("starcoder2".to_string()));
    }

    #[test]
    fn test_multiple_models_deserialization() {
        let json_input = r#"{
            "models": [
                {
                    "name": "starcoder2:3b-q6_K",
                    "model": "starcoder2:3b-q6_K",
                    "modified_at": "2024-08-27T12:02:13.295534973+01:00",
                    "size": 2490902249,
                    "digest": "a5864ede0c4971b7eb12c14b27069902e8bb32691d997a55ac71c4831cdd01e2",
                    "details": {
                        "parent_model": "",
                        "format": "gguf",
                        "family": "starcoder2",
                        "families": ["starcoder2"],
                        "parameter_size": "3B",
                        "quantization_level": "Q6_K"
                    }
                },
                {
                    "name": "mxbai-embed-large:latest",
                    "model": "mxbai-embed-large:latest",
                    "modified_at": "2024-08-27T12:00:12.689822377+01:00",
                    "size": 669615493,
                    "digest": "468836162de7f81e041c43663fedbbba921dcea9b9fefea135685a39b2d83dd8",
                    "details": {
                        "parent_model": "",
                        "format": "gguf",
                        "family": "bert",
                        "families": ["bert"],
                        "parameter_size": "334M",
                        "quantization_level": "F16"
                    }
                }
            ]
        }"#;

        #[derive(Deserialize)]
        struct Models {
            models: Vec<Model>,
        }

        let models: Models = serde_json::from_str(json_input).unwrap();

        assert_eq!(models.models.len(), 2);
        assert_eq!(models.models[0].name, "starcoder2:3b-q6_K");
        assert_eq!(models.models[1].name, "mxbai-embed-large:latest");
        assert_eq!(models.models[1].details.family, Some("bert".to_string()));
    }
}
