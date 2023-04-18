use serde_json::Value;
use crate::model::json_schema::array::JsonSchemaArray;
use crate::model::json_schema::JsonSchema;
use crate::model::json_schema::object::JsonSchemaObject;
use crate::model::json_schema::scalar::JsonSchemaScalar;

pub mod array;
pub mod scalar;
pub mod object;

pub trait Union<U> {
    fn union(&self, rhs: &U) -> Self;
}

pub trait ClearDefault {
    fn clear_default(&mut self);
}

impl JsonSchema {
    pub fn is_scalar(&self) -> bool {
        match self {
            JsonSchema::String(_) => true,
            JsonSchema::Boolean(_) => true,
            JsonSchema::Number(_) => true,
            JsonSchema::Null(_) => true,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        if let JsonSchema::Array(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_object(&self) -> bool {
        if let JsonSchema::Object(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_scalar(&self) -> &JsonSchemaScalar {
        match self {
            JsonSchema::String(s) => s,
            JsonSchema::Boolean(b) => b,
            JsonSchema::Number(n) => n,
            JsonSchema::Null(n) => n,
            _ => panic!("cannot emit non-scalar as scalar")
        }
    }

    pub fn as_array(&self) -> &JsonSchemaArray {
        if let JsonSchema::Array(a) = self {
            a
        } else {
            panic!("cannot emit non-array as array")
        }
    }

    pub fn as_object(&self) -> &JsonSchemaObject {
        if let JsonSchema::Object(o) = self {
            o
        } else {
            panic!("cannot emit non-object as object")
        }
    }

    pub fn from_value(value: &Value) -> JsonSchema {
        if value.is_object() {
            JsonSchema::Object(JsonSchemaObject::from_object(value.as_object().unwrap()))
        } else if value.is_array() {
            JsonSchema::Array(JsonSchemaArray::from_array(value.as_array().unwrap()))
        } else {
            let scalar = JsonSchemaScalar::from_value(value);
            if value.is_number() {
                JsonSchema::Number(scalar)
            } else if value.is_string() {
                JsonSchema::String(scalar)
            } else if value.is_boolean() {
                JsonSchema::Boolean(scalar)
            } else {
                JsonSchema::Null(scalar)
            }
        }
    }
}

impl Union<JsonSchema> for JsonSchema {
    fn union(&self, rhs: &JsonSchema) -> Self {
        if self.is_scalar() && rhs.is_scalar() {
            let scalar = self.as_scalar().union(rhs.as_scalar());

            return match (self, rhs) {
                (JsonSchema::String(_), _) => JsonSchema::String(scalar),
                (_, JsonSchema::String(_)) => JsonSchema::String(scalar),
                (JsonSchema::Number(_), JsonSchema::Number(_)) => JsonSchema::Number(scalar),
                (JsonSchema::Boolean(_), JsonSchema::Boolean(_)) => JsonSchema::Boolean(scalar),
                (JsonSchema::Null(_), alt) => {
                    match alt {
                        JsonSchema::Boolean(_) => JsonSchema::Boolean(scalar),
                        JsonSchema::Number(_) => JsonSchema::Number(scalar),
                        JsonSchema::Null(_) => JsonSchema::Null(scalar),
                        JsonSchema::String(_) => unreachable!("all string is already covered in parent case"),
                        _ => unreachable!("no other categories are present under scalar")
                    }
                }
                _ => panic!("cannot combine lhs with rhs as scalar: {:?}, {:?}", self, rhs)
            };
        } else if self.is_array() && rhs.is_array() {
            let array = self.as_array().union(rhs.as_array());

            return JsonSchema::Array(array);
        } else if self.is_object() && rhs.is_object() {
            let object = self.as_object().union(rhs.as_object());

            return JsonSchema::Object(object);
        }

        panic!("cannot combine lhs with rhs: {:?}, {:?}", self, rhs)
    }
}

impl ClearDefault for JsonSchema {
    fn clear_default(&mut self) {
        match self {
            JsonSchema::String(v) => v.default = None,
            JsonSchema::Boolean(v) => v.default = None,
            JsonSchema::Number(v) => v.default = None,
            JsonSchema::Null(v) => v.default = None,
            JsonSchema::Array(a) => a.clear_default(),
            JsonSchema::Object(o) => o.clear_default()
        }
    }
}
