use serde_json::Value;

use super::{
    field_type::FieldType,
    literal_naive_value::{
        BooleanFieldValue, CascaderFieldValue, DateTimeFieldValue, FileFieldValue,
        MultipleLineFieldValue, NumericFieldValue, RelationFieldValue, SingleLineFieldValue,
        TableRowFieldValue,
    },
    literal_value::{LiteralValue, ParseLiteralValueError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanListFieldValue {
    Value(Vec<BooleanFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CascaderListFieldValue {
    Value(Vec<CascaderFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeListFieldValue {
    Value(Vec<DateTimeFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileListFieldValue {
    Value(Vec<FileFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultipleLineListFieldValue {
    Value(Vec<MultipleLineFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumericListFieldValue {
    Value(Vec<NumericFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationListFieldValue {
    Value(Vec<RelationFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleLineListFieldValue {
    Value(Vec<SingleLineFieldValue>),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableRowListFieldValue {
    Value(Vec<TableRowFieldValue>),
    Nil,
}

impl LiteralValue for BooleanListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, BooleanListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(BooleanListFieldValue::Nil);
        }

        match list_from_json::<BooleanFieldValue>(value) {
            Ok(values) => Ok(BooleanListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            BooleanListFieldValue::Value(values) => list_to_json(values),
            BooleanListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::BooleanListField
    }
}

impl LiteralValue for CascaderListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, CascaderListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(CascaderListFieldValue::Nil);
        }

        match list_from_json::<CascaderFieldValue>(value) {
            Ok(values) => Ok(CascaderListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            CascaderListFieldValue::Value(values) => list_to_json(values),
            CascaderListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::CascaderListField
    }
}

impl LiteralValue for DateTimeListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, DateTimeListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(DateTimeListFieldValue::Nil);
        }

        match list_from_json::<DateTimeFieldValue>(value) {
            Ok(values) => Ok(DateTimeListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            DateTimeListFieldValue::Value(values) => list_to_json(values),
            DateTimeListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::DateTimeListField
    }
}

impl LiteralValue for FileListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, FileListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(FileListFieldValue::Nil);
        }

        match list_from_json::<FileFieldValue>(value) {
            Ok(values) => Ok(FileListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            FileListFieldValue::Value(values) => list_to_json(values),
            FileListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::FileListField
    }
}

impl LiteralValue for MultipleLineListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, MultipleLineListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(MultipleLineListFieldValue::Nil);
        }

        match list_from_json::<MultipleLineFieldValue>(value) {
            Ok(values) => Ok(MultipleLineListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            MultipleLineListFieldValue::Value(values) => list_to_json(values),
            MultipleLineListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::MultipleLineListField
    }
}

impl LiteralValue for NumericListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, NumericListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(NumericListFieldValue::Nil);
        }

        match list_from_json::<NumericFieldValue>(value) {
            Ok(values) => Ok(NumericListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            NumericListFieldValue::Value(values) => list_to_json(values),
            NumericListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::NumericListField
    }
}

impl LiteralValue for RelationListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, RelationListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(RelationListFieldValue::Nil);
        }

        match list_from_json::<RelationFieldValue>(value) {
            Ok(values) => Ok(RelationListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            RelationListFieldValue::Value(values) => list_to_json(values),
            RelationListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::RelationListField
    }
}

impl LiteralValue for SingleLineListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, SingleLineListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(SingleLineListFieldValue::Nil);
        }

        match list_from_json::<SingleLineFieldValue>(value) {
            Ok(values) => Ok(SingleLineListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            SingleLineListFieldValue::Value(values) => list_to_json(values),
            SingleLineListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::SingleLineListField
    }
}

impl LiteralValue for TableRowListFieldValue {
    fn is_nil(&self) -> bool {
        matches!(self, TableRowListFieldValue::Nil)
    }

    fn from_json(value: &Value) -> Result<Self, ParseLiteralValueError> {
        if value.is_null() {
            return Ok(TableRowListFieldValue::Nil);
        }

        match list_from_json::<TableRowFieldValue>(value) {
            Ok(values) => Ok(TableRowListFieldValue::Value(values)),
            Err(_err) => Err(ParseLiteralValueError),
        }
    }

    fn to_json(&self) -> Value {
        match self {
            TableRowListFieldValue::Value(values) => list_to_json(values),
            TableRowListFieldValue::Nil => Value::Null,
        }
    }

    fn get_field_type(&self) -> FieldType {
        FieldType::TableRowListField
    }
}

fn list_from_json<T: LiteralValue>(value: &Value) -> Result<Vec<T>, ParseLiteralValueError> {
    match value.as_array() {
        Some(values) => {
            let mut result: Vec<T> = Vec::new();
            for value in values {
                match T::from_json(value) {
                    Ok(v) => result.push(v),
                    Err(_err) => return Err(ParseLiteralValueError),
                }
            }

            Ok(result)
        }
        None => Err(ParseLiteralValueError),
    }
}

fn list_to_json<T: LiteralValue>(values: &Vec<T>) -> Value {
    let mut result = Vec::new();

    for value in values {
        result.push(value.to_json());
    }

    Value::Array(result)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::value_presenter::value::{
        cascader_value::CascaderValue, naive_date_time::NaiveDateTime, number::Number, uuid::Uuid,
    };

    use super::*;

    #[test]
    fn test_boolean_list_field_value_from_json() {
        {
            let value = json!([true, false, null]);
            let result = BooleanListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(
                result,
                BooleanListFieldValue::Value(values) if values.as_slice() == vec![
                    BooleanFieldValue::Value(true),
                    BooleanFieldValue::Value(false),
                    BooleanFieldValue::Nil,
                ]
            ));
        }

        // null
        {
            let value = json!(null);
            let result = BooleanListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(result, BooleanListFieldValue::Nil));
        }
    }

    #[test]
    fn test_cascader_list_field_value_from_json() {
        {
            let value = json!([
                              {
                                  "options_table_uuid": "00000000-0000-0000-0000-000000000000",
                                  "row_uuid": "00000000-0000-0000-0000-000000000001"
                              },
                              null
            ]);
            let result = CascaderListFieldValue::from_json(&value);

            assert!(result.is_ok());

            assert!(matches!(
                result.unwrap(),
                CascaderListFieldValue::Value(values) if values.as_slice() == vec![
                    CascaderFieldValue::Value(CascaderValue {
                        options_table_uuid: Uuid("00000000-0000-0000-0000-000000000000".to_string()),
                        row_uuid: Uuid("00000000-0000-0000-0000-000000000001".to_string()),
                    }),
                    CascaderFieldValue::Nil,
                ]
            ));
        }

        // null
        {
            let value = json!(null);
            let result = CascaderListFieldValue::from_json(&value);

            assert!(result.is_ok());

            assert!(matches!(result.unwrap(), CascaderListFieldValue::Nil));
        }
    }

    #[test]
    fn test_date_time_list_field_value_from_json() {
        {
            let value = json!(["2020-01-01T00:00:00Z", null]);
            let result = DateTimeListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(
                result,
                DateTimeListFieldValue::Value(values) if values.as_slice() == vec![
                    DateTimeFieldValue::Value(NaiveDateTime::new(2020, 1, 1, 0, 0, 0, 0)),
                    DateTimeFieldValue::Nil,
                ]
            ));
        }

        // null
        {
            let value = json!(null);
            let result = DateTimeListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(result, DateTimeListFieldValue::Nil));
        }
    }

    #[test]
    fn test_numeric_list_field_value_from_json() {
        {
            let value = json!([1 as i64, 1.01 as f64, null]);
            let result = NumericListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(
                result,
                NumericListFieldValue::Value(values) if values.as_slice() == vec![
                    NumericFieldValue::Value(Number::Integer(1)),
                    NumericFieldValue::Value(Number::Float(1.01)),
                    NumericFieldValue::Nil,
                ]
            ));
        }

        // null
        {
            let value = json!(null);
            let result = NumericListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(result, NumericListFieldValue::Nil));
        }
    }

    #[test]
    fn test_single_line_list_field_value_from_json() {
        {
            let value = json!(["foo", "bar", null]);
            let result = SingleLineListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(
                result,
                SingleLineListFieldValue::Value(values) if values.as_slice() == vec![
                    SingleLineFieldValue::Value("foo".to_string()),
                    SingleLineFieldValue::Value("bar".to_string()),
                    SingleLineFieldValue::Nil,
                ]
            ));
        }

        // null
        {
            let value = json!(null);
            let result = SingleLineListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(result, SingleLineListFieldValue::Nil));
        }
    }

    #[test]
    fn test_table_row_list_field_value_from_json() {
        {
            let value = json!(["67e55044-10b1-426f-9247-bb680e5fe0c8", null]);
            let result = TableRowListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(
                result,
                TableRowListFieldValue::Value(values) if values.as_slice() == vec![
                    TableRowFieldValue::Value(Uuid("67e55044-10b1-426f-9247-bb680e5fe0c8".to_string())),
                    TableRowFieldValue::Nil,
                ]
            ));
        }

        // null
        {
            let value = json!(null);
            let result = TableRowListFieldValue::from_json(&value);

            assert!(result.is_ok());

            let result = result.unwrap();

            assert!(matches!(result, TableRowListFieldValue::Nil));
        }
    }
}
