pub mod error;
pub mod field_type;
pub mod value_type;

pub mod codec;
pub mod literal;

use self::{field_type::FieldType, literal::LiteralValuePresenter};

#[derive(Debug, Clone)]
pub enum ValuePresenter {
    Literal(LiteralValuePresenter),
}

impl ValuePresenter {
    pub fn get_field_type(&self) -> FieldType {
        match self {
            ValuePresenter::Literal(literal_value_presenter) => {
                literal_value_presenter.get_field_type()
            }
        }
    }
}
