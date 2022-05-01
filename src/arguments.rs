use serde_json::Value;

use crate::value_presenter::{error::DecodeError, field_type::FieldType, ValuePresenter};

pub fn parse(args: &Value, types: Vec<FieldType>) -> Result<Vec<ValuePresenter>, DecodeError> {
    match args {
        Value::Array(list) => {
            let types_len = types.len();

            let pairs = types.into_iter().zip(list.iter());

            if pairs.len() != types_len {
                panic!("Invalid number of arguments");
            }

            let mut result: Vec<ValuePresenter> = Vec::new();

            for (field_type, value) in pairs {
                match value {
                    Value::Object(obj) => match ValuePresenter::from_json(obj) {
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
                    },
                    value => return Err(DecodeError::InvalidJsonObject(value)),
                }
            }

            Ok(result)
        }
        value => Err(DecodeError::InvalidJsonObject(value)),
    }
}

#[cfg(test)]
mod tests {
    use crate::value_presenter::literal::LiteralValuePresenter;

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

        let result = parse(&args, vec![FieldType::BooleanField]);

        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().len() == 1);

        assert!(matches!(
            result.unwrap().as_slice(),
            [ValuePresenter::Literal(
                LiteralValuePresenter::BooleanField(Some(true))
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

        let result = parse(&args, vec![FieldType::BooleanField]);

        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().len() == 1);

        assert!(matches!(
            result.unwrap().as_slice(),
            [ValuePresenter::Literal(
                LiteralValuePresenter::BooleanField(Some(true))
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
    #[should_panic(expected = "Invalid number of arguments")]
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
