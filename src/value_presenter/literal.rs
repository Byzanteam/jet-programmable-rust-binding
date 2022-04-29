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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_field_type_matches() {
        let vp = LiteralValuePresenter::SingleLineField(Some("value".to_string()));

        assert_eq!(vp.field_type_matches(&FieldType::SingleLineField), true);
        assert_eq!(vp.field_type_matches(&FieldType::BooleanField), false);
    }

    #[test]
    fn test_make_single_line_field_presenter() {
        let json = json!({
            "type": "literal",
            "field_type": "single_line_field",
            "value": "value"
        });

        let object = json.as_object().unwrap();
        let vp = make_single_line_field_presenter(object).unwrap();

        assert!(matches!(
            vp,
            LiteralValuePresenter::SingleLineField(Some(_))
        ));
    }

    #[test]
    fn test_make_single_line_field_presenter_with_null() {
        let json = json!({
            "type": "literal",
            "field_type": "single_line_field",
            "value": null
        });

        let object = json.as_object().unwrap();
        let vp = make_single_line_field_presenter(object).unwrap();

        assert!(matches!(vp, LiteralValuePresenter::SingleLineField(None)));

        // value is not present, so we should get None
        let json = json!({
            "type": "literal",
            "field_type": "single_line_field"
        });

        let object = json.as_object().unwrap();
        let vp = make_single_line_field_presenter(object).unwrap();

        assert!(matches!(vp, LiteralValuePresenter::SingleLineField(None)));
    }

    #[test]
    fn test_make_single_line_field_presenter_and_return_error() {
        let json = json!({
            "type": "literal",
            "field_type": "single_line_field",
            "value": 123
        });

        let object = json.as_object().unwrap();
        let result = make_single_line_field_presenter(object);

        assert!(matches!(
            result,
            Err(DecodeError::InvalidValue {
                field_type: _,
                value: _
            })
        ));
    }
}
