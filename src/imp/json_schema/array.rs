use serde_json::Value;
use crate::imp::json_schema::{ClearDefault, Union};
use crate::model::json_schema::array::JsonSchemaArray;
use crate::model::json_schema::JsonSchema;

impl JsonSchemaArray {
    pub fn from_array(array: &Vec<Value>) -> Self {
        let mut items = array.iter()
            .map(|v| JsonSchema::from_value(v))
            .collect::<Vec<_>>();


        JsonSchemaArray {
            default: Some(array.to_owned()),
            items: Some(Box::new(items.pop().unwrap())),
            ..Default::default()
        }
    }
}

impl Union<JsonSchemaArray> for JsonSchemaArray {
    fn union(&self, rhs: &JsonSchemaArray) -> Self {
        let mut schema = self.clone();

        if let (Some(li), Some(ri)) = (&self.items, &rhs.items) {
            schema.items = Some(Box::new(li.union(&ri)));
        } else if let Some(ri) = &rhs.items {
            schema.items = Some(ri.to_owned());
        }

        return schema;
    }
}

impl ClearDefault for JsonSchemaArray {
    fn clear_default(&mut self) {
        self.default = None;
        if let Some(items) = &mut self.items {
            items.clear_default()
        }
    }
}