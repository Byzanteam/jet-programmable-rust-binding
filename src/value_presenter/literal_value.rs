use super::field_type::FieldType;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseLiteralValueError;

pub trait LiteralValue {
    fn is_nil(&self) -> bool;
    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError>
    where
        Self: Sized;
    fn to_json(&self) -> Value;
    fn get_field_type(&self) -> FieldType;
}
