#[macro_use]
extern crate cfg_if;

#[cfg(feature = "wasm")]
pub mod wasm;
