use super::{field_type::FieldType, value::json_codec::JsonCodec};
use serde_json::Value;

use super::value::{
    naive_date_time::NaiveDateTime, number::Number, options_value::OptionsValue,
    user_boundary::UserBoundary, uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BooleanFieldValue {
    Value(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CheckboxFieldValue {
    Value(OptionsValue),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateTimeFieldValue {
    Value(NaiveDateTime),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumericFieldValue {
    Value(Number),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RadioButtonFieldValue {
    Value(OptionsValue),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SingleLineFieldValue {
    Value(String),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableRowFieldValue {
    Value(Uuid),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserBoundaryFieldValue {
    Value(UserBoundary),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseLiteralFieldValueError;

pub trait LiteralFieldValue {
    fn is_nil(&self) -> bool;
    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError>
    where
        Self: Sized;
    fn to_json(&self) -> Value;
    fn get_field_type(&self) -> FieldType;
}

impl LiteralFieldValue for BooleanFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, BooleanFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(BooleanFieldValue::Nil);
        }

        match value.as_bool() {
            Some(v) => Ok(BooleanFieldValue::Value(v)),
            None => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            BooleanFieldValue::Value(value) => Value::Bool(*value),
            BooleanFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::BooleanField
    }
}

impl LiteralFieldValue for CheckboxFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, CheckboxFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(CheckboxFieldValue::Nil);
        }

        match OptionsValue::from_json(value) {
            Ok(v) => Ok(CheckboxFieldValue::Value(v)),
            Err(_) => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            CheckboxFieldValue::Value(value) => value.to_json(),
            CheckboxFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::CheckboxField
    }
}

impl LiteralFieldValue for DateTimeFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, DateTimeFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(DateTimeFieldValue::Nil);
        }

        match NaiveDateTime::from_json(value) {
            Ok(v) => Ok(DateTimeFieldValue::Value(v)),
            Err(_) => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            DateTimeFieldValue::Value(value) => value.to_json(),
            DateTimeFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::DateTimeField
    }
}

impl LiteralFieldValue for NumericFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, NumericFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(NumericFieldValue::Nil);
        }

        match Number::from_json(value) {
            Ok(v) => Ok(NumericFieldValue::Value(v)),
            Err(_) => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            NumericFieldValue::Value(value) => value.to_json(),
            NumericFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::NumericField
    }
}

impl LiteralFieldValue for RadioButtonFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, RadioButtonFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(RadioButtonFieldValue::Nil);
        }

        match OptionsValue::from_json(value) {
            Ok(v) => Ok(RadioButtonFieldValue::Value(v)),
            Err(_) => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            RadioButtonFieldValue::Value(value) => value.to_json(),
            RadioButtonFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::RadioButtonField
    }
}

impl LiteralFieldValue for SingleLineFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, SingleLineFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(SingleLineFieldValue::Nil);
        }

        match value.as_str() {
            Some(v) => Ok(SingleLineFieldValue::Value(v.to_string())),
            None => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            SingleLineFieldValue::Value(value) => Value::String(value.to_string()),
            SingleLineFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::SingleLineField
    }
}

impl LiteralFieldValue for TableRowFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, TableRowFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(TableRowFieldValue::Nil);
        }

        match Uuid::from_json(value) {
            Ok(v) => Ok(TableRowFieldValue::Value(v)),
            Err(_) => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            TableRowFieldValue::Value(value) => value.to_json(),
            TableRowFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::TableRowField
    }
}

impl LiteralFieldValue for UserBoundaryFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, UserBoundaryFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralFieldValueError> {
        if value.is_null() {
            return Ok(UserBoundaryFieldValue::Nil);
        }

        match UserBoundary::from_json(value) {
            Ok(v) => Ok(UserBoundaryFieldValue::Value(v)),
            Err(_) => Err(ParseLiteralFieldValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            UserBoundaryFieldValue::Value(value) => value.to_json(),
            UserBoundaryFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::UserBoundaryField
    }
}
