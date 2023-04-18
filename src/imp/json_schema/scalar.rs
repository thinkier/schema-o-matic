use serde_json::Value;
use crate::imp::json_schema::Union;
use crate::model::json_schema::scalar::JsonSchemaScalar;

impl JsonSchemaScalar {
    pub fn from_value(value: &Value) -> Self {
        JsonSchemaScalar {
            default: Some(value.to_owned()),
            ..Default::default()
        }
    }
}

impl Union<JsonSchemaScalar> for JsonSchemaScalar {
    fn union(&self, rhs: &JsonSchemaScalar) -> Self {
        let mut schema = self.clone();

        if self.default.is_none() || rhs.default.is_none() {
            schema.default = None;
        }

        return schema;
    }
}
