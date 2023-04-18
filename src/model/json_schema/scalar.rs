use serde_json::Value;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonSchemaScalar<T = Value> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
