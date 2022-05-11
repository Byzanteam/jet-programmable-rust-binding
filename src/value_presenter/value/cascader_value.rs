use serde_json::{json, Value};

use super::{json_codec::JsonCodec, uuid::Uuid};

#[derive(Debug)]
pub enum ParseCascaderValueError {
    InvalidJson,
    InvalidOptionsTableUuid,
    InvalidRowUuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CascaderValue {
    pub options_table_uuid: Uuid,
    pub row_uuid: Uuid,
}

impl CascaderValue {
    pub fn new(options_table_uuid: Uuid, row_uuid: Uuid) -> Self {
        Self {
            options_table_uuid,
            row_uuid,
        }
    }
}

impl JsonCodec for CascaderValue {
    type Err = ParseCascaderValueError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        if !value.is_object() {
            return Err(ParseCascaderValueError::InvalidJson);
        }

        let options_table_uuid = match value.get("options_table_uuid") {
            Some(options_table_uuid) => match Uuid::from_json(options_table_uuid) {
                Ok(uuid) => uuid,
                Err(_) => return Err(ParseCascaderValueError::InvalidOptionsTableUuid),
            },
            None => return Err(ParseCascaderValueError::InvalidOptionsTableUuid),
        };

        let row_uuid = match value.get("row_uuid") {
            Some(row_uuid) => match Uuid::from_json(row_uuid) {
                Ok(uuid) => uuid,
                Err(_) => return Err(ParseCascaderValueError::InvalidRowUuid),
            },
            None => return Err(ParseCascaderValueError::InvalidRowUuid),
        };

        Ok(CascaderValue {
            options_table_uuid,
            row_uuid,
        })
    }

    fn to_json(&self) -> Value {
        json!({
            "options_table_uuid": self.options_table_uuid.to_json(),
            "row_uuid": self.row_uuid.to_json(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_from_json() {
        {
            let json = json!({
                "options_table_uuid": "00000000-0000-0000-0000-ffff00000000",
                "row_uuid": "00000000-0000-0000-0000-ffff00000001"
            });

            let user_boundary = CascaderValue::from_json(&json);
            assert!(matches!(
                user_boundary,
                Ok(CascaderValue {
                    options_table_uuid,
                    row_uuid,
                }) if options_table_uuid == Uuid("00000000-0000-0000-0000-ffff00000000".to_string()) && row_uuid == Uuid("00000000-0000-0000-0000-ffff00000001".to_string())
            ));
        }
    }

    #[test]
    fn test_from_invalid_json() {
        {
            let json = json!([]);

            let user_boundary = CascaderValue::from_json(&json);
            assert!(matches!(
                user_boundary,
                Err(ParseCascaderValueError::InvalidJson)
            ));
        }

        // invalid options_table_uuid
        {
            let json = json!({"options_table_uuid": "invalid", "row_uuid": "00000000-0000-0000-0000-ffff00000001"});

            let user_boundary = CascaderValue::from_json(&json);
            assert!(matches!(
                user_boundary,
                Err(ParseCascaderValueError::InvalidOptionsTableUuid)
            ));
        }

        // invalid row_uuid
        {
            let json = json!({"options_table_uuid": "00000000-0000-0000-0000-ffff00000000", "row_uuid": "invalid"});

            let user_boundary = CascaderValue::from_json(&json);
            assert!(matches!(
                user_boundary,
                Err(ParseCascaderValueError::InvalidRowUuid)
            ));
        }
    }

    #[test]
    fn test_to_json() {
        {
            let user_boundary = CascaderValue {
                options_table_uuid: Uuid::new("00000000-0000-0000-0000-ffff00000000").unwrap(),
                row_uuid: Uuid::new("00000000-0000-0000-0000-ffff00000001").unwrap(),
            };

            let json = json!({
                "options_table_uuid": "00000000-0000-0000-0000-ffff00000000",
                "row_uuid": "00000000-0000-0000-0000-ffff00000001"
            });

            assert!(user_boundary.to_json() == json);
        }
    }
}
