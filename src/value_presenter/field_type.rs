#[derive(PartialEq, Debug)]
pub enum FieldType {
    BooleanField,
    CheckboxField,
    DateTimeField,
    NumericField,
    RadioButtonField,
    SingleLineField,
    TableRowField,
    UserBoundaryField,
}

pub struct ParseFieldTypeError;

impl FieldType {
    pub fn parse_str(s: &str) -> Result<Self, ParseFieldTypeError> {
        match s {
            "BooleanField" | "BOOLEAN_FIELD" | "boolean_field" => Ok(FieldType::BooleanField),
            "CheckboxField" | "CHECKBOX_FIELD" | "checkbox_field" => Ok(FieldType::CheckboxField),
            "DateTimeField" | "DATE_TIME_FIELD" | "date_time_field" => Ok(FieldType::DateTimeField),
            "NumericField" | "NUMERIC_FIELD" | "numeric_field" => Ok(FieldType::NumericField),
            "RadioButtonField" | "RADIO_BUTTON_FIELD" | "radio_button_field" => {
                Ok(FieldType::RadioButtonField)
            }
            "SingleLineField" | "SINGLE_LINE_FIELD" | "single_line_field" => {
                Ok(FieldType::SingleLineField)
            }
            "TableRowField" | "TABLE_ROW_FIELD" | "table_row_field" => Ok(FieldType::TableRowField),
            "UserBoundaryField" | "USER_BOUNDARY_FIELD" | "user_boundary_field" => {
                Ok(FieldType::UserBoundaryField)
            }
            _ => Err(ParseFieldTypeError),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            FieldType::BooleanField => From::from("BOOLEAN_FIELD"),
            FieldType::CheckboxField => From::from("CHECKBOX_FIELD"),
            FieldType::DateTimeField => From::from("DATE_TIME_FIELD"),
            FieldType::NumericField => From::from("NUMERIC_FIELD"),
            FieldType::RadioButtonField => From::from("RADIO_BUTTON_FIELD"),
            FieldType::SingleLineField => From::from("SINGLE_LINE_FIELD"),
            FieldType::TableRowField => From::from("TABLE_ROW_FIELD"),
            FieldType::UserBoundaryField => From::from("USER_BOUNDARY_FIELD"),
        }
    }
}
