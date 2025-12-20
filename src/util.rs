#[cfg(feature = "with_tetgen")]
pub const TETGEN_IS_AVAILABLE: bool = true;

#[cfg(not(feature = "with_tetgen"))]
pub const TETGEN_IS_AVAILABLE: bool = false;
