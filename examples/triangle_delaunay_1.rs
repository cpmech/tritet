use plotpy::Plot;
use tritet::{StrError, Triangle};

fn main() -> Result<(), StrError> {
    // allocate data for 5 points
    let mut triangle = Triangle::new(15, None, None, None)?;

    // set points
    triangle
        .set_point(0, 0.0, 0.0)?
        .set_point(1, -0.416, 0.909)?
        .set_point(2, -1.35, 0.436)?
        .set_point(3, -1.64, -0.549)?
        .set_point(4, -1.31, -1.51)?
        .set_point(5, -0.532, -2.17)?
        .set_point(6, 0.454, -2.41)?
        .set_point(7, 1.45, -2.21)?
        .set_point(8, 2.29, -1.66)?
        .set_point(9, 2.88, -0.838)?
        .set_point(10, 3.16, 0.131)?
        .set_point(11, 3.12, 1.14)?
        .set_point(12, 2.77, 2.08)?
        .set_point(13, 2.16, 2.89)?
        .set_point(14, 1.36, 3.49)?;

    // generate Delaunay triangulation
    triangle.generate_delaunay(true)?;

    // draw triangles
    let mut plot = Plot::new();
    triangle.draw_triangles(&mut plot, true, true, true, true, None, None, None);
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/tmp/tritet/example_triangle_delaunay_1.svg")?;
    Ok(())
}
