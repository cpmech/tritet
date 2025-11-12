use plotpy::Plot;
use tritet::{InputDataTriMesh, StrError, Trigen};

const SAVE_FIGURE: bool = false;

fn main() -> Result<(), StrError> {
    // set input data
    let input_data = InputDataTriMesh {
        points: vec![
            (0, 0.0, 0.0), // boundary marker, x, y
            (0, 1.0, 0.0),
            (0, 1.0, 1.0),
            (0, 0.0, 1.0),
            (0, 0.2, 0.2),
            (0, 0.8, 0.2),
            (0, 0.8, 0.8),
            (0, 0.2, 0.8),
            (0, 0.0, 0.5),
            (0, 0.2, 0.5),
            (0, 0.8, 0.5),
            (0, 1.0, 0.5),
        ],
        segments: vec![
            (-1, 0, 1), // boundary marker, point indices
            (-1, 1, 2),
            (-1, 2, 3),
            (-1, 3, 0),
            (-1, 4, 5),
            (-1, 5, 6),
            (-1, 6, 7),
            (-1, 7, 4),
            (-1, 8, 9),
            (-1, 10, 11),
        ],
        holes: vec![
            (0.5, 0.5), // x, y
        ],
        regions: vec![
            (1, 0.1, 0.1, None), // attribute, x, y, max area
            (2, 0.1, 0.9, None),
        ],
    };

    // allocate generator from input data
    let trigen = Trigen::from_input_data(&input_data)?;

    // generate o2 mesh without constraints
    trigen.generate_mesh(false, true, false, None, None)?;
    assert_eq!(trigen.out_ncell(), 12);

    // draw mesh
    if SAVE_FIGURE {
        let mut plot = Plot::new();
        trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/doc_triangle_mesh_1.svg")?;
    }
    Ok(())
}
