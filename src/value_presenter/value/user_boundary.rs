use serde_json::{json, Value};

use super::{json_codec::JsonCodec, uuid::Uuid};

#[derive(Debug)]
pub enum ParseUserBoundaryError {
    InvalidJson,
    InvalidUserUuids,
    InvalidSimpleDepartmentUuids,
    InvalidPenetratingDepartmentUuids,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserBoundary {
    pub user_uuids: Vec<Uuid>,
    pub simple_department_uuids: Vec<Uuid>,
    pub penetrating_department_uuids: Vec<Uuid>,
}

impl UserBoundary {
    pub fn is_empty(&self) -> bool {
        self.user_uuids.len() == 0
            && self.simple_department_uuids.len() == 0
            && self.penetrating_department_uuids.len() == 0
    }
}

impl JsonCodec for UserBoundary {
    type Err = ParseUserBoundaryError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        if !value.is_object() {
            return Err(ParseUserBoundaryError::InvalidJson);
        }

        let mut user_uuids = vec![];
        let mut simple_department_uuids = vec![];
        let mut penetrating_department_uuids = vec![];

        if let Some(user_uuids_json) = value.get("user_uuids") {
            match uuids_from_json(user_uuids_json) {
                Ok(uuids) => user_uuids = uuids,
                Err(_msg) => return Err(ParseUserBoundaryError::InvalidUserUuids),
            }
        }

        if let Some(simple_department_uuids_json) = value.get("simple_department_uuids") {
            match uuids_from_json(simple_department_uuids_json) {
                Ok(uuids) => simple_department_uuids = uuids,
                Err(_msg) => return Err(ParseUserBoundaryError::InvalidSimpleDepartmentUuids),
            }
        }

        if let Some(penetrating_department_uuids_json) = value.get("penetrating_department_uuids") {
            match uuids_from_json(penetrating_department_uuids_json) {
                Ok(uuids) => penetrating_department_uuids = uuids,
                Err(_msg) => return Err(ParseUserBoundaryError::InvalidPenetratingDepartmentUuids),
            }
        }

        Ok(UserBoundary {
            user_uuids,
            simple_department_uuids,
            penetrating_department_uuids,
        })
    }

    fn to_json(&self) -> Value {
        json!({
            "user_uuids": uuids_to_json(&self.user_uuids),
            "simple_department_uuids": uuids_to_json(&self.simple_department_uuids),
            "penetrating_department_uuids": uuids_to_json(&self.penetrating_department_uuids),
        })
    }
}

fn uuids_from_json(json: &Value) -> Result<Vec<Uuid>, &'static str> {
    let mut uuids = vec![];

    match json {
        Value::Array(array) => {
            for uuid_json in array {
                match Uuid::from_json(uuid_json) {
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

fn uuids_to_json(uuids: &[Uuid]) -> Vec<Value> {
    uuids.iter().map(|uuid| uuid.to_json()).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_is_empty() {
        {
            let empty = UserBoundary {
                user_uuids: vec![],
                simple_department_uuids: vec![],
                penetrating_department_uuids: vec![],
            };

            assert!(empty.is_empty());
        }

        {
            let uuid = Uuid::new("00000000-0000-0000-0000-ffff00000000").unwrap();

            let user_boundary = UserBoundary {
                user_uuids: vec![uuid],
                simple_department_uuids: vec![],
                penetrating_department_uuids: vec![],
            };

            assert!(!user_boundary.is_empty());
        }
    }

    #[test]
    fn test_from_json() {
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

            let user_boundary = UserBoundary::from_json(&json);
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
                "user_uuids": [],
                "simple_department_uuids": [],
                "penetrating_department_uuids": [],
            });

            let user_boundary = UserBoundary::from_json(&json);
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

            let user_boundary = UserBoundary::from_json(&json);
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
    fn test_from_invalid_json() {
        {
            let json = json!([]);

            let user_boundary = UserBoundary::from_json(&json);
            assert!(matches!(
                user_boundary,
                Err(ParseUserBoundaryError::InvalidJson)
            ));
        }

        // invalid user_uuids
        {
            let json = json!({"user_uuids": "invalid"});

            let user_boundary = UserBoundary::from_json(&json);
            assert!(matches!(
                user_boundary,
                Err(ParseUserBoundaryError::InvalidUserUuids)
            ));
        }

        // invalid simple_department_uuids
        {
            let json = json!({"simple_department_uuids": "invalid"});

            let user_boundary = UserBoundary::from_json(&json);
            assert!(matches!(
                user_boundary,
                Err(ParseUserBoundaryError::InvalidSimpleDepartmentUuids)
            ));
        }

        // invalid penetrating_department_uuids
        {
            let json = json!({"penetrating_department_uuids": "invalid"});

            let user_boundary = UserBoundary::from_json(&json);
            assert!(matches!(
                user_boundary,
                Err(ParseUserBoundaryError::InvalidPenetratingDepartmentUuids)
            ));
        }
    }

    #[test]
    fn test_to_json() {
        {
            let user_boundary = UserBoundary {
                user_uuids: vec![],
                simple_department_uuids: vec![],
                penetrating_department_uuids: vec![],
            };

            let json = json!({
                "user_uuids": [],
                "simple_department_uuids": [],
                "penetrating_department_uuids": [],
            });

            assert!(user_boundary.to_json() == json);
        }

        {
            let user_boundary = UserBoundary {
                user_uuids: vec![Uuid::new("00000000-0000-0000-0000-ffff00000000").unwrap()],
                simple_department_uuids: vec![
                    Uuid::new("00000000-0000-0000-0000-ffff00000001").unwrap(),
                    Uuid::new("00000000-0000-0000-0000-ffff00000002").unwrap(),
                ],
                penetrating_department_uuids: vec![
                    Uuid::new("00000000-0000-0000-0000-ffff00000003").unwrap(),
                    Uuid::new("00000000-0000-0000-0000-ffff00000004").unwrap(),
                    Uuid::new("00000000-0000-0000-0000-ffff00000005").unwrap(),
                ],
            };

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

            assert!(user_boundary.to_json() == json);
        }
    }
}
