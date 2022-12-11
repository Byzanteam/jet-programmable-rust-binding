pub mod value_presenter;

pub mod inputs;
pub mod outputs;

#[macro_use]
mod scaffolding;

pub mod hostcalls;
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "networking")]
pub mod networking;

pub use crate::scaffolding::wrap_run;
