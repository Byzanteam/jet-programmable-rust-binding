use serde_json::Value;

use crate::value_presenter::ValuePresenter;

pub struct Outputs(pub Vec<ValuePresenter>);

impl Outputs {
    pub fn to_json(&self) -> Value {
        Value::Array(self.0.iter().map(|vp| vp.to_json()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::value_presenter::literal::LiteralValuePresenter;

    use super::*;
    use serde_json::json;

    #[test]
    fn test_to_json() {
        let outputs = Outputs(vec![
            ValuePresenter::Literal(LiteralValuePresenter::BooleanField(Some(true))),
            ValuePresenter::Literal(LiteralValuePresenter::BooleanField(None)),
        ]);

        let expected = json!([
              {
                  "type": "literal",
                  "field_type": "BOOLEAN_FIELD",
                  "value": true
              },
              {
                  "type": "literal",
                  "field_type": "BOOLEAN_FIELD",
                  "value": null
              }
        ]);

        assert!(outputs.to_json().to_string() == expected.to_string());
    }
}