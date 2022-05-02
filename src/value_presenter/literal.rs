use serde_json::{json, Map, Number, Value};

use super::{
    error::DecodeError,
    field_type::FieldType,
    value_type::{NaiveDateTime, OptionsValue, UserBoundary, UuidV4},
};

#[derive(Debug, Clone)]
pub enum LiteralValuePresenter {
    BooleanField(Option<bool>),
    CheckboxField(Option<OptionsValue>),
    DateTimeField(Option<NaiveDateTime>),
    NumericField(Option<Number>),
    RadioButtonField(Option<OptionsValue>),
    SingleLineField(Option<String>),
    TableRowField(Option<UuidV4>),
    UserBoundaryField(Option<UserBoundary>),
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

    pub fn to_json(&self) -> Value {
        match self {
            LiteralValuePresenter::BooleanField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::BooleanField.to_str(),
                    "value": value,
                })
            }
            LiteralValuePresenter::CheckboxField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::CheckboxField.to_str(),
                    "value": value.as_ref().map(|v| v.to_json()),
                })
            }
            LiteralValuePresenter::DateTimeField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::DateTimeField.to_str(),
                    "value": value.as_ref().map(|v| v.to_str()),
                })
            }
            LiteralValuePresenter::NumericField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::NumericField.to_str(),
                    "value": value
                })
            }
            LiteralValuePresenter::RadioButtonField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::RadioButtonField.to_str(),
                    "value": value.as_ref().map(|v| v.to_json()),
                })
            }
            LiteralValuePresenter::SingleLineField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::SingleLineField.to_str(),
                    "value": value,
                })
            }
            LiteralValuePresenter::TableRowField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::TableRowField.to_str(),
                    "value": value.as_ref().map(|v| v.to_str()),
                })
            }
            LiteralValuePresenter::UserBoundaryField(value) => {
                json!({
                    "type": "literal",
                    "field_type": FieldType::UserBoundaryField.to_str(),
                    "value": value.as_ref().map(|v| v.to_json()),
                })
            }
        }
    }
}

