use std::collections::HashMap;
use serde_json::{Value, Map};
use crate::model::json_schema::JsonSchema;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonSchemaObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, JsonSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
