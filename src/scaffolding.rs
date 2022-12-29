//! Scaffolding for a guest application.

use crate::{
    hostcalls::hostcall_set_outputs,
    inputs::parse,
    outputs::Outputs,
    value_presenter::{field_type::FieldType, ValuePresenter},
};
use serde_json::Value;

#[macro_export]
#[cfg(feature = "memory")]
macro_rules! if_memory {
        ($($i:item)*) => ($($i)*)
}

#[macro_export]
#[cfg(not(feature = "memory"))]
macro_rules! if_memory {
    ($($i:item)*) => {};
}

#[macro_export]
macro_rules! program {
    ($entrypoint:ident, $types:expr) => {
        #[no_mangle]
        pub fn run(inputs: &str) {
            $crate::wrap_run(inputs, $entrypoint, $types)
        }

        $crate::if_memory! {
            pub use $crate::memory::*;
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
