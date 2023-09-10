use plotpy::Plot;
use tritet::{StrError, Trigen};

fn main() -> Result<(), StrError> {
    // allocate data for 26 points, 22 segments, and 3 holes
    let mut trigen = Trigen::new(26, Some(22), None, Some(3))?;

    // the outer polyhedron
    trigen
        .set_point(0, 80.0, 0.0)?
        .set_point(1, 100.0, 50.0)?
        .set_point(2, 0.0, 100.0)?
        .set_point(3, -100.0, 50.0)?
        .set_point(4, -80.0, 0.0)?
        .set_point(5, -100.0, -50.0)?
        .set_point(6, 0.0, -100.0)?
        .set_point(7, 100.0, -50.0)?;
    // the mouth
    trigen
        .set_point(8, 0.0, -90.0)?
        .set_point(9, 80.0, -50.0)?
        .set_point(10, 0.0, -10.0)?
        .set_point(11, -80.0, -50.0)?;
    // the left eye
    trigen
        .set_point(12, -70.0, 50.0)?
        .set_point(13, -60.0, 30.0)?
        .set_point(14, -10.0, 55.0)?
        .set_point(15, -40.0, 55.0)?;
    // the right eye
    trigen
        .set_point(16, 70.0, 50.0)?
        .set_point(17, 60.0, 30.0)?
        .set_point(18, 10.0, 55.0)?
        .set_point(19, 40.0, 55.0)?;
    // two nostril segments
    trigen
        .set_point(20, -10.0, 25.0)?
        .set_point(21, -20.0, -10.0)?
        .set_point(22, 10.0, 25.0)?
        .set_point(23, 20.0, -10.0)?;
    // two dimples
    trigen.set_point(24, -50.0, 0.0)?.set_point(25, 50.0, 0.0)?;

    // the outer polyhedron
    trigen
        .set_segment(0, 0, 1)?
        .set_segment(1, 1, 2)?
        .set_segment(2, 2, 3)?
        .set_segment(3, 3, 4)?
        .set_segment(4, 4, 5)?
        .set_segment(5, 5, 6)?
        .set_segment(6, 6, 7)?
        .set_segment(7, 7, 0)?;
    // the mouth
    trigen
        .set_segment(8, 8, 9)?
        .set_segment(9, 9, 10)?
        .set_segment(10, 10, 11)?
        .set_segment(11, 11, 8)?;
    // the left eye
    trigen
        .set_segment(12, 12, 13)?
        .set_segment(13, 13, 14)?
        .set_segment(14, 14, 15)?
        .set_segment(15, 15, 12)?;
    // the right eye
    trigen
        .set_segment(16, 16, 17)?
        .set_segment(17, 17, 18)?
        .set_segment(18, 18, 19)?
        .set_segment(19, 19, 16)?;
    // two nostril segments
    trigen.set_segment(20, 20, 21)?.set_segment(21, 22, 23)?;

    // three holes
    trigen
        .set_hole(0, 0.0, -50.0)? // mouth
        .set_hole(1, -50.0, 50.0)? // left eye
        .set_hole(2, 50.0, 50.0)?; // right eye

    // generate mesh without constraints
    trigen.generate_mesh(true, true, true, None, None)?;

    // draw mesh
    let mut plot = Plot::new();
    trigen.draw_triangles(&mut plot, true, false, false, false, None, None, None);
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/tmp/tritet/example_triangle_mesh_1.svg")?;
    Ok(())
}
