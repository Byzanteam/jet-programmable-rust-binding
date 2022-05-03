use serde_json::{Number as JsonNumber, Value};

use super::json_codec::JsonCodec;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct ParseNumberError;

impl JsonCodec for Number {
    type Err = ParseNumberError;

    fn from_json(value: &Value) -> Result<Self, Self::Err> {
        match value.as_u64() {
            Some(number) => Ok(Number::Integer(number as i64)),
            None => match value.as_i64() {
                Some(number) => Ok(Number::Integer(number)),
                None => match value.as_f64() {
                    Some(number) => Ok(Number::Float(number)),
                    None => Err(ParseNumberError),
                },
            },
        }
    }

    fn to_json(&self) -> Value {
        match self {
            Number::Integer(i) => Value::Number(JsonNumber::from(*i)),
            Number::Float(f) => Value::Number(JsonNumber::from_f64(*f).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        {
            let json: Value = serde_json::from_str("1").unwrap();

            let number = Number::from_json(&json).unwrap();

            assert!(matches!(number, Number::Integer(1)));
        }

        // float
        {
            let json: Value = serde_json::from_str("1.1").unwrap();

            let number = Number::from_json(&json).unwrap();

            assert!(matches!(number, Number::Float(f) if f == 1.1));
        }

        // signed int
        {
            let json: Value = serde_json::from_str("-1").unwrap();

            let number = Number::from_json(&json).unwrap();

            assert!(matches!(number, Number::Integer(-1)));
        }
    }

    #[test]
    fn test_to_json() {
        {
            let number = Number::Integer(-1);
            assert!(number.to_json() == Value::Number(JsonNumber::from(-1)));
        }

        {
            let number = Number::Float(1.1);
            assert!(number.to_json() == Value::Number(JsonNumber::from_f64(1.1).unwrap()));
        }

        {
            let number = Number::Integer(-1);
            assert!(number.to_json() == Value::Number(JsonNumber::from(-1)));
        }
    }
}
