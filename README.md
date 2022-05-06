# The rust binding for jet programmable

This repository contains the codec and host functions for the programmable wasm program.

## Example
```rust
#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    outputs::Outputs, program, value_presenter::field_type::FieldType,
    value_presenter::ValuePresenter,
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    Outputs::build(inputs)
}

program!(entrypoint, vec![FieldType::SingleLineField]);

#[cfg(test)]
mod tests {
    // test code goes here
    // ...snpi
}
```
