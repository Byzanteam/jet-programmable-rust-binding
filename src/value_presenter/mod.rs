pub mod error;
pub mod field_type;
pub mod value;

pub mod literal_list_value;
pub mod literal_naive_value;
pub mod literal_value;

pub mod literal_value_presenter;

use serde_json::Value;

use error::DecodeError;
use field_type::FieldType;
use literal_value_presenter::LiteralValuePresenter;

#[derive(Debug, Clone, PartialEq)]
pub enum ValuePresenter {
    Literal(LiteralValuePresenter),
}

impl ValuePresenter {
    pub fn get_field_type(&self) -> FieldType {
        match self {
            ValuePresenter::Literal(literal_value_presenter) => {
                literal_value_presenter.get_field_type()
            }
        }
    }
}

impl ValuePresenter {
    pub fn from_json(json: &Value) -> Result<Self, DecodeError> {
        if !json.is_object() {
            return Err(DecodeError::InvalidJsonObject(json));
        }

        match json.get("type") {
            Some(value) => match value {
                Value::String(ref type_name) => match type_name.as_str() {
                    "literal" | "LITERAL" => match LiteralValuePresenter::from_json(json) {
                        Ok(literal_vp) => Ok(ValuePresenter::Literal(literal_vp)),
                        Err(error) => Err(error),
                    },
                    _ => Err(DecodeError::UnsupportedType(json)),
                },
                _ => Err(DecodeError::UnsupportedType(json)),
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::value_presenter::literal_naive_value::BooleanFieldValue;

    use super::*;

    #[test]
    fn test_literal_value_presenter_from_json() {
        {
            let json = json!({
                "type": "LITERAL",
                "field_type": "boolean_field",
                "value": true,
            });

            let vp = ValuePresenter::from_json(&json).unwrap();

            assert!(
                vp == ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Value(true)
                ))
            );
        }

        {
            let json = json!({
                "type": "LITERAL",
                "field_type": "BOOLEAN_FIELD",
                "value": true,
            });

            let vp = ValuePresenter::from_json(&json).unwrap();

            assert!(
                vp == ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Value(true)
                ))
            );
        }
    }

    #[test]
    fn test_literal_value_presenter_to_json() {
        let vp = ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
            BooleanFieldValue::Value(true),
        ));

        assert!(
            vp.to_json()
                == json!({
                    "type": "LITERAL",
                    "field_type": "BOOLEAN_FIELD",
                    "value": true,
                })
        );
    }
}
