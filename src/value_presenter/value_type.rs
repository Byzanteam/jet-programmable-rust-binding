use serde_json::{json, Map, Value};
use time::{
    format_description::well_known::Rfc3339, macros::format_description, PrimitiveDateTime,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UuidV4(pub Uuid);

#[derive(Debug)]
pub struct ParseUuidV4Error;

impl UuidV4 {
    pub fn parse_str(str: &str) -> Result<Self, ParseUuidV4Error> {
        match Uuid::parse_str(str) {
            Ok(uuid) => Ok(UuidV4(uuid)),
            Err(_err) => Err(ParseUuidV4Error),
        }
    }

    pub fn to_str(&self) -> String {
        self.0.hyphenated().to_string()
    }
}

#[derive(Debug, Clone)]
pub struct UserBoundary {
    pub user_uuids: Vec<UuidV4>,
    pub simple_department_uuids: Vec<UuidV4>,
    pub penetrating_department_uuids: Vec<UuidV4>,
}

#[derive(Debug)]
pub enum ParseUserBoundaryError {
    UserUuids,
    SimpleDepartmentUuids,
    PenetratingDepartmentUuids,
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

    pub fn from_json(json: &Map<String, Value>) -> Result<Self, ParseUserBoundaryError> {
        let mut user_uuids = vec![];
        let mut simple_department_uuids = vec![];
        let mut penetrating_department_uuids = vec![];

        if let Some(user_uuids_json) = json.get("user_uuids") {
            match Self::extract_uuids_from_json(user_uuids_json) {
                Ok(uuids) => user_uuids = uuids,
                Err(_msg) => return Err(ParseUserBoundaryError::UserUuids),
            }
        }

        if let Some(simple_department_uuids_json) = json.get("simple_department_uuids") {
            match Self::extract_uuids_from_json(simple_department_uuids_json) {
                Ok(uuids) => simple_department_uuids = uuids,
                Err(_msg) => return Err(ParseUserBoundaryError::SimpleDepartmentUuids),
            }
        }

        if let Some(penetrating_department_uuids_json) = json.get("penetrating_department_uuids") {
            match Self::extract_uuids_from_json(penetrating_department_uuids_json) {
                Ok(uuids) => penetrating_department_uuids = uuids,
                Err(_msg) => return Err(ParseUserBoundaryError::PenetratingDepartmentUuids),
            }
        }

        Ok(UserBoundary {
            user_uuids,
            simple_department_uuids,
            penetrating_department_uuids,
        })
    }

    fn extract_uuids_from_json(json: &Value) -> Result<Vec<UuidV4>, &'static str> {
        let mut uuids = vec![];

        match json {
            Value::Array(array) => {
                for uuid_json in array {
                    match UuidV4::parse_str(uuid_json.as_str().unwrap()) {
                        Ok(uuid) => uuids.push(uuid),
                        Err(_err) => return Err("invalid uuid"),
                    }
                }
            }
            Value::Null => {}
            _ => return Err("invalid uuid"),
        }

        Ok(uuids)
    }

    pub fn to_json(&self) -> Value {
        json!({
            "user_uuids": Self::uuids_to_str_vec(&self.user_uuids),
            "simple_department_uuids": Self::uuids_to_str_vec(&self.simple_department_uuids),
            "penetrating_department_uuids": Self::uuids_to_str_vec(&self.penetrating_department_uuids),
        })
    }

    fn uuids_to_str_vec(uuids: &[UuidV4]) -> Vec<String> {
        uuids.iter().map(|uuid| uuid.to_str()).collect::<Vec<_>>()
    }
}

type OptionValue = String;

#[derive(Debug, Clone)]
pub struct OptionsValue {
    pub options: Vec<OptionValue>,
    pub other: Option<OptionValue>,
}

#[derive(Debug)]
pub enum ParseOptionsValueError {
    Options,
    Other,
}

impl OptionsValue {
    pub fn count_options(&self) -> usize {
        self.options.len() + self.other.is_some() as usize
    }

    pub fn from_json(map: &Map<String, Value>) -> Result<Self, ParseOptionsValueError> {
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
                            return Err(ParseOptionsValueError::Options);
                        }
                    }
                }
            }
            Some(Value::Null) => (),
            Some(_) => {
                return Err(ParseOptionsValueError::Options);
            }
            None => (),
        }

        match map.get("other") {
            Some(Value::String(other)) => {
                other_option = Some(other.to_string());
            }
            Some(Value::Null) => (),
            Some(_) => {
                return Err(ParseOptionsValueError::Other);
            }
            None => (),
        }

        Ok(OptionsValue {
            options: options_vec,
            other: other_option,
        })
    }

    pub fn to_json(&self) -> Value {
        json!({ "options": self.options, "other": self.other, })
    }
}

#[derive(Debug, Clone)]
pub struct NaiveDateTime(pub PrimitiveDateTime);

#[derive(Debug)]
pub struct ParseNaiveDateTimeError;

