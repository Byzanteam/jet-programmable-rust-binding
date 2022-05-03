use serde_json::{json, Value};

use super::json_codec::JsonCodec;

type OptionValue = String;

#[derive(Debug, Clone)]
pub struct OptionsValue {
    pub options: Vec<OptionValue>,
    pub other: Option<OptionValue>,
}

#[derive(Debug)]
pub enum ParseOptionsValueError {
    InvalidJson,
    InvalidOptions,
    InvalidOther,
}

impl JsonCodec for OptionsValue {
    type Err = ParseOptionsValueError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        if !value.is_object() {
            return Err(ParseOptionsValueError::InvalidJson);
        }

        let mut options_vec = vec![];
        let mut other_option = None;

        match value.get("options") {
            Some(Value::Array(options)) => {
                for option in options {
                    match option {
                        Value::String(option) => {
                            options_vec.push(option.to_string());
                        }
                        _ => {
                            return Err(ParseOptionsValueError::InvalidOptions);
                        }
                    }
                }
            }
            Some(Value::Null) => (),
            Some(_) => {
                return Err(ParseOptionsValueError::InvalidOptions);
            }
            None => (),
        }

        match value.get("other") {
            Some(Value::String(other)) => {
                other_option = Some(other.to_string());
            }
            Some(Value::Null) => (),
            Some(_) => {
                return Err(ParseOptionsValueError::InvalidOther);
            }
            None => (),
        }

        Ok(OptionsValue {
            options: options_vec,
            other: other_option,
        })
    }

    fn to_json(&self) -> Value {
        json!({ "options": self.options, "other": self.other, })
    }
}

impl OptionsValue {
    pub fn count_options(&self) -> usize {
        self.options.len() + self.other.is_some() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_options_value() {
        let options_value = OptionsValue {
            options: vec![String::from("option1"), String::from("option2")],
            other: None,
        };

        assert!(options_value.options.len() != 0);
    }

    #[test]
    fn test_new_options_value_from_str() {
        let option1 = String::from("option1");

        let options_value = OptionsValue {
            options: vec![option1],
            other: None,
        };

        assert!(options_value.options.len() != 0);
    }

    #[test]
    fn test_new_options_value_with_other() {
        let other = Some(String::from("other"));

        let options_value = OptionsValue {
            options: vec![],
            other,
        };

        assert!(options_value.options.len() == 0);
        assert!(options_value.other.unwrap() == "other");
    }

    #[test]
    fn test_options_value_from_json() {
        {
            let json = json!({
                "options": ["option1", "option2"],
                "other": "other"
            });

            let options_value = OptionsValue::from_json(&json).unwrap();

            let option1 = String::from("option1");
            let option2 = String::from("option2");
            let other = String::from("other");

            assert!(options_value.options.len() == 2);
            assert!(options_value.options[0] == option1);
            assert!(options_value.options[1] == option2);
            assert!(options_value.other == Some(other));
        }

        // empty options
        {
            let json = json!({
                "options": [],
                "other": "other"
            });

            let options_value = OptionsValue::from_json(&json).unwrap();

            let other = String::from("other");

            assert!(options_value.options.len() == 0);
            assert!(options_value.other == Some(other));
        }

        // null options
        {
            let json = json!({
                "options": null,
                "other": "other"
            });

            let options_value = OptionsValue::from_json(&json).unwrap();

            let other = String::from("other");

            assert!(options_value.options.len() == 0);
            assert!(options_value.other == Some(other));
        }

        // options is not present
        {
            let json = json!({
                "other": "other"
            });

            let options_value = OptionsValue::from_json(&json).unwrap();

            let other = String::from("other");

            assert!(options_value.options.len() == 0);
            assert!(options_value.other == Some(other));
        }

        // null other
        {
            let json = json!({
                "options": [],
                "other": null
            });

            let options_value = OptionsValue::from_json(&json).unwrap();

            assert!(options_value.other == None);
        }

        // other is not present
        {
            let json = json!({
                "options": [],
            });

            let options_value = OptionsValue::from_json(&json).unwrap();

            assert!(options_value.other == None);
        }

        // invalid options
        {
            let json = json!({
                "options": "option",
                "other": "other"
            });

            let result = OptionsValue::from_json(&json);

            assert!(matches!(result, Err(_)));
        }

        // invalid other
        {
            let json = json!({
                "options": [],
                "other": 123
            });

            let result = OptionsValue::from_json(&json);

            assert!(matches!(result, Err(_)));
        }
    }

    #[test]
    fn test_count_options_of_options_value() {
        // only options
        {
            let options_value = OptionsValue {
                options: vec![String::from("option1"), String::from("option2")],
                other: None,
            };

            assert!(options_value.count_options() == 2);
        }

        // only other
        {
            let options_value = OptionsValue {
                options: vec![],
                other: Some(String::from("other")),
            };

            assert!(options_value.count_options() == 1);
        }

        // options and other
        {
            let options_value = OptionsValue {
                options: vec![String::from("option1"), String::from("option2")],
                other: Some(String::from("other")),
            };

            assert!(options_value.count_options() == 3);
        }

        // empty
        {
            let options_value = OptionsValue {
                options: vec![],
                other: None,
            };

            assert!(options_value.count_options() == 0);
        }
    }
}
