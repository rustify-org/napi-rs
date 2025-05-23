#![deny(clippy::all)]

mod basic;
mod class;
mod object;
pub use basic::*;
pub use class::*;
pub use object::*;

#[macro_use]
extern crate napi_derive;
