#[derive(PartialEq)]
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

impl FieldType {
    pub fn from_str(s: &str) -> Option<FieldType> {
        match s {
            "BooleanField" | "BOOLEAN_FIELD" | "boolean_field" => Some(FieldType::BooleanField),
            "CheckboxField" | "CHECKBOX_FIELD" | "checkbox_field" => Some(FieldType::CheckboxField),
            "DateTimeField" | "DATE_TIME_FIELD" | "date_time_field" => {
                Some(FieldType::DateTimeField)
            }
            "NumericField" | "NUMERIC_FIELD" | "numeric_field" => Some(FieldType::NumericField),
            "RadioButtonField" | "RADIO_BUTTON_FIELD" | "radio_button_field" => {
                Some(FieldType::RadioButtonField)
            }
            "SingleLineField" | "SINGLE_LINE_FIELD" | "single_line_field" => {
                Some(FieldType::SingleLineField)
            }
            "TableRowField" | "TABLE_ROW_FIELD" | "table_row_field" => {
                Some(FieldType::TableRowField)
            }
            "UserBoundaryField" | "USER_BOUNDARY_FIELD" | "user_boundary_field" => {
                Some(FieldType::UserBoundaryField)
            }
            _ => None,
        }
    }
}
