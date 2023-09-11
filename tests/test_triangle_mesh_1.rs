use plotpy::Plot;
use tritet::{StrError, Trigen};

#[test]
fn test_triangle_mesh_1() -> Result<(), StrError> {
    let mut triangle = Trigen::new(12, Some(20), Some(7), Some(2))?;
    triangle
        .set_point(0, 0, 0.0, 0.0)?
        .set_point(1, 0, 2.0, 0.0)?
        .set_point(2, 0, 4.0, 0.0)?
        .set_point(3, 0, 1.0, 1.0)?
        .set_point(4, 0, 3.0, 1.0)?
        .set_point(5, 0, 0.0, 2.0)?
        .set_point(6, 0, 4.0, 2.0)?
        .set_point(7, 0, 1.0, 3.0)?
        .set_point(8, 0, 3.0, 3.0)?
        .set_point(9, 0, 0.0, 4.0)?
        .set_point(10, 0, 2.0, 4.0)?
        .set_point(11, 0, 4.0, 4.0)?;
    triangle
        .set_segment(0, 0, 0, 1)?
        .set_segment(1, 0, 1, 2)?
        .set_segment(2, 0, 0, 5)?
        .set_segment(3, 0, 1, 3)?
        .set_segment(4, 0, 1, 4)?
        .set_segment(5, 0, 2, 6)?
        .set_segment(6, 0, 3, 5)?
        .set_segment(7, 0, 4, 6)?
        .set_segment(8, 0, 5, 9)?
        .set_segment(9, 0, 5, 7)?
        .set_segment(10, 0, 3, 7)?
        .set_segment(11, 0, 4, 8)?
        .set_segment(12, 0, 6, 8)?
        .set_segment(13, 0, 6, 11)?
        .set_segment(14, 0, 7, 10)?
        .set_segment(15, 0, 8, 10)?
        .set_segment(16, 0, 9, 10)?
        .set_segment(17, 0, 10, 11)?
        .set_segment(18, 0, 3, 4)?
        .set_segment(19, 0, 7, 8)?;
    triangle
        .set_region(0, 1, 0.1, 0.1, None)?
        .set_region(1, 2, 2.0, 0.1, None)?
        .set_region(2, 3, 3.9, 0.1, None)?
        .set_region(3, 4, 0.1, 3.9, None)?
        .set_region(4, 5, 2.0, 3.9, None)?
        .set_region(5, 6, 3.9, 3.9, None)?
        .set_region(6, 7, 2.0, 2.0, None)?;
    triangle.set_hole(0, 0.1, 2.0)?.set_hole(1, 3.9, 2.0)?;
    triangle.generate_mesh(false, false, false, None, None)?;

    assert_eq!(triangle.out_npoint(), 12);
    assert_eq!(triangle.out_ncell(), 12);
    assert_eq!(triangle.out_cell_attribute(0), 1);
    assert_eq!(triangle.out_cell_attribute(1), 7);
    assert_eq!(triangle.out_cell_attribute(2), 1);
    assert_eq!(triangle.out_cell_attribute(3), 4);
    assert_eq!(triangle.out_cell_attribute(4), 4);
    assert_eq!(triangle.out_cell_attribute(5), 2);
    assert_eq!(triangle.out_cell_attribute(6), 3);
    assert_eq!(triangle.out_cell_attribute(7), 3);
    assert_eq!(triangle.out_cell_attribute(8), 6);
    assert_eq!(triangle.out_cell_attribute(9), 5);
    assert_eq!(triangle.out_cell_attribute(10), 6);
    assert_eq!(triangle.out_cell_attribute(11), 7);

    let mut plot = Plot::new();
    triangle.draw_triangles(&mut plot, true, true, true, true, Some(12.0), Some(24.0), Some(14.0));
    if false {
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/test_triangle_mesh_1.svg")?;
    }
    Ok(())
}
