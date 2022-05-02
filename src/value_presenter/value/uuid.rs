use serde_json::Value;
use uuid::Uuid as UuidLib;

use super::json_codec::JsonCodec;

#[derive(Debug, Clone)]
pub struct Uuid(pub String);

#[derive(Debug)]
pub struct ParseUuidError;

fn parse_str(str: &str) -> Result<Uuid, ParseUuidError> {
    match UuidLib::parse_str(str) {
        Ok(uuid) => Ok(Uuid(uuid.hyphenated().to_string())),
        Err(_err) => Err(ParseUuidError),
    }
}

impl JsonCodec for Uuid {
    type Err = ParseUuidError;

    fn from_json(value: &Value) -> Result<Uuid, ParseUuidError> {
        match value.as_str() {
            Some(str) => parse_str(str),
            None => Err(ParseUuidError),
        }
    }

    fn to_json(&self) -> Value {
        Value::String(self.0.clone())
    }
}

impl Uuid {
    pub fn new(str: &str) -> Result<Uuid, ParseUuidError> {
        parse_str(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        {
            let result = Uuid::new("67e55044-10b1-426f-9247-bb680e5fe0c8");
            let expected = String::from("67e55044-10b1-426f-9247-bb680e5fe0c8");

            assert!(matches!(result, Ok(Uuid(uuid)) if uuid == expected));
        }

        // invalid str
        {
            let result = Uuid::new("67e5504410b1-426f-9247-bb680e5fe0c8");

            assert!(matches!(result, Err(ParseUuidError)));
        }
    }
}
