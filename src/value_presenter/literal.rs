use serde_json::{Map, Number, Value};

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
    NumericField(Option<Number>),
    RadioButtonField(Option<OptionsValue>),
    TableRowField(Option<UuidV4>),
}

impl LiteralValuePresenter {
    pub fn get_field_type(&self) -> FieldType {
        match self {
            LiteralValuePresenter::BooleanField(_) => FieldType::BooleanField,
            LiteralValuePresenter::CheckboxField(_) => FieldType::CheckboxField,
            LiteralValuePresenter::DateTimeField(_) => FieldType::DateTimeField,
            LiteralValuePresenter::NumericField(_) => FieldType::NumericField,
            LiteralValuePresenter::RadioButtonField(_) => FieldType::RadioButtonField,
            LiteralValuePresenter::SingleLineField(_) => FieldType::SingleLineField,
            LiteralValuePresenter::TableRowField(_) => FieldType::TableRowField,
            LiteralValuePresenter::UserBoundaryField(_) => FieldType::UserBoundaryField,
        }
    }
}

pub fn make_literal_value_presenter(
    object: &Map<String, Value>,
) -> Result<LiteralValuePresenter, DecodeError> {
    match object.get("field_type") {
        Some(value) => match value {
            Value::String(ref field_type) => match FieldType::from_str(field_type) {
                Some(ref field_type) => do_make_literal_presenter(field_type, object),
                _ => Err(DecodeError::UnsupportedFieldType(value)),
            },
            value => Err(DecodeError::UnsupportedFieldType(value)),
        },
        None => Err(DecodeError::NoFieldType),
    }
}

