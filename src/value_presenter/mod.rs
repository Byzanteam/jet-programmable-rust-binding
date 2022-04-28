pub mod error;
pub mod field_type;
pub mod value_type;

pub mod codec;
pub mod literal;

use self::{field_type::FieldType, literal::LiteralValuePresenter};

pub enum ValuePresenter {
    Literal(LiteralValuePresenter),
}

impl ValuePresenter {
    pub fn field_type_matches(&self, field_type: &FieldType) -> bool {
        match self {
            ValuePresenter::Literal(literal_value_presenter) => {
                literal_value_presenter.field_type_matches(field_type)
            }
        }
    }
}
