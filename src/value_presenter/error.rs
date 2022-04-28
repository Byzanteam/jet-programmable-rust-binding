use serde_json::Value;

use super::{field_type::FieldType, ValuePresenter};

pub enum DecodeError<'a> {
    NoType,
    UnsupportedType(&'a Value),

    NoFieldType,
    UnsupportedFieldType(&'a Value),

    InvalidValue {
        field_type: FieldType,
        value: &'a Value,
    },

    InvalidJsonObject(&'a Value),

    MismatchedFieldType {
        field_type: FieldType,
        value_presenter: ValuePresenter,
    },
}