fn do_make_literal_presenter<'a>(
    field_type: &FieldType,
    object: &'a Map<String, Value>,
) -> Result<LiteralValuePresenter, DecodeError<'a>> {
    match field_type {
        FieldType::BooleanField => match object.get("value") {
            Some(Value::Bool(value)) => Ok(LiteralValuePresenter::BooleanField(Some(*value))),
            Some(Value::Null) => Ok(LiteralValuePresenter::BooleanField(None)),
            Some(value) => Err(DecodeError::InvalidValue {
                field_type: FieldType::BooleanField,
                value,
            }),
            None => Ok(LiteralValuePresenter::BooleanField(None)),
        },
        FieldType::CheckboxField => match object.get("value") {
            Some(value) => match value {
                Value::Object(object) => match OptionsValue::from_json(object) {
                    Ok(options_value) => {
                        Ok(LiteralValuePresenter::CheckboxField(Some(options_value)))
                    }
                    Err(_err) => Err(DecodeError::InvalidValue {
                        field_type: FieldType::CheckboxField,
                        value,
                    }),
                },
                Value::Null => Ok(LiteralValuePresenter::CheckboxField(None)),
                _value => Err(DecodeError::InvalidValue {
                    field_type: FieldType::CheckboxField,
                    value,
                }),
            },
            None => Ok(LiteralValuePresenter::CheckboxField(None)),
        },
        FieldType::DateTimeField => match object.get("value") {
            Some(value) => match value {
                Value::String(str) => match NaiveDateTime::from_str(str) {
                    Ok(datetime) => Ok(LiteralValuePresenter::DateTimeField(Some(datetime))),
                    Err(_err) => Err(DecodeError::InvalidValue {
                        field_type: FieldType::DateTimeField,
                        value,
                    }),
                },
                Value::Null => Ok(LiteralValuePresenter::DateTimeField(None)),
                _value => Err(DecodeError::InvalidValue {
                    field_type: FieldType::DateTimeField,
                    value,
                }),
            },
            None => Ok(LiteralValuePresenter::DateTimeField(None)),
        },
        FieldType::NumericField => match object.get("value") {
            Some(Value::Number(number)) => {
                Ok(LiteralValuePresenter::NumericField(Some(number.to_owned())))
            }
            Some(Value::Null) => Ok(LiteralValuePresenter::NumericField(None)),
            Some(value) => Err(DecodeError::InvalidValue {
                field_type: FieldType::NumericField,
                value,
            }),
            None => Ok(LiteralValuePresenter::NumericField(None)),
        },
        FieldType::RadioButtonField => match object.get("value") {
            Some(value) => match value {
                Value::Object(object) => match OptionsValue::from_json(object) {
                    Ok(options_value) if options_value.count_options() <= 1 => {
                        Ok(LiteralValuePresenter::RadioButtonField(Some(options_value)))
                    }
                    Ok(_options_value) => Err(DecodeError::InvalidValue {
                        field_type: FieldType::RadioButtonField,
                        value,
                    }),
                    Err(_err) => Err(DecodeError::InvalidValue {
                        field_type: FieldType::RadioButtonField,
                        value,
                    }),
                },
                Value::Null => Ok(LiteralValuePresenter::RadioButtonField(None)),
                _value => Err(DecodeError::InvalidValue {
                    field_type: FieldType::RadioButtonField,
                    value,
                }),
            },
            None => Ok(LiteralValuePresenter::RadioButtonField(None)),
        },
        FieldType::SingleLineField => match object.get("value") {
            Some(Value::String(value)) => Ok(LiteralValuePresenter::SingleLineField(Some(
                value.to_string(),
            ))),
            Some(Value::Null) => Ok(LiteralValuePresenter::SingleLineField(None)),
            Some(value) => Err(DecodeError::InvalidValue {
                field_type: FieldType::SingleLineField,
                value,
            }),
            None => Ok(LiteralValuePresenter::SingleLineField(None)),
        },
        _ => panic!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use time::macros::datetime;

    #[test]
    fn test_field_type_matches() {
        let vp = LiteralValuePresenter::SingleLineField(Some("value".to_string()));
        let field_type = vp.get_field_type();

        assert!(field_type == FieldType::SingleLineField);
        assert!(field_type != FieldType::BooleanField);
    }

    // test boolean_field

    #[test]
    fn test_do_make_literal_boolean_field_presenter() {
        // test true value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": true
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::BooleanField, object).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::BooleanField(Some(true))
            ));
        }

        // test false value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": false
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::BooleanField, object).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::BooleanField(Some(false))
            ));
        }

        // test null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": null
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::BooleanField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::BooleanField(None)));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::BooleanField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::BooleanField(None)));
        }

        // test invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "boolean_field",
                "value": 123
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::BooleanField, object);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    // test checkbox_field
    #[test]
    fn test_do_make_literal_checkbox_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
                "value": {
                    "options": ["option1", "option2"],
                    "other": "other"
                }
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::CheckboxField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::CheckboxField(Some(_))));
        }

        // test null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
                "value": null
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::CheckboxField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::CheckboxField(None)));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::CheckboxField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::CheckboxField(None)));
        }

        // test invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "checkbox_field",
                "value": "invalid"
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::CheckboxField, object);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    // test single_line_field

    // test date_time_field

    #[test]
    fn test_do_make_literal_date_time_field_presenter() {
        let expected = datetime!(2022-04-29 07:34:10.420159);

        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field",
                "value": "2022-04-29T07:34:10.420159"
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::DateTimeField, object).unwrap();

            assert!(
                matches!(vp, LiteralValuePresenter::DateTimeField(Some(NaiveDateTime(value))) if value == expected)
            );
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field",
                "value": null
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::DateTimeField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::DateTimeField(None)));
        }

        // value is not present, so we should get None
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field"
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::DateTimeField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::DateTimeField(None)));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "date_time_field",
                "value": 123
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::DateTimeField, object);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    // test numeric_field
    #[test]
    fn test_do_make_literal_number_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": 123
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::NumericField, object).unwrap();

            assert!(
                matches!(vp, LiteralValuePresenter::NumericField(Some(n)) if n == Number::from(123 as i64))
            );
        }

        // float
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": 123.123
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::NumericField, object).unwrap();

            assert!(
                matches!(vp, LiteralValuePresenter::NumericField(Some(n)) if n == Number::from_f64(123.123).unwrap())
            );
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": null
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::NumericField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::NumericField(None)));
        }

        // value is not present, so we should get None
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::NumericField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::NumericField(None)));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "numeric_field",
                "value": "123"
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::NumericField, object);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    // test radio_button_field
    #[test]
    fn test_do_make_literal_radio_button_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "radio_button_field",
                "value": {
                    "options": ["option"],
                    "other": null
                }
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::RadioButtonField, object).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::RadioButtonField(Some(OptionsValue {
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

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::RadioButtonField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::RadioButtonField(None)));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "radio_button_field",
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::RadioButtonField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::RadioButtonField(None)));
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

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::RadioButtonField, object);

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

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::RadioButtonField, object);

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
    fn test_do_make_literal_single_line_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field",
                "value": "value"
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::SingleLineField, object).unwrap();

            assert!(matches!(
                vp,
                LiteralValuePresenter::SingleLineField(Some(_))
            ));
        }

        // test null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field",
                "value": null
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::SingleLineField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::SingleLineField(None)));
        }

        // value is not present, so we should get None
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field"
            });

            let object = json.as_object().unwrap();
            let vp = do_make_literal_presenter(&FieldType::SingleLineField, object).unwrap();

            assert!(matches!(vp, LiteralValuePresenter::SingleLineField(None)));
        }

        // test invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "single_line_field",
                "value": 123
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::SingleLineField, object);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }
}