impl NaiveDateTime {
    pub fn parse_str(str: &str) -> Result<Self, ParseNaiveDateTimeError> {
        // ISO 8601 string `2022-04-29T07:34:10.420159`

        let str = Self::normalize(str);

        match PrimitiveDateTime::parse(&str, &Rfc3339) {
            Ok(date) => Ok(NaiveDateTime(date)),
            Err(_) => Err(ParseNaiveDateTimeError),
        }
    }

    fn normalize(str: &str) -> String {
        if str.ends_with('Z') {
            str.to_string()
        } else {
            str.to_string() + "Z"
        }
    }

    pub fn to_str(&self) -> String {
        let format = if self.0.nanosecond() == 0 {
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]")
        } else {
            format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:1+]"
            )
        };

        self.0.format(&format).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use time::macros::datetime;
    use uuid::uuid;

    #[test]
    fn test_user_boundary_is_empty() {
        let empty = UserBoundary {
            user_uuids: vec![],
            simple_department_uuids: vec![],
            penetrating_department_uuids: vec![],
        };

        assert!(empty.is_empty());

        let uuid = UuidV4(uuid!("00000000-0000-0000-0000-ffff00000000"));

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
    fn test_user_boundary_from_json() {
        {
            let json = json!({
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
            });

            let user_boundary = UserBoundary::from_json(json.as_object().unwrap());
            assert!(matches!(
                user_boundary,
                Ok(UserBoundary {
                    user_uuids,
                    simple_department_uuids,
                    penetrating_department_uuids
                }) if user_uuids.len() == 1 && simple_department_uuids.len() == 2 && penetrating_department_uuids.len() == 3
            ));
        }

        // empty uuids
        {
            let json = json!({
                "user_uuids": [
                ],
                "simple_department_uuids": [
                ],
                "penetrating_department_uuids": [
                ],
            });

            let user_boundary = UserBoundary::from_json(json.as_object().unwrap());
            assert!(matches!(
                user_boundary,
                Ok(UserBoundary {
                    user_uuids,
                    simple_department_uuids,
                    penetrating_department_uuids
                }) if user_uuids.len() == 0 && simple_department_uuids.len() == 0 && penetrating_department_uuids.len() == 0
            ));
        }

        // uuids is not present
        {
            let json = json!({});

            let user_boundary = UserBoundary::from_json(json.as_object().unwrap());
            assert!(matches!(
                user_boundary,
                Ok(UserBoundary {
                    user_uuids,
                    simple_department_uuids,
                    penetrating_department_uuids
                }) if user_uuids.len() == 0 && simple_department_uuids.len() == 0 && penetrating_department_uuids.len() == 0
            ));
        }
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

    #[test]
    fn test_make_naive_date_time_from_str() {
        let expected = datetime!(2022-04-29 07:34:10.420159);

        {
            let str = "2022-04-29T07:34:10.420159";
            let ndt = NaiveDateTime::parse_str(&str);

            assert_eq!(ndt.unwrap().0, expected);
        }

        // with timezone
        {
            let str = "2022-04-29T07:34:10.420159Z";
            let ndt = NaiveDateTime::parse_str(&str);

            assert_eq!(ndt.unwrap().0, expected);
        }

        // without ms
        {
            let str = "2022-04-29T07:34:10Z";
            let ndt = NaiveDateTime::parse_str(&str);
            let expected = datetime!(2022-04-29 07:34:10);

            assert_eq!(ndt.unwrap().0, expected);
        }

        // invalid str
        {
            let str = "2022-04-29 07:34";
            let ndt = NaiveDateTime::parse_str(&str);

            assert!(matches!(ndt, Err(_)));
        }
    }

    #[test]
    fn test_naive_date_time_to_str() {
        {
            let str = "2022-04-29T07:34:10";
            let ndt = NaiveDateTime::parse_str(&str);

            assert_eq!(ndt.unwrap().to_str(), str);
        }

        // with nanosecond
        {
            let str = "2022-04-29T07:34:10.420159";
            let ndt = NaiveDateTime::parse_str(&str);

            assert_eq!(ndt.unwrap().to_str(), str);
        }
    }

    #[test]
    fn test_make_uuid_v4() {
        {
            let result = UuidV4::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8");
            let expected = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

            assert!(matches!(result, Ok(UuidV4(uuid)) if uuid == expected));
        }

        // invalid str
        {
            let result = UuidV4::parse_str("67e5504410b1-426f-9247-bb680e5fe0c8");

            assert!(matches!(result, Err(_)));
        }
    }

    #[test]
    fn test_uuid_to_str() {
        // simple
        {
            let result = UuidV4::parse_str("67e5504410b1426f9247bb680e5fe0c8");
            let expected = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

            assert!(matches!(result, Ok(UuidV4(uuid)) if uuid == expected));
        }

        // hyphenated
        {
            let result = UuidV4::parse_str("67e5504410b1-426f-9247-bb680e5fe0c8");

            assert!(matches!(result, Err(_)));
        }
    }
}
