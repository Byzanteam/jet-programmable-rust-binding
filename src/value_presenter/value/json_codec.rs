use serde_json::Value;

pub trait JsonCodec {
    type Err;

    fn from_json(value: &Value) -> Result<Self, Self::Err>;
    fn to_json(&self) -> Value;
}
