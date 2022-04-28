use time::PrimitiveDateTime;
use uuid::Uuid;

pub type UuidV4 = Uuid;

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

pub struct OptionsValue {
    pub options: Vec<OptionValue>,
    pub other: Option<OptionValue>,
}

pub type NaiveDateTime = PrimitiveDateTime;

#[cfg(test)]
mod tests {
    use super::*;
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
}
