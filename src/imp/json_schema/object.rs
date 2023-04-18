use std::collections::HashMap;
use serde_json::{Map, Value};
use crate::imp::json_schema::{ClearDefault, Union};
use crate::model::json_schema::JsonSchema;
use crate::model::json_schema::object::JsonSchemaObject;

impl JsonSchemaObject {
    pub fn from_object(object: &Map<String, Value>) -> Self {
        JsonSchemaObject {
            default: Some(object.to_owned()),
            properties: Some(object.iter()
                .map(|(k, v)| (k.to_owned(), JsonSchema::from_value(v)))
                .collect()),
            ..Default::default()
        }
    }
}

impl Union<JsonSchemaObject> for JsonSchemaObject {
    fn union(&self, rhs: &JsonSchemaObject) -> Self {
        let mut schema = self.clone();
        schema.properties = Some(HashMap::new());
        let empty_map = HashMap::new();

        let mut keys = vec![];

        let lp = if let Some(lp) = &self.properties {
            keys.extend(lp.keys());
            lp
        } else {
            &empty_map
        };

        let rp = if let Some(rp) = &rhs.properties {
            keys.extend(rp.keys());
            rp
        } else {
            &empty_map
        };

        keys.sort();
        keys.dedup();

        for key in keys {
            let i = match (lp.get(key), rp.get(key)) {
                (Some(li), Some(ri)) => li.union(ri),
                (Some(li), None) => li.to_owned(),
                (None, Some(ri)) => ri.to_owned(),
                (None, None) => unreachable!("keys are a union set of lp and rp, this shouldn't happen")
            };

            schema.properties.as_mut().unwrap().insert(key.to_owned(), i);
        }

        if self.default.is_none() || rhs.default.is_none() {
            schema.default = None;
        }

        return schema;
    }
}

impl ClearDefault for JsonSchemaObject {
    fn clear_default(&mut self) {
        self.default = None;
        if let Some(properties) = &mut self.properties {
            properties.values_mut().for_each(|v| v.clear_default());
        }
    }
}
