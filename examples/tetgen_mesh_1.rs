use plotpy::Plot;
use tritet::{write_tet_vtu, StrError, Tetgen};

fn main() -> Result<(), StrError> {
    // allocate data for 16 points and 12 facets
    // (one cube/hole inside another cube)
    let mut tetgen = Tetgen::new(
        16,
        Some(vec![
            4, 4, 4, 4, 4, 4, // inner cube
            4, 4, 4, 4, 4, 4, // outer cube
        ]),
        Some(1),
        Some(1),
    )?;

    // inner cube
    tetgen
        .set_point(0, 0.0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0, 0.0)?
        .set_point(2, 1.0, 1.0, 0.0)?
        .set_point(3, 0.0, 1.0, 0.0)?
        .set_point(4, 0.0, 0.0, 1.0)?
        .set_point(5, 1.0, 0.0, 1.0)?
        .set_point(6, 1.0, 1.0, 1.0)?
        .set_point(7, 0.0, 1.0, 1.0)?;

    // outer cube
    tetgen
        .set_point(8, -1.0, -1.0, -1.0)?
        .set_point(9, 2.0, -1.0, -1.0)?
        .set_point(10, 2.0, 2.0, -1.0)?
        .set_point(11, -1.0, 2.0, -1.0)?
        .set_point(12, -1.0, -1.0, 2.0)?
        .set_point(13, 2.0, -1.0, 2.0)?
        .set_point(14, 2.0, 2.0, 2.0)?
        .set_point(15, -1.0, 2.0, 2.0)?;

    // inner cube
    tetgen
        .set_facet_point(0, 0, 0)?
        .set_facet_point(0, 1, 4)?
        .set_facet_point(0, 2, 7)?
        .set_facet_point(0, 3, 3)?;
    tetgen
        .set_facet_point(1, 0, 1)?
        .set_facet_point(1, 1, 2)?
        .set_facet_point(1, 2, 6)?
        .set_facet_point(1, 3, 5)?;
    tetgen
        .set_facet_point(2, 0, 0)?
        .set_facet_point(2, 1, 1)?
        .set_facet_point(2, 2, 5)?
        .set_facet_point(2, 3, 4)?;
    tetgen
        .set_facet_point(3, 0, 2)?
        .set_facet_point(3, 1, 3)?
        .set_facet_point(3, 2, 7)?
        .set_facet_point(3, 3, 6)?;
    tetgen
        .set_facet_point(4, 0, 0)?
        .set_facet_point(4, 1, 3)?
        .set_facet_point(4, 2, 2)?
        .set_facet_point(4, 3, 1)?;
    tetgen
        .set_facet_point(5, 0, 4)?
        .set_facet_point(5, 1, 5)?
        .set_facet_point(5, 2, 6)?
        .set_facet_point(5, 3, 7)?;

    // outer cube
    tetgen
        .set_facet_point(6, 0, 8 + 0)?
        .set_facet_point(6, 1, 8 + 4)?
        .set_facet_point(6, 2, 8 + 7)?
        .set_facet_point(6, 3, 8 + 3)?;
    tetgen
        .set_facet_point(7, 0, 8 + 1)?
        .set_facet_point(7, 1, 8 + 2)?
        .set_facet_point(7, 2, 8 + 6)?
        .set_facet_point(7, 3, 8 + 5)?;
    tetgen
        .set_facet_point(8, 0, 8 + 0)?
        .set_facet_point(8, 1, 8 + 1)?
        .set_facet_point(8, 2, 8 + 5)?
        .set_facet_point(8, 3, 8 + 4)?;
    tetgen
        .set_facet_point(9, 0, 8 + 2)?
        .set_facet_point(9, 1, 8 + 3)?
        .set_facet_point(9, 2, 8 + 7)?
        .set_facet_point(9, 3, 8 + 6)?;
    tetgen
        .set_facet_point(10, 0, 8 + 0)?
        .set_facet_point(10, 1, 8 + 3)?
        .set_facet_point(10, 2, 8 + 2)?
        .set_facet_point(10, 3, 8 + 1)?;
    tetgen
        .set_facet_point(11, 0, 8 + 4)?
        .set_facet_point(11, 1, 8 + 5)?
        .set_facet_point(11, 2, 8 + 6)?
        .set_facet_point(11, 3, 8 + 7)?;

    // set region and hole
    tetgen.set_region(0, -0.9, -0.9, -0.9, 1, None)?;
    tetgen.set_hole(0, 0.5, 0.5, 0.5)?;

    // generate mesh
    tetgen.generate_mesh(false, false, None, None)?;

    // generate file for Paraview
    write_tet_vtu(&tetgen, "/tmp/tritet/example_tetgen_mesh_1.vtu")?;

    // draw edges of tetrahedra
    let mut plot = Plot::new();
    tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/tmp/tritet/example_tetgen_mesh_1.svg")?;
    Ok(())
}
