use plotpy::Plot;
use tritet::{StrError, Tetgen};

fn main() -> Result<(), StrError> {
    // allocate data for 8 points
    let mut tetgen = Tetgen::new(8, None, None, None)?;

    // set points
    tetgen
        .set_point(0, 0, 0.0, 0.0, 0.0)?
        .set_point(1, 0, 1.0, 0.0, 0.0)?
        .set_point(2, 0, 1.0, 1.0, 0.0)?
        .set_point(3, 0, 0.0, 1.0, 0.0)?
        .set_point(4, 0, 0.0, 0.0, 1.0)?
        .set_point(5, 0, 1.0, 0.0, 1.0)?
        .set_point(6, 0, 1.0, 1.0, 1.0)?
        .set_point(7, 0, 0.0, 1.0, 1.0)?;

    // generate Delaunay triangulation
    tetgen.generate_delaunay(false)?;

    // draw edges of tetrahedra
    let mut plot = Plot::new();
    tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/tmp/tritet/example_tetgen_delaunay_1.svg")?;
    Ok(())
}
