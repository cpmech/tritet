//! Triangle and tetrahedron mesh generators

/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

mod constants;
mod to_i32;
mod triangle;
pub use crate::triangle::*;
