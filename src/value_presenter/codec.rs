use serde_json::{Map, Value};

use super::{error::DecodeError, literal::make_literal_value_presenter, ValuePresenter};

impl ValuePresenter {
    pub fn from_json(object: &Map<String, Value>) -> Result<Self, DecodeError> {
        match object.get("type") {
            Some(value) => match value {
                Value::String(ref type_name) => match type_name.as_str() {
                    "literal" => match make_literal_value_presenter(object) {
                        Ok(literal_vp) => Ok(ValuePresenter::Literal(literal_vp)),
                        Err(error) => Err(error),
                    },
                    _ => Err(DecodeError::UnsupportedType(value)),
                },
                _ => Err(DecodeError::UnsupportedType(value)),
            },
            None => Err(DecodeError::NoType),
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            ValuePresenter::Literal(vp) => vp.to_json(),
        }
    }
}
