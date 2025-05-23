#![deny(clippy::all)]

mod basic;
mod class;

pub use basic::*;
pub use class::*;

#[macro_use]
extern crate napi_derive;