pub fn make_literal_value_presenter(
    object: &Map<String, Value>,
) -> Result<LiteralValuePresenter, DecodeError> {
    match object.get("field_type") {
        Some(value) => match value {
            Value::String(ref field_type) => match FieldType::parse_str(field_type) {
                Ok(ref field_type) => do_make_literal_presenter(field_type, object),
                Err(_err) => Err(DecodeError::UnsupportedFieldType(value)),
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
                Value::String(str) => match NaiveDateTime::parse_str(str) {
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
        FieldType::TableRowField => match object.get("value") {
            Some(value) => match value {
                Value::String(str) => match UuidV4::parse_str(str) {
                    Ok(uuid) => Ok(LiteralValuePresenter::TableRowField(Some(uuid))),
                    Err(_err) => Err(DecodeError::InvalidValue {
                        field_type: FieldType::TableRowField,
                        value,
                    }),
                },
                Value::Null => Ok(LiteralValuePresenter::TableRowField(None)),
                value => Err(DecodeError::InvalidValue {
                    field_type: FieldType::TableRowField,
                    value,
                }),
            },
            None => Ok(LiteralValuePresenter::TableRowField(None)),
        },
        FieldType::UserBoundaryField => match object.get("value") {
            Some(value) => match value {
                Value::Object(array) => match UserBoundary::from_json(array) {
                    Ok(user_boundary) => Ok(LiteralValuePresenter::UserBoundaryField(Some(
                        user_boundary,
                    ))),
                    Err(_err) => Err(DecodeError::InvalidValue {
                        field_type: FieldType::UserBoundaryField,
                        value,
                    }),
                },
                Value::Null => Ok(LiteralValuePresenter::UserBoundaryField(None)),
                value => Err(DecodeError::InvalidValue {
                    field_type: FieldType::UserBoundaryField,
                    value,
                }),
            },
            None => Ok(LiteralValuePresenter::UserBoundaryField(None)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use time::macros::datetime;
    use uuid::uuid;

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

    // test table_row_field
    #[test]
    fn test_do_make_literal_table_row_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field",
                "value": "67e55044-10b1-426f-9247-bb680e5fe0c8"
            });

            let expected_uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::TableRowField, object);

            assert!(matches!(
                result,
                Ok(LiteralValuePresenter::TableRowField(Some(UuidV4(uuid)))) if uuid == expected_uuid
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field",
                "value": null
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::TableRowField, object);

            assert!(matches!(
                result,
                Ok(LiteralValuePresenter::TableRowField(None))
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field"
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::TableRowField, object);

            assert!(matches!(
                result,
                Ok(LiteralValuePresenter::TableRowField(None))
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "table_row_field",
                "value": "invalid"
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::TableRowField, object);

            assert!(matches!(
                result,
                Err(DecodeError::InvalidValue {
                    field_type: _,
                    value: _
                })
            ));
        }
    }

    // test user_boundary_field
    #[test]
    fn test_do_make_literal_user_boundary_field_presenter() {
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field",
                "value": {
                    "user_uuids": ["67e55044-10b1-426f-9247-bb680e5fe0c8"]
                }
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::UserBoundaryField, object);

            assert!(matches!(
                result,
                Ok(LiteralValuePresenter::UserBoundaryField(Some(UserBoundary {
                    user_uuids,
                    simple_department_uuids: _,
                    penetrating_department_uuids: _
                }))) if user_uuids.len() == 1
            ));
        }

        // null value
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field",
                "value": null
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::UserBoundaryField, object);

            assert!(matches!(
                result,
                Ok(LiteralValuePresenter::UserBoundaryField(None))
            ));
        }

        // value is not present
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field"
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::UserBoundaryField, object);

            assert!(matches!(
                result,
                Ok(LiteralValuePresenter::UserBoundaryField(None))
            ));
        }

        // invalid value
        {
            let json = json!({
                "type": "literal",
                "field_type": "user_boundary_field",
                "value": "invalid"
            });

            let object = json.as_object().unwrap();
            let result = do_make_literal_presenter(&FieldType::UserBoundaryField, object);

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
            let vp = LiteralValuePresenter::BooleanField(Some(true));
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "BOOLEAN_FIELD", "value": true});

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::BooleanField(None);
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "BOOLEAN_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    #[test]
    fn test_literal_checkbox_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::CheckboxField(Some(OptionsValue {
                options: vec![String::from("option1"), String::from("option2")],
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
            let vp = LiteralValuePresenter::CheckboxField(Some(OptionsValue {
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
    }

    #[test]
    fn test_literal_date_time_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::DateTimeField(Some(
                NaiveDateTime::parse_str("2020-01-01T00:00:00Z").unwrap(),
            ));
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "DATE_TIME_FIELD", "value": "2020-01-01T00:00:00"});

            assert!(str == expected.to_string());
        }

        // with ms
        {
            let vp = LiteralValuePresenter::DateTimeField(Some(
                NaiveDateTime::parse_str("2020-01-01T00:00:00.123456Z").unwrap(),
            ));
            let str = vp.to_json().to_string();
            let expected = json!({"type": "literal", "field_type": "DATE_TIME_FIELD", "value": "2020-01-01T00:00:00.123456"});

            assert!(str == expected.to_string());
        }

        {
            let vp = LiteralValuePresenter::DateTimeField(None);
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "DATE_TIME_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    #[test]
    fn test_literal_numeric_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::NumericField(Some(Number::from(123)));
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
            let vp = LiteralValuePresenter::NumericField(Number::from_f64(123.1 as f64));
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
            let vp = LiteralValuePresenter::NumericField(None);
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

    #[test]
    fn test_literal_radio_button_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::RadioButtonField(Some(OptionsValue {
                options: vec![String::from("option")],
                other: None,
            }));
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
            let vp = LiteralValuePresenter::RadioButtonField(Some(OptionsValue {
                options: vec![],
                other: Some(String::from("other")),
            }));
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
            let vp = LiteralValuePresenter::RadioButtonField(Some(OptionsValue {
                options: vec![],
                other: None,
            }));
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "RADIO_BUTTON_FIELD",
                "value": {"options": [], "other": null}
            });

            assert!(str == expected.to_string());
        }
    }

    #[test]
    fn test_literal_single_line_field_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::SingleLineField(Some(String::from("hello")));
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "SINGLE_LINE_FIELD", "value": "hello"});

            assert!(str == expected.to_string());
        }

        // null value
        {
            let vp = LiteralValuePresenter::SingleLineField(None);
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "SINGLE_LINE_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    #[test]
    fn test_literal_table_row_field_value_presenter_to_json() {
        {
            let uuid_str = "67e55044-10b1-426f-9247-bb680e5fe0c8";
            let vp =
                LiteralValuePresenter::TableRowField(Some(UuidV4::parse_str(uuid_str).unwrap()));
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "TABLE_ROW_FIELD", "value": uuid_str});

            assert!(
                str == expected.to_string(),
                "str: {}, expected: {}",
                str,
                expected
            );
        }

        // null value
        {
            let vp = LiteralValuePresenter::TableRowField(None);
            let str = vp.to_json().to_string();
            let expected =
                json!({"type": "literal", "field_type": "TABLE_ROW_FIELD", "value": null});

            assert!(str == expected.to_string());
        }
    }

    #[test]
    fn test_literal_user_boundary_value_presenter_to_json() {
        {
            let vp = LiteralValuePresenter::UserBoundaryField(Some(UserBoundary {
                user_uuids: vec![UuidV4::parse_str("00000000-0000-0000-0000-ffff00000000").unwrap()],
                simple_department_uuids: vec![
                    UuidV4::parse_str("00000000-0000-0000-0000-ffff00000001").unwrap(),
                    UuidV4::parse_str("00000000-0000-0000-0000-ffff00000002").unwrap(),
                ],
                penetrating_department_uuids: vec![
                    UuidV4::parse_str("00000000-0000-0000-0000-ffff00000003").unwrap(),
                    UuidV4::parse_str("00000000-0000-0000-0000-ffff00000004").unwrap(),
                    UuidV4::parse_str("00000000-0000-0000-0000-ffff00000005").unwrap(),
                ],
            }));
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
            let vp = LiteralValuePresenter::UserBoundaryField(None);
            let str = vp.to_json().to_string();
            let expected = json!({
                "type": "literal",
                "field_type": "USER_BOUNDARY_FIELD",
                "value": null
            });

            assert!(str == expected.to_string());
        }
    }
}
