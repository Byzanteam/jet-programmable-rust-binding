use serde_json::{json, Value};

use super::json_codec::JsonCodec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProsemirrorState {
    pub doc: Value,
}

#[derive(Debug)]
pub struct ParseProsemirrorStateError;

impl JsonCodec for ProsemirrorState {
    type Err = ParseProsemirrorStateError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        if !value.is_object() {
            return Err(ParseProsemirrorStateError);
        }

        Ok(ProsemirrorState { doc: value.clone() })
    }

    fn to_json(&self) -> Value {
        json!(self.doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, json};

    #[test]
    fn test_from_json() {
        {
            let doc = r#"{
                "type": "doc",
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": "hello world!"
                            }
                        ]
                    }
                ]
            }"#;

            let json = from_str(doc).unwrap();

            let state = ProsemirrorState::from_json(&json);

            assert!(matches!(state, Ok(ProsemirrorState { doc: state_doc }) if state_doc == json),);
        }
    }

    #[test]
    fn test_from_invalid_json() {
        {
            let json = json!([]);

            let state = ProsemirrorState::from_json(&json);

            assert!(matches!(state, Err(ParseProsemirrorStateError)));
        }
    }

    #[test]
    fn test_to_json() {
        {
            let doc = r#"{
                "type": "doc",
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": "hello world!"
                            }
                        ]
                    }
                ]
            }"#;
            let state = ProsemirrorState {
                doc: from_str(doc).unwrap(),
            };

            let json: Value = from_str(doc).unwrap();

            assert!(state.to_json() == json);
        }
    }
}
