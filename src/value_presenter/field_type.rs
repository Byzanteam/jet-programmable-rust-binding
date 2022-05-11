#[derive(PartialEq, Debug)]
pub enum FieldType {
    BooleanField,
    CascaderField,
    CheckboxField,
    DateTimeField,
    FileField,
    MultipleLineField,
    NumericField,
    RadioButtonField,
    RelationField,
    SingleLineField,
    TableRowField,
    UserBoundaryField,

    // list field
    BooleanListField,
    CascaderListField,
    DateTimeListField,
    FileListField,
    MultipleLineListField,
    NumericListField,
    RelationListField,
    SingleLineListField,
    TableRowListField,
}

pub struct ParseFieldTypeError;

impl FieldType {
    pub fn parse_str(s: &str) -> Result<Self, ParseFieldTypeError> {
        match s {
            "BooleanField" | "BOOLEAN_FIELD" | "boolean_field" => Ok(FieldType::BooleanField),
            "CascaderField" | "CASCADER_FIELD" | "cascader_field" => Ok(FieldType::CascaderField),
            "CheckboxField" | "CHECKBOX_FIELD" | "checkbox_field" => Ok(FieldType::CheckboxField),
            "DateTimeField" | "DATE_TIME_FIELD" | "date_time_field" => Ok(FieldType::DateTimeField),
            "FileField" | "FILE_FIELD" | "file_field" => Ok(FieldType::FileField),
            "MultipleLineField" | "MULTIPLE_LINE_FIELD" | "multiple_line_field" => {
                Ok(FieldType::MultipleLineField)
            }
            "NumericField" | "NUMERIC_FIELD" | "numeric_field" => Ok(FieldType::NumericField),
            "RadioButtonField" | "RADIO_BUTTON_FIELD" | "radio_button_field" => {
                Ok(FieldType::RadioButtonField)
            }
            "RelationField" | "RELATION_FIELD" | "relation_field" => Ok(FieldType::RelationField),
            "SingleLineField" | "SINGLE_LINE_FIELD" | "single_line_field" => {
                Ok(FieldType::SingleLineField)
            }
            "TableRowField" | "TABLE_ROW_FIELD" | "table_row_field" => Ok(FieldType::TableRowField),
            "UserBoundaryField" | "USER_BOUNDARY_FIELD" | "user_boundary_field" => {
                Ok(FieldType::UserBoundaryField)
            }

            // list field
            "BooleanListField" | "BOOLEAN_LIST_FIELD" | "boolean_list_field" => {
                Ok(FieldType::BooleanListField)
            }
            "CascaderListField" | "CASCADER_LIST_FIELD" | "cascader_list_field" => {
                Ok(FieldType::CascaderListField)
            }
            "DateTimeListField" | "DATE_TIME_LIST_FIELD" | "date_time_list_field" => {
                Ok(FieldType::DateTimeListField)
            }
            "FileListField" | "FILE_LIST_FIELD" | "file_list_field" => Ok(FieldType::FileListField),
            "MultipleLineListField" | "MULTIPLE_LINE_LIST_FIELD" | "multiple_line_list_field" => {
                Ok(FieldType::MultipleLineListField)
            }
            "NumericListField" | "NUMERIC_LIST_FIELD" | "numeric_list_field" => {
                Ok(FieldType::NumericListField)
            }
            "RelationListField" | "RELATION_LIST_FIELD" | "relation_list_field" => {
                Ok(FieldType::RelationListField)
            }
            "SingleLineListField" | "SINGLE_LINE_LIST_FIELD" | "single_line_list_field" => {
                Ok(FieldType::SingleLineListField)
            }
            "TableRowListField" | "TABLE_ROW_LIST_FIELD" | "table_row_list_field" => {
                Ok(FieldType::TableRowListField)
            }
            _ => Err(ParseFieldTypeError),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            FieldType::BooleanField => From::from("BOOLEAN_FIELD"),
            FieldType::CascaderField => From::from("CASCADER_FIELD"),
            FieldType::CheckboxField => From::from("CHECKBOX_FIELD"),
            FieldType::DateTimeField => From::from("DATE_TIME_FIELD"),
            FieldType::FileField => From::from("FILE_FIELD"),
            FieldType::MultipleLineField => From::from("MULTIPLE_LINE_FIELD"),
            FieldType::NumericField => From::from("NUMERIC_FIELD"),
            FieldType::RadioButtonField => From::from("RADIO_BUTTON_FIELD"),
            FieldType::RelationField => From::from("RELATION_FIELD"),
            FieldType::SingleLineField => From::from("SINGLE_LINE_FIELD"),
            FieldType::TableRowField => From::from("TABLE_ROW_FIELD"),
            FieldType::UserBoundaryField => From::from("USER_BOUNDARY_FIELD"),

            // list field
            FieldType::BooleanListField => From::from("BOOLEAN_LIST_FIELD"),
            FieldType::CascaderListField => From::from("CASCADER_LIST_FIELD"),
            FieldType::DateTimeListField => From::from("DATE_TIME_LIST_FIELD"),
            FieldType::FileListField => From::from("FILE_LIST_FIELD"),
            FieldType::MultipleLineListField => From::from("MULTIPLE_LINE_LIST_FIELD"),
            FieldType::NumericListField => From::from("NUMERIC_LIST_FIELD"),
            FieldType::RelationListField => From::from("RELATION_LIST_FIELD"),
            FieldType::SingleLineListField => From::from("SINGLE_LINE_LIST_FIELD"),
            FieldType::TableRowListField => From::from("TABLE_ROW_LIST_FIELD"),
        }
    }
}
