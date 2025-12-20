//! Triangle and tetrahedron mesh generators

/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

mod constants;
mod conversion;
mod input_data_tet_mesh;
mod input_data_tri_mesh;
mod tetgen;
mod tetgen_paraview;
mod trigen;
mod trigen_paraview;
mod util;
pub use input_data_tet_mesh::*;
pub use input_data_tri_mesh::*;
pub use tetgen::*;
pub use trigen::*;
pub use util::*;

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
