use serde_json::{json, Value};

use super::{
    error::DecodeError,
    field_type::FieldType,
    literal_list_value::{
        BooleanListFieldValue, DateTimeListFieldValue, NumericListFieldValue,
        SingleLineListFieldValue, TableRowListFieldValue,
    },
    literal_naive_value::{
        BooleanFieldValue, CheckboxFieldValue, DateTimeFieldValue, NumericFieldValue,
        RadioButtonFieldValue, SingleLineFieldValue, TableRowFieldValue, UserBoundaryFieldValue,
    },
    literal_value::{LiteralValue, ParseLiteralValueError},
};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValuePresenter {
    BooleanField(BooleanFieldValue),
    CheckboxField(CheckboxFieldValue),
    DateTimeField(DateTimeFieldValue),
    NumericField(NumericFieldValue),
    RadioButtonField(RadioButtonFieldValue),
    SingleLineField(SingleLineFieldValue),
    TableRowField(TableRowFieldValue),
    UserBoundaryField(UserBoundaryFieldValue),

    // list field
    BooleanListField(BooleanListFieldValue),
    DateTimeListField(DateTimeListFieldValue),
    NumericListField(NumericListFieldValue),
    SingleLineListField(SingleLineListFieldValue),
    TableRowListField(TableRowListFieldValue),
}

impl LiteralValuePresenter {
    pub fn get_field_type(&self) -> FieldType {
        match self {
            LiteralValuePresenter::BooleanField(value) => value.get_field_type(),
            LiteralValuePresenter::CheckboxField(value) => value.get_field_type(),
            LiteralValuePresenter::DateTimeField(value) => value.get_field_type(),
            LiteralValuePresenter::NumericField(value) => value.get_field_type(),
            LiteralValuePresenter::RadioButtonField(value) => value.get_field_type(),
            LiteralValuePresenter::SingleLineField(value) => value.get_field_type(),
            LiteralValuePresenter::TableRowField(value) => value.get_field_type(),
            LiteralValuePresenter::UserBoundaryField(value) => value.get_field_type(),

            // list field
            LiteralValuePresenter::BooleanListField(value) => value.get_field_type(),
            LiteralValuePresenter::DateTimeListField(value) => value.get_field_type(),
            LiteralValuePresenter::NumericListField(value) => value.get_field_type(),
            LiteralValuePresenter::SingleLineListField(value) => value.get_field_type(),
            LiteralValuePresenter::TableRowListField(value) => value.get_field_type(),
        }
    }

    pub fn from_json(json: &Value) -> Result<Self, DecodeError> {
        if !json.is_object() {
            return Err(DecodeError::InvalidJsonObject(json));
        }

        match json.get("field_type") {
            Some(field_type_value) => match field_type_value {
                Value::String(ref field_type) => match FieldType::parse_str(field_type) {
                    Ok(field_type) => make_literal_field_value(&field_type, json).map_err(|_err| {
                        DecodeError::InvalidValue {
                            field_type,
                            value: json,
                        }
                    }),
                    Err(_err) => Err(DecodeError::UnsupportedFieldType(json)),
                },
                _other => Err(DecodeError::UnsupportedFieldType(json)),
            },
            None => Err(DecodeError::NoFieldType),
        }
    }

    pub fn to_json(&self) -> Value {
        let value = match self {
            LiteralValuePresenter::BooleanField(value) => value.to_json(),
            LiteralValuePresenter::CheckboxField(value) => value.to_json(),
            LiteralValuePresenter::DateTimeField(value) => value.to_json(),
            LiteralValuePresenter::NumericField(value) => value.to_json(),
            LiteralValuePresenter::RadioButtonField(value) => value.to_json(),
            LiteralValuePresenter::SingleLineField(value) => value.to_json(),
            LiteralValuePresenter::TableRowField(value) => value.to_json(),
            LiteralValuePresenter::UserBoundaryField(value) => value.to_json(),

            // list field
            LiteralValuePresenter::BooleanListField(value) => value.to_json(),
            LiteralValuePresenter::DateTimeListField(value) => value.to_json(),
            LiteralValuePresenter::NumericListField(value) => value.to_json(),
            LiteralValuePresenter::SingleLineListField(value) => value.to_json(),
            LiteralValuePresenter::TableRowListField(value) => value.to_json(),
        };

        json!({
            "type": "literal",
            "field_type": self.get_field_type().to_str(),
            "value": value,
        })
    }

