use serde_json::{json, Value};

use super::{json_codec::JsonCodec, uuid::Uuid};

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    DatabaseTable,
    DatabaseRow,

    WorkflowCase,
    WorkflowWorkflow,
    WorkflowVersionedWorkflow,

    IdentityAccessUser,
    IdentityAccessDepartment,
}

pub struct ParseResourceTypeError;

impl ResourceType {
    pub fn parse_str(s: &str) -> Result<Self, ParseResourceTypeError> {
        match s {
            "database_table" | "DATABASE_TABLE" => Ok(ResourceType::DatabaseTable),
            "database_row" | "DATABASE_ROW" => Ok(ResourceType::DatabaseRow),

            "workflow_case" | "WORKFLOW_CASE" => Ok(ResourceType::WorkflowCase),
            "workflow_workflow" | "WORKFLOW_WORKFLOW" => Ok(ResourceType::WorkflowWorkflow),
            "workflow_versioned_workflow" | "WORKFLOW_VERSIONED_WORKFLOW" => {
                Ok(ResourceType::WorkflowVersionedWorkflow)
            }

            "identity_access_user" | "IDENTITY_ACCESS_USER" => Ok(ResourceType::IdentityAccessUser),
            "identity_access_department" | "IDENTITY_ACCESS_DEPARTMENT" => {
                Ok(ResourceType::IdentityAccessDepartment)
            }
            _ => Err(ParseResourceTypeError),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ResourceType::DatabaseTable => "DATABASE_TABLE",
            ResourceType::DatabaseRow => "DATABASE_ROW",

            ResourceType::WorkflowCase => "WORKFLOW_CASE",
            ResourceType::WorkflowWorkflow => "WORKFLOW_WORKFLOW",
            ResourceType::WorkflowVersionedWorkflow => "WORKFLOW_VERSIONED_WORKFLOW",

            ResourceType::IdentityAccessUser => "IDENTITY_ACCESS_USER",
            ResourceType::IdentityAccessDepartment => "IDENTITY_ACCESS_DEPARTMENT",
        }
    }
}

#[derive(Debug)]
pub enum ParseRelationValueError {
    InvalidJson,
    InvalidResourceType,
    InvalidResourceUuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelationValue {
    pub resource_type: ResourceType,
    pub resource_uuid: Uuid,
}

impl RelationValue {
    pub fn new(resource_type: ResourceType, resource_uuid: Uuid) -> Self {
        Self {
            resource_type,
            resource_uuid,
        }
    }
}

impl JsonCodec for RelationValue {
    type Err = ParseRelationValueError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        if !value.is_object() {
            return Err(ParseRelationValueError::InvalidJson);
        }

        let resource_type = match value.get("type") {
            Some(object_uuid) => match object_uuid.as_str() {
                Some(s) => match ResourceType::parse_str(s) {
                    Ok(resource_type) => resource_type,
                    Err(_) => return Err(ParseRelationValueError::InvalidResourceType),
                },
                None => return Err(ParseRelationValueError::InvalidResourceType),
            },
            None => return Err(ParseRelationValueError::InvalidResourceType),
        };

        let resource_uuid = match value.get("uuid") {
            Some(uuid_json) => match Uuid::from_json(uuid_json) {
                Ok(uuid) => uuid,
                Err(_) => return Err(ParseRelationValueError::InvalidResourceUuid),
            },
            None => return Err(ParseRelationValueError::InvalidResourceUuid),
        };

        Ok(RelationValue {
            resource_type,
            resource_uuid,
        })
    }

    fn to_json(&self) -> Value {
        json!({
            "type": self.resource_type.to_str(),
            "uuid": self.resource_uuid.to_json(),
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
                "type": "database_table",
                "uuid": "00000000-0000-0000-0000-ffff00000000",
            });

            let relation_value = RelationValue::from_json(&json);
            assert!(matches!(
                relation_value,
                Ok(RelationValue {
                    resource_type: ResourceType::DatabaseTable,
                    resource_uuid,
                }) if resource_uuid == Uuid("00000000-0000-0000-0000-ffff00000000".to_string())
            ));
        }

        // UPPER_CASE type
        {
            let json = json!({
                "type": "DATABASE_TABLE",
                "uuid": "00000000-0000-0000-0000-ffff00000000",
            });

            let relation_value = RelationValue::from_json(&json);
            assert!(matches!(
                relation_value,
                Ok(RelationValue {
                    resource_type: ResourceType::DatabaseTable,
                    resource_uuid,
                }) if resource_uuid == Uuid("00000000-0000-0000-0000-ffff00000000".to_string())
            ));
        }
    }

    #[test]
    fn test_from_invalid_json() {
        {
            let json = json!([]);

            let relation_value = RelationValue::from_json(&json);
            assert!(matches!(
                relation_value,
                Err(ParseRelationValueError::InvalidJson)
            ));
        }

        // invalid type
        {
            let json =
                json!({"type": "invalid_type", "uuid": "00000000-0000-0000-0000-ffff00000000"});

            let relation_value = RelationValue::from_json(&json);
            assert!(matches!(
                relation_value,
                Err(ParseRelationValueError::InvalidResourceType)
            ));
        }

        // invalid uuid
        {
            let json = json!({"type": "database_table", "uuid": "invalid_uuid"});

            let relation_value = RelationValue::from_json(&json);
            assert!(matches!(
                relation_value,
                Err(ParseRelationValueError::InvalidResourceUuid)
            ));
        }
    }

    #[test]
    fn test_to_json() {
        {
            let relation_value = RelationValue {
                resource_type: ResourceType::DatabaseTable,
                resource_uuid: Uuid("00000000-0000-0000-0000-ffff00000000".to_string()),
            };

            let json = json!({
                "type": "DATABASE_TABLE",
                "uuid": "00000000-0000-0000-0000-ffff00000000",
            });

            assert!(relation_value.to_json() == json);
        }
    }
}
