use serde_json::Value;
use crate::imp::json_schema::{ClearDefault, Union};
use crate::model::json_schema::array::JsonSchemaArray;
use crate::model::json_schema::JsonSchema;

impl JsonSchemaArray {
    pub fn from_array(array: &Vec<Value>) -> Self {
        let items = array.iter()
            .map(|v| JsonSchema::from_value(v))
            .collect::<Vec<_>>();

        let items_schema = if items.len() > 0 {
            let mut schema = items[0].clone();
            for i in 1..items.len() {
                schema = schema.union(&items[i]);
            }
            Some(Box::new(schema))
        } else {
            None
        };

        JsonSchemaArray {
            default: Some(array.to_owned()),
            items: items_schema,
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
