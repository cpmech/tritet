//! Triangle and tetrahedron mesh generators

/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

mod constants;
mod conversion;
mod tetgen;
mod tetgen_paraview;
mod trigen;
mod trigen_paraview;
pub use crate::tetgen::*;
pub use crate::tetgen_paraview::*;
pub use crate::trigen::*;

// run code from README file
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }
    external_doc_test!(include_str!("../README.md"));
}
