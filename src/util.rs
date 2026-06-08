/// Indicates whether Tetgen is available (enabled via the `with_tetgen` feature)
#[cfg(feature = "with_tetgen")]
pub const TETGEN_IS_AVAILABLE: bool = true;

/// Indicates whether Tetgen is available (enabled via the `with_tetgen` feature)
#[cfg(not(feature = "with_tetgen"))]
pub const TETGEN_IS_AVAILABLE: bool = false;