    pub fn as_boolean_field_value(&self) -> Option<&BooleanFieldValue> {
        match self {
            LiteralValuePresenter::BooleanField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_checkbox_field_value(&self) -> Option<&CheckboxFieldValue> {
        match self {
            LiteralValuePresenter::CheckboxField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_date_time_field_value(&self) -> Option<&DateTimeFieldValue> {
        match self {
            LiteralValuePresenter::DateTimeField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_numeric_field_value(&self) -> Option<&NumericFieldValue> {
        match self {
            LiteralValuePresenter::NumericField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_radio_button_field_value(&self) -> Option<&RadioButtonFieldValue> {
        match self {
            LiteralValuePresenter::RadioButtonField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_single_line_field_value(&self) -> Option<&SingleLineFieldValue> {
        match self {
            LiteralValuePresenter::SingleLineField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_table_row_field_value(&self) -> Option<&TableRowFieldValue> {
        match self {
            LiteralValuePresenter::TableRowField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_user_boundary_field_value(&self) -> Option<&UserBoundaryFieldValue> {
        match self {
            LiteralValuePresenter::UserBoundaryField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_boolean_list_field_value(&self) -> Option<&BooleanListFieldValue> {
        match self {
            LiteralValuePresenter::BooleanListField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_date_time_list_field_value(&self) -> Option<&DateTimeListFieldValue> {
        match self {
            LiteralValuePresenter::DateTimeListField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_numeric_list_field_value(&self) -> Option<&NumericListFieldValue> {
        match self {
            LiteralValuePresenter::NumericListField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_single_line_list_field_value(&self) -> Option<&SingleLineListFieldValue> {
        match self {
            LiteralValuePresenter::SingleLineListField(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_table_row_list_field_value(&self) -> Option<&TableRowListFieldValue> {
        match self {
            LiteralValuePresenter::TableRowListField(value) => Some(value),
            _ => None,
        }
    }
}

fn make_literal_field_value(
    field_type: &FieldType,
    value: &Value,
) -> Result<LiteralValuePresenter, ParseLiteralValueError> {
    if !value.is_object() {
        return Err(ParseLiteralValueError);
    }

    match field_type {
        FieldType::BooleanField => match value.get("value") {
            Some(value) => {
                BooleanFieldValue::from_json(value).map(LiteralValuePresenter::BooleanField)
            }
            None => Ok(LiteralValuePresenter::BooleanField(BooleanFieldValue::Nil)),
        },
        FieldType::CheckboxField => match value.get("value") {
            Some(value) => {
                CheckboxFieldValue::from_json(value).map(LiteralValuePresenter::CheckboxField)
            }
            None => Ok(LiteralValuePresenter::CheckboxField(
                CheckboxFieldValue::Nil,
            )),
        },
        FieldType::DateTimeField => match value.get("value") {
            Some(value) => {
                DateTimeFieldValue::from_json(value).map(LiteralValuePresenter::DateTimeField)
            }
            None => Ok(LiteralValuePresenter::DateTimeField(
                DateTimeFieldValue::Nil,
            )),
        },
        FieldType::NumericField => match value.get("value") {
            Some(value) => {
                NumericFieldValue::from_json(value).map(LiteralValuePresenter::NumericField)
            }
            None => Ok(LiteralValuePresenter::NumericField(NumericFieldValue::Nil)),
        },
        FieldType::RadioButtonField => match value.get("value") {
            Some(value) => RadioButtonFieldValue::from_json(value)
                .and_then(|field_value| match field_value {
                    RadioButtonFieldValue::Nil => Ok(field_value),
                    RadioButtonFieldValue::Value(ref options_value) => {
                        if options_value.count_options() <= 1 {
                            Ok(field_value)
                        } else {
                            Err(ParseLiteralValueError)
                        }
                    }
                })
                .map(LiteralValuePresenter::RadioButtonField),
            None => Ok(LiteralValuePresenter::RadioButtonField(
                RadioButtonFieldValue::Nil,
            )),
        },
        FieldType::SingleLineField => match value.get("value") {
            Some(value) => {
                SingleLineFieldValue::from_json(value).map(LiteralValuePresenter::SingleLineField)
            }
            None => Ok(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Nil,
            )),
        },
        FieldType::TableRowField => match value.get("value") {
            Some(value) => {
                TableRowFieldValue::from_json(value).map(LiteralValuePresenter::TableRowField)
            }
            None => Ok(LiteralValuePresenter::TableRowField(
                TableRowFieldValue::Nil,
            )),
        },
        FieldType::UserBoundaryField => match value.get("value") {
            Some(value) => UserBoundaryFieldValue::from_json(value)
                .map(LiteralValuePresenter::UserBoundaryField),
            None => Ok(LiteralValuePresenter::UserBoundaryField(
                UserBoundaryFieldValue::Nil,
            )),
        },

        // list field
        FieldType::BooleanListField => match value.get("value") {
            Some(value) => {
                BooleanListFieldValue::from_json(value).map(LiteralValuePresenter::BooleanListField)
            }
            None => Ok(LiteralValuePresenter::BooleanListField(
                BooleanListFieldValue::Nil,
            )),
        },
        FieldType::DateTimeListField => match value.get("value") {
            Some(value) => DateTimeListFieldValue::from_json(value)
                .map(LiteralValuePresenter::DateTimeListField),
            None => Ok(LiteralValuePresenter::DateTimeListField(
                DateTimeListFieldValue::Nil,
            )),
        },
        FieldType::NumericListField => match value.get("value") {
            Some(value) => {
                NumericListFieldValue::from_json(value).map(LiteralValuePresenter::NumericListField)
            }
            None => Ok(LiteralValuePresenter::NumericListField(
                NumericListFieldValue::Nil,
            )),
        },
        FieldType::SingleLineListField => match value.get("value") {
            Some(value) => SingleLineListFieldValue::from_json(value)
                .map(LiteralValuePresenter::SingleLineListField),
            None => Ok(LiteralValuePresenter::SingleLineListField(
                SingleLineListFieldValue::Nil,
            )),
        },
        FieldType::TableRowListField => match value.get("value") {
            Some(value) => TableRowListFieldValue::from_json(value)
                .map(LiteralValuePresenter::TableRowListField),
            None => Ok(LiteralValuePresenter::TableRowListField(
                TableRowListFieldValue::Nil,
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::value_presenter::value::{
        naive_date_time::NaiveDateTime, number::Number, options_value::OptionsValue,
        user_boundary::UserBoundary, uuid::Uuid,
    };

    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_field_type() {
        let vp = LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(
            "value".to_string(),
        ));
        let field_type = vp.get_field_type();

        assert!(field_type == FieldType::SingleLineField);
        assert!(field_type != FieldType::BooleanField);
    }

    // test boolean_field

    #[test]
    fn test_make_literal_boolean_field_presenter() {
        // test true value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": true
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::BooleanField(BooleanFieldValue::Value(true))
            ));
        }

        // test false value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": false
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::BooleanField(BooleanFieldValue::Value(false))
            ));
        }

        // test null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": null
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::BooleanField(BooleanFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::BooleanField(BooleanFieldValue::Nil)
            ));
        }

        // test invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": 123
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_boolean_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::BooleanField(BooleanFieldValue::Value(true));
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "BOOLEAN_FIELD", "value": true});

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::BooleanField(BooleanFieldValue::Nil);
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "BOOLEAN_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    // test checkbox_field
    #[test]
    fn test_make_literal_checkbox_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
                "value": {
                    "options": ["option1", "option2"],
                    "other": "other"
                }
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::CheckboxField(CheckboxFieldValue::Value(_))
            ));
        }

        // test null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
                "value": null
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::CheckboxField(CheckboxFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::CheckboxField(CheckboxFieldValue::Nil)
            ));
        }

        // test invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
                "value": "invalid"
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_checkbox_field_value_presenter_to_json() {
        {
            let vp =
                LiteralValuePresenter::CheckboxField(CheckboxFieldValue::Value(OptionsValue {
                    options: vec!["option1".to_string(), "option2".to_string()],
                    other: Some(String::from("other")),
                }));
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "CHECKBOX_FIELD",
                "value": {"options": ["option1", "option2"], "other": "other"}
            });

            assert!(str == expected.to_string());
        }

        // empty options and other
        {
            let vp =
                LiteralValuePresenter::CheckboxField(CheckboxFieldValue::Value(OptionsValue {
                    options: vec![],
                    other: None,
                }));
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "CHECKBOX_FIELD",
                "value": {"options": [], "other": null}
            });

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::CheckboxField(CheckboxFieldValue::Nil);
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "CHECKBOX_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }

    // test date_time_field

    #[test]
    fn test_make_literal_date_time_field_presenter() {
        let expected = NaiveDateTime {
            year: 2022,
            month: 4,
            day: 29,
            hour: 7,
            minute: 34,
            second: 10,
            nanosecond: 420159000,
        };

        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field",
                "value": "2022-04-29T07:34:10.420159"
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Value(value))
                if value == expected
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field",
                "value": null
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Nil)
            ));
        }

        // value is not present, so we should get None
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field"
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field",
                "value": 123
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_date_time_field_value_presenter_to_json() {
        {
            let vp =
                LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Value(NaiveDateTime {
                    year: 2020,
                    month: 1,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    second: 0,
                    nanosecond: 0,
                }));
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "DATE_TIME_FIELD", "value": "2020-01-01T00:00:00"});

            assert!(str == expected.to_string());
        }

        // with ms
        {
            let vp =
                LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Value(NaiveDateTime {
                    year: 2020,
                    month: 1,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    second: 0,
                    nanosecond: 123456000,
                }));
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "DATE_TIME_FIELD", "value": "2020-01-01T00:00:00.123456"});

            assert!(str == expected.to_string());
        }

        {
            let vp = LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Nil);
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "DATE_TIME_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    // test numeric_field
    #[test]
    fn test_make_literal_number_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": 123 as i64
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::NumericField(NumericFieldValue::Value(Number::Integer(123)))
            ));
        }

        // float
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": 123.123 as f64
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            let expected = Number::Float(123.123);

            assert!(matches!(
                vp,
                LiteralValuePresenter::NumericField(NumericFieldValue::Value(value)) if value == expected
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": null
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::NumericField(NumericFieldValue::Nil)
            ));
        }

        // value is not present, so we should get None
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::NumericField(NumericFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": "123"
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_numeric_field_value_presenter_to_json() {
        {
            let vp =
                LiteralValuePresenter::NumericField(NumericFieldValue::Value(Number::Integer(123)));
            let str = vp.to_json().to_string();

            let expected: Value = serde_json::from_str(
                r#"
                {
                    "type": "literal",
                    "field_type": "NUMERIC_FIELD",
                    "value": 123
                }"#,
            )
            .unwrap();

            assert!(str == expected.to_string());
        }

        // float
        {
            let vp =
                LiteralValuePresenter::NumericField(NumericFieldValue::Value(Number::Float(123.1)));
            let str = vp.to_json().to_string();
            let expected: Value = serde_json::from_str(
                r#"
                {
                    "type": "literal",
                    "field_type": "NUMERIC_FIELD",
                    "value": 123.1
                }"#,
            )
            .unwrap();

            assert!(str == expected.to_string());
        }

        // null
        {
            let vp = LiteralValuePresenter::NumericField(NumericFieldValue::Nil);
            let str = vp.to_json().to_string();

            let expected: Value = serde_json::from_str(
                r#"
                {
                    "type": "literal",
                    "field_type": "NUMERIC_FIELD",
                    "value": null
                }"#,
            )
            .unwrap();

            assert!(str == expected.to_string());
        }
    }

    // test radio_button_field
    #[test]
    fn test_make_literal_radio_button_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "radio_button_field",
                "value": {
                    "options": ["option"],
                    "other": null
                }
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::RadioButtonField(RadioButtonFieldValue::Value(OptionsValue {
                    options,
                    other: None
                })) if options.len() == 1
            ));
        }

        // test null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "radio_button_field",
                "value": null
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::RadioButtonField(RadioButtonFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "radio_button_field",
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::RadioButtonField(RadioButtonFieldValue::Nil)
            ));
        }

        // test invalid options count
        {
            let json = json!({
                "type": "literal",
                "field_type": "radio_button_field",
                "value": {
                    "options": ["option1", "option2"],
                    "other": null
                }
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }

        // test invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "radio_button_field",
                "value": "invalid"
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_radio_button_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::RadioButtonField(RadioButtonFieldValue::Value(
                OptionsValue {
                    options: vec!["option".to_string()],
                    other: None,
                },
            ));
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "RADIO_BUTTON_FIELD",
                "value": {"options": ["option"], "other": null}
            });

            assert!(str == expected.to_string());
        }

        // with other
        {
            let vp = LiteralValuePresenter::RadioButtonField(RadioButtonFieldValue::Value(
                OptionsValue {
                    options: vec![],
                    other: Some("other".to_string()),
                },
            ));
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "RADIO_BUTTON_FIELD",
                "value": {"options": [], "other": "other"}
            });

            assert!(str == expected.to_string());
        }

        // empty options and other
        {
            let vp = LiteralValuePresenter::RadioButtonField(RadioButtonFieldValue::Value(
                OptionsValue {
                    options: vec![],
                    other: None,
                },
            ));
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "RADIO_BUTTON_FIELD",
                "value": {"options": [], "other": null}
            });

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::RadioButtonField(RadioButtonFieldValue::Nil);
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "RADIO_BUTTON_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }

    // test single_select_field

    #[test]
    fn test_make_literal_single_line_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field",
                "value": "value"
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(value))
                if value == "value"
            ));
        }

        // test null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field",
                "value": null
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Nil)
            ));
        }

        // value is not present, so we should get None
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field"
            });

            let vp = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Nil)
            ));
        }

        // test invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field",
                "value": 123
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_single_line_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(
                "hello".to_string(),
            ));
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "SINGLE_LINE_FIELD", "value": "hello"});

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Nil);
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "SINGLE_LINE_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    // test table_row_field

    #[test]
    fn test_make_literal_table_row_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field",
                "value": "67e55044-10b1-426f-9247-bb680e5fe0c8"
            });

            let expected_uuid = Uuid("67e55044-10b1-426f-9247-bb680e5fe0c8".to_string());

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::TableRowField(TableRowFieldValue::Value(value))
                if value == expected_uuid
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field",
                "value": null
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::TableRowField(TableRowFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field"
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::TableRowField(TableRowFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field",
                "value": "invalid"
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_table_row_field_value_presenter_to_json() {
        {
            let uuid_str = "67e55044-10b1-426f-9247-bb680e5fe0c8";
            let vp = LiteralValuePresenter::TableRowField(TableRowFieldValue::Value(Uuid(
                uuid_str.to_string(),
            )));
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "TABLE_ROW_FIELD", "value": uuid_str});

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::TableRowField(TableRowFieldValue::Nil);
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "TABLE_ROW_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    // test user_boundary_field
    #[test]
    fn test_make_literal_user_boundary_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field",
                "value": {
                    "user_uuids": ["67e55044-10b1-426f-9247-bb680e5fe0c8"]
                }
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::UserBoundaryField(UserBoundaryFieldValue::Value(UserBoundary {
                    user_uuids,
                    simple_department_uuids: _,
                    penetrating_department_uuids: _
                })) if user_uuids.len() == 1
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field",
                "value": null
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::UserBoundaryField(UserBoundaryFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field"
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::UserBoundaryField(UserBoundaryFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field",
                "value": "invalid"
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_user_boundary_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::UserBoundaryField(UserBoundaryFieldValue::Value(
                UserBoundary {
                    user_uuids: vec![Uuid("00000000-0000-0000-0000-ffff00000000".to_string())],
                    simple_department_uuids: vec![
                        Uuid("00000000-0000-0000-0000-ffff00000001".to_string()),
                        Uuid("00000000-0000-0000-0000-ffff00000002".to_string()),
                    ],
                    penetrating_department_uuids: vec![
                        Uuid("00000000-0000-0000-0000-ffff00000003".to_string()),
                        Uuid("00000000-0000-0000-0000-ffff00000004".to_string()),
                        Uuid("00000000-0000-0000-0000-ffff00000005".to_string()),
                    ],
                },
            ));
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "USER_BOUNDARY_FIELD",
                "value": {
                    "user_uuids": [
                        "00000000-0000-0000-0000-ffff00000000",
                    ],
                    "simple_department_uuids": [
                        "00000000-0000-0000-0000-ffff00000001",
                        "00000000-0000-0000-0000-ffff00000002",
                    ],
                    "penetrating_department_uuids": [
                        "00000000-0000-0000-0000-ffff00000003",
                        "00000000-0000-0000-0000-ffff00000004",
                        "00000000-0000-0000-0000-ffff00000005",
                    ],
                }
            });

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::UserBoundaryField(UserBoundaryFieldValue::Nil);
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "USER_BOUNDARY_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }

    // test boolean_list_field
    #[test]
    fn test_make_literal_boolean_list_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_list_field",
                "value": [true, false, null]
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            let expected = vec![
                BooleanFieldValue::Value(true),
                BooleanFieldValue::Value(false),
                BooleanFieldValue::Nil,
            ];

            assert!(matches!(
                result,
                LiteralValuePresenter::BooleanListField(BooleanListFieldValue::Value(values)) if values.as_slice() ==  expected
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_list_field",
                "value": null
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::BooleanListField(BooleanListFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_list_field",
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::BooleanListField(BooleanListFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_list_field",
                "value": 123
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_boolean_list_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::BooleanListField(BooleanListFieldValue::Value(vec![
                BooleanFieldValue::Value(true),
                BooleanFieldValue::Value(false),
                BooleanFieldValue::Nil,
            ]));

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "BOOLEAN_LIST_FIELD",
                "value": [true, false, null]
            });

            assert!(str == expected.to_string());
        }

        {
            let vp = LiteralValuePresenter::BooleanListField(BooleanListFieldValue::Nil);

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "BOOLEAN_LIST_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }

    // test date_time_list_field
    #[test]
    fn test_make_literal_date_time_list_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_list_field",
                "value": [
                    "2020-01-01T00:00:00Z",
                    null,
                ]
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::DateTimeListField(DateTimeListFieldValue::Value(value)) if value.as_slice() == [
                DateTimeFieldValue::Value(NaiveDateTime::new(2020, 1, 1, 0, 0, 0, 0)),
                DateTimeFieldValue::Nil,
                ]

            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_list_field",
                "value": null
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::DateTimeListField(DateTimeListFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_list_field",
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::DateTimeListField(DateTimeListFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_list_field",
                "value": 123
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_date_time_list_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::DateTimeListField(DateTimeListFieldValue::Value(vec![
                DateTimeFieldValue::Value(NaiveDateTime::new(2020, 1, 1, 0, 0, 0, 0)),
                DateTimeFieldValue::Nil,
            ]));

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "DATE_TIME_LIST_FIELD",
                "value": [
                    "2020-01-01T00:00:00",
                    null,
                ]
            });

            assert!(str == expected.to_string());
        }

        {
            let vp = LiteralValuePresenter::DateTimeListField(DateTimeListFieldValue::Nil);

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "DATE_TIME_LIST_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }

    // test numeric_list_field
    #[test]
    fn test_make_literal_numeric_list_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_list_field",
                "value": [
                    1,
                    1.2 as f64,
                    null,
                ]
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            let expected = vec![
                NumericFieldValue::Value(Number::Integer(1)),
                NumericFieldValue::Value(Number::Float(1.2)),
                NumericFieldValue::Nil,
            ];

            assert!(matches!(
                result,
                LiteralValuePresenter::NumericListField(NumericListFieldValue::Value(value)) if value.as_slice() == expected
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_list_field",
                "value": null
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::NumericListField(NumericListFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_list_field",
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::NumericListField(NumericListFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_list_field",
                "value": 123
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_numeric_list_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::NumericListField(NumericListFieldValue::Value(vec![
                NumericFieldValue::Value(Number::Integer(1)),
                NumericFieldValue::Value(Number::Float(1.2)),
                NumericFieldValue::Nil,
            ]));

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "NUMERIC_LIST_FIELD",
                "value": [
                    1,
                    1.2,
                    null,
                ]
            });

            assert!(str == expected.to_string());
        }

        {
            let vp = LiteralValuePresenter::NumericListField(NumericListFieldValue::Nil);

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "NUMERIC_LIST_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }

    // test single_line_list_field
    #[test]
    fn test_make_literal_single_line_list_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_list_field",
                "value": [
                    "foo",
                    null
                ]
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            let expected = vec![
                SingleLineFieldValue::Value("foo".to_string()),
                SingleLineFieldValue::Nil,
            ];

            assert!(matches!(
                result,
                LiteralValuePresenter::SingleLineListField(SingleLineListFieldValue::Value(value)) if value.as_slice() == expected
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_list_field",
                "value": null
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::SingleLineListField(SingleLineListFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_list_field",
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::SingleLineListField(SingleLineListFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_list_field",
                "value": 123
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_single_line_list_field_value_presenter_to_json() {
        {
            let vp =
                LiteralValuePresenter::SingleLineListField(SingleLineListFieldValue::Value(vec![
                    SingleLineFieldValue::Value("foo".to_string()),
                    SingleLineFieldValue::Nil,
                ]));

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "SINGLE_LINE_LIST_FIELD",
                "value": [
                    "foo",
                    null,
                ]

            });

            assert!(str == expected.to_string());
        }

        {
            let vp = LiteralValuePresenter::SingleLineListField(SingleLineListFieldValue::Nil);

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "SINGLE_LINE_LIST_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }

    // test table_row_list_field
    #[test]
    fn test_make_literal_table_row_list_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_list_field",
                "value": [
                    "00000000-0000-0000-0000-000000000000",
                    null
                ]
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            let expected = vec![
                TableRowFieldValue::Value(Uuid("00000000-0000-0000-0000-000000000000".to_string())),
                TableRowFieldValue::Nil,
            ];

            assert!(matches!(
                result,
                LiteralValuePresenter::TableRowListField(TableRowListFieldValue::Value(value)) if value.as_slice() == expected
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_list_field",
                "value": null
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::TableRowListField(TableRowListFieldValue::Nil)
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_list_field",
            });

            let result = LiteralValuePresenter::from_json(&json).unwrap();

            assert!(matches!(
                result,
                LiteralValuePresenter::TableRowListField(TableRowListFieldValue::Nil)
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_list_field",
                "value": 123 as i64
            });

            let result = LiteralValuePresenter::from_json(&json);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    #[test]
    fn test_literal_table_row_list_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::TableRowListField(TableRowListFieldValue::Value(vec![
                TableRowFieldValue::Value(Uuid("00000000-0000-0000-0000-000000000000".to_string())),
                TableRowFieldValue::Nil,
            ]));

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "TABLE_ROW_LIST_FIELD",
                "value": [
                    "00000000-0000-0000-0000-000000000000",
                    null,

                ]
            });

            assert!(str == expected.to_string());
        }

        {
            let vp = LiteralValuePresenter::TableRowListField(TableRowListFieldValue::Nil);

            let str = vp.to_json().to_string();

            let expected = json!({
                "type": "literal",
                "field_type": "TABLE_ROW_LIST_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }
}