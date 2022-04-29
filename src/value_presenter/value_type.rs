use serde_json::{Map, Value};
use time::PrimitiveDateTime;
use uuid::Uuid;

pub type UuidV4 = Uuid;

#[derive(Debug)]
pub struct UserBoundary {
    user_uuids: Vec<UuidV4>,
    simple_department_uuids: Vec<UuidV4>,
    penetrating_department_uuids: Vec<UuidV4>,
}

impl UserBoundary {
    pub fn empty() -> Self {
        UserBoundary {
            user_uuids: vec![],
            simple_department_uuids: vec![],
            penetrating_department_uuids: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.user_uuids.len() == 0
            && self.simple_department_uuids.len() == 0
            && self.penetrating_department_uuids.len() == 0
    }
}

type OptionValue = String;

#[derive(Debug)]
pub struct OptionsValue {
    pub options: Vec<OptionValue>,
    pub other: Option<OptionValue>,
}

impl OptionsValue {
    pub fn from_json(map: &Map<String, Value>) -> Result<Self, &'static str> {
        let mut options_vec = vec![];
        let mut other_option = None;

        match map.get("options") {
            Some(Value::Array(options)) => {
                for option in options {
                    match option {
                        Value::String(option) => {
                            options_vec.push(option.to_string());
                        }
                        _ => {
                            return Err("options must be an array of strings");
                        }
                    }
                }
            }
            Some(Value::Null) => (),
            Some(_) => {
                return Err("options is not an array");
            }
            None => (),
        }

        match map.get("other") {
            Some(Value::String(other)) => {
                other_option = Some(other.to_string());
            }
            Some(Value::Null) => (),
            Some(_) => {
                return Err("other is not a string");
            }
            None => (),
        }

        Ok(OptionsValue {
            options: options_vec,
            other: other_option,
        })
    }
}

pub type NaiveDateTime = PrimitiveDateTime;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::{uuid, Uuid};

    #[test]
    fn test_user_boundary_is_empty() {
        let empty = UserBoundary {
            user_uuids: vec![],
            simple_department_uuids: vec![],
            penetrating_department_uuids: vec![],
        };

        assert!(empty.is_empty());

        let uuid: Uuid = uuid!("00000000-0000-0000-0000-ffff00000000");

        let user_boundary = UserBoundary {
            user_uuids: vec![uuid],
            simple_department_uuids: vec![],
            penetrating_department_uuids: vec![],
        };

        assert!(!user_boundary.is_empty());
    }

    #[test]
    fn test_user_boundary_empty() {
        let empty = UserBoundary::empty();

        assert!(empty.is_empty());
    }

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

            let object = json.as_object().unwrap();
            let options_value = OptionsValue::from_json(object).unwrap();

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

            let object = json.as_object().unwrap();
            let options_value = OptionsValue::from_json(object).unwrap();

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

            let object = json.as_object().unwrap();
            let options_value = OptionsValue::from_json(object).unwrap();

            let other = String::from("other");

            assert!(options_value.options.len() == 0);
            assert!(options_value.other == Some(other));
        }

        // options is not present
        {
            let json = json!({
                "other": "other"
            });

            let object = json.as_object().unwrap();
            let options_value = OptionsValue::from_json(object).unwrap();

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

            let object = json.as_object().unwrap();
            let options_value = OptionsValue::from_json(object).unwrap();

            assert!(options_value.other == None);
        }

        // other is not present
        {
            let json = json!({
                "options": [],
            });

            let object = json.as_object().unwrap();
            let options_value = OptionsValue::from_json(object).unwrap();

            assert!(options_value.other == None);
        }

        // invalid options
        {
            let json = json!({
                "options": "option",
                "other": "other"
            });

            let object = json.as_object().unwrap();
            let result = OptionsValue::from_json(object);

            assert!(matches!(result, Err(_)));
        }

        // invalid other
        {
            let json = json!({
                "options": [],
                "other": 123
            });

            let object = json.as_object().unwrap();
            let result = OptionsValue::from_json(object);

            assert!(matches!(result, Err(_)));
        }
    }
}
