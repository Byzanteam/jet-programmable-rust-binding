use serde_json::Value;

use crate::value_presenter::ValuePresenter;

pub struct Outputs(pub Vec<ValuePresenter>);

impl Outputs {
    pub fn build(value_presenters: Vec<ValuePresenter>) -> Self {
        Self(value_presenters)
    }

    pub fn to_json(&self) -> Value {
        Value::Array(self.0.iter().map(|vp| vp.to_json()).collect())
    }
}

#[cfg(test)]
mod tests {

    use crate::value_presenter::{
        literal_naive_value::BooleanFieldValue, literal_value_presenter::LiteralValuePresenter,
    };

    use super::*;
    use serde_json::json;

    #[test]
    fn test_to_json() {
        let outputs = Outputs(vec![
            ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                BooleanFieldValue::Value(true),
            )),
            ValuePresenter::Literal(LiteralValuePresenter::BooleanField(BooleanFieldValue::Nil)),
        ]);

        let expected = json!([
              {
                  "type": "LITERAL",
                  "field_type": "BOOLEAN_FIELD",
                  "value": true
              },
              {
                  "type": "LITERAL",
                  "field_type": "BOOLEAN_FIELD",
                  "value": null
              }
        ]);

        assert!(outputs.to_json().to_string() == expected.to_string());
    }
}
