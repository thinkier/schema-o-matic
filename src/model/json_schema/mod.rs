use serde_derive::{Deserialize, Serialize};
use crate::model::json_schema::array::JsonSchemaArray;
use crate::model::json_schema::object::JsonSchemaObject;
use crate::model::json_schema::scalar::JsonSchemaScalar;

pub mod array;
pub mod object;
pub mod scalar;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JsonSchema {
    String(JsonSchemaScalar),
    Number(JsonSchemaScalar),
    Boolean(JsonSchemaScalar),
    Object(JsonSchemaObject),
    Array(JsonSchemaArray),
    Null(JsonSchemaScalar),
}
