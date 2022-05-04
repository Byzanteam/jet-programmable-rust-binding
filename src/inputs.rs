use serde_json::Value;

use crate::value_presenter::{error::DecodeError, field_type::FieldType, ValuePresenter};

pub fn parse(args: &Value, types: Vec<FieldType>) -> Result<Vec<ValuePresenter>, DecodeError> {
    match args {
        Value::Array(list) => {
            let types_len = types.len();

            let pairs = types.into_iter().zip(list.iter());

            if pairs.len() != types_len {
                panic!("Invalid number of inputs");
            }

            let mut result: Vec<ValuePresenter> = Vec::new();

            for (field_type, value) in pairs {
                if value.is_object() {
                    match ValuePresenter::from_json(value) {
                        Ok(vp) => {
                            if vp.get_field_type() == field_type {
                                result.push(vp);
                            } else {
                                return Err(DecodeError::MismatchedFieldType {
                                    value_presenter: vp,
                                    field_type,
                                });
                            }
                        }
                        Err(error) => return Err(error),
                    }
                } else {
                    return Err(DecodeError::InvalidJsonObject(value));
                }
            }

            Ok(result)
        }
        value => Err(DecodeError::InvalidJsonObject(value)),
    }
}

#[cfg(test)]
mod tests {
    use crate::value_presenter::{field_value::BooleanFieldValue, literal::LiteralValuePresenter};

    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_valid_json() {
        let args = json!([
            {
                "type": "literal",
                "field_type": "boolean_field",
                "value": true
            }
        ]);

        let vps = parse(&args, vec![FieldType::BooleanField]).unwrap();

        assert!(matches!(
            vps.as_slice(),
            [ValuePresenter::Literal(
                LiteralValuePresenter::BooleanField(BooleanFieldValue::Value(true))
            )]
        ));
    }

    #[test]
    fn test_parse_valid_json_with_too_few_types() {
        let args = json!([
            {
                "type": "literal",
                "field_type": "boolean_field",
                "value": true
            },
            {
                "type": "literal",
                "field_type": "boolean_field",
                "value": true
            }
        ]);

        let vps = parse(&args, vec![FieldType::BooleanField]).unwrap();

        assert!(matches!(
            vps.as_slice(),
            [ValuePresenter::Literal(
                LiteralValuePresenter::BooleanField(BooleanFieldValue::Value(true))
            )]
        ));
    }

    #[test]
    fn test_parse_valid_json_with_mismatched_field_type() {
        let args = json!([
            {
                "type": "literal",
                "field_type": "boolean_field",
                "value": true
            }
        ]);

        let result = parse(&args, vec![FieldType::SingleLineField]);

        assert!(matches!(
            result,
            Err(DecodeError::MismatchedFieldType { .. })
        ));
    }

    #[test]
    fn test_parse_invalid_json() {
        let args = json!(
            {
                "type": "literal",
                "field_type": "boolean_field",
                "value": "123"
            }
        );

        let result = parse(&args, vec![FieldType::BooleanField]);

        assert!(matches!(result, Err(DecodeError::InvalidJsonObject(_))));
    }

    #[test]
    fn test_parse_invalid_value_presenter_json() {
        let args = json!([
            {
                "type": "literal",
                "field_type": "boolean_field",
                "value": "123"
            }
        ]);

        let result = parse(&args, vec![FieldType::BooleanField]);

        assert!(matches!(result, Err(DecodeError::InvalidValue { .. })));
    }

    #[test]
    #[should_panic(expected = "Invalid number of inputs")]
    fn test_parse_valid_json_with_too_much_types() {
        let args = json!([
            {
                "type": "literal",
                "field_type": "boolean_field",
                "value": "123"
            }
        ]);

        let _result = parse(
            &args,
            vec![FieldType::BooleanField, FieldType::BooleanField],
        );
    }
}
