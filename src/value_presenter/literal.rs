use serde_json::{Map, Value};

use super::{
    error::DecodeError,
    field_type::FieldType,
    value_type::{NaiveDateTime, OptionsValue, UserBoundary, UuidV4},
};

#[derive(Debug)]
pub enum LiteralValuePresenter {
    SingleLineField(Option<String>),
    UserBoundaryField(Option<UserBoundary>),
    BooleanField(Option<bool>),
    CheckboxField(Option<OptionsValue>),
    DateTimeField(Option<NaiveDateTime>),
    NumericField(Option<f64>),
    RadioButtonField(Option<OptionsValue>),
    TableRowField(Option<UuidV4>),
}

impl LiteralValuePresenter {
    pub fn field_type_matches(&self, field_type: &FieldType) -> bool {
        match self {
            LiteralValuePresenter::SingleLineField(_value) => {
                *field_type == FieldType::SingleLineField
            }
            _ => false,
        }
    }
}

pub fn make_literal_value_presenter(
    object: &Map<String, Value>,
) -> Result<LiteralValuePresenter, DecodeError> {
    match object.get("field_type") {
        Some(value) => match value {
            Value::String(ref field_type) => match FieldType::from_str(field_type) {
                Some(FieldType::SingleLineField) => make_single_line_field_presenter(object),
                _ => Err(DecodeError::UnsupportedFieldType(value)),
            },
            value => Err(DecodeError::UnsupportedFieldType(value)),
        },
        None => Err(DecodeError::NoFieldType),
    }
}

fn make_single_line_field_presenter(
    object: &Map<String, Value>,
) -> Result<LiteralValuePresenter, DecodeError> {
    match object.get("value") {
        Some(Value::String(value)) => Ok(LiteralValuePresenter::SingleLineField(Some(
            value.to_string(),
        ))),
        Some(Value::Null) => Ok(LiteralValuePresenter::SingleLineField(None)),
        Some(value) => Err(DecodeError::InvalidValue {
            field_type: FieldType::SingleLineField,
            value,
        }),
        None => Ok(LiteralValuePresenter::SingleLineField(None)),
    }
}
