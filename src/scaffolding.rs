//! Scaffolding for a guest application.

use crate::{
    arguments::parse,
    hostcalls::hostcall_set_outputs,
    outputs::Outputs,
    value_presenter::{field_type::FieldType, ValuePresenter},
};
use serde_json::Value;

#[macro_export]
macro_rules! program {
    ($entrypoint:ident, $types:expr) => {
        #[no_mangle]
        pub fn run(inputs: &str) {
            jet_programmable_rust_binding::wrap_run(inputs, $entrypoint, $types)
        }
    };
}

#[doc(hidden)]
pub fn wrap_run<F>(inputs: &str, entrypoint: F, types: Vec<FieldType>)
where
    F: Fn(Vec<ValuePresenter>) -> Outputs,
{
    let json: Value = match serde_json::from_str(inputs) {
        Ok(json) => json,
        Err(err) => panic!("Failed to parse inputs: {}", err),
    };

    let outputs: Outputs = match parse(&json, types) {
        Ok(inputs) => entrypoint(inputs),
        Err(err) => panic!("Failed to decode inputs: {:?}", err),
    };

    let str = outputs.to_json().to_string();

    unsafe {
        hostcall_set_outputs(str.as_ptr(), str.len());
    }
}
