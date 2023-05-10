use serde_json::Value;
use crate::model::json_schema::array::JsonSchemaArray;
use crate::model::json_schema::{FundamentalType, JsonSchema};
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
    pub fn get_type(&self) -> FundamentalType {
        match self {
            JsonSchema::String(_) |
            JsonSchema::Boolean(_) |
            JsonSchema::Number(_) => FundamentalType::Scalar,
            JsonSchema::Null(_) => FundamentalType::Null,
            JsonSchema::Array(_) => FundamentalType::Vector,
            JsonSchema::Object(_) => FundamentalType::Object,
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
        let types = (self.get_type(), rhs.get_type());

        match types {
            (FundamentalType::Vector, FundamentalType::Vector) => {
                let array = self.as_array().union(rhs.as_array());

                JsonSchema::Array(array)
            }
            (FundamentalType::Object, FundamentalType::Object) => {
                let object = self.as_object().union(rhs.as_object());

                JsonSchema::Object(object)
            }
            (FundamentalType::Scalar, FundamentalType::Scalar) => {
                let scalar = self.as_scalar().union(rhs.as_scalar());

                match (self, rhs) {
                    (JsonSchema::String(_), _) |
                    (_, JsonSchema::String(_)) => JsonSchema::String(scalar),
                    (JsonSchema::Number(_), JsonSchema::Number(_)) => JsonSchema::Number(scalar),
                    (JsonSchema::Boolean(_), JsonSchema::Boolean(_)) => JsonSchema::Boolean(scalar),
                    _ => panic!("cannot combine lhs with rhs as scalar: {:?}, {:?}", self, rhs)
                }
            }
            (FundamentalType::Null, _) => self.clone(),
            (_, FundamentalType::Null) => rhs.clone(),
            _ => panic!("cannot combine lhs with rhs: {:?}, {:?}", self, rhs)
        }
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
