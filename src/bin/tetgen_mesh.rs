use plotpy::Plot;
use tritet::{InputDataTetMesh, StrError, Tetgen};

const SAVE_FIGURE: bool = false;

fn main() -> Result<(), StrError> {
    // set input data
    let input_data = InputDataTetMesh {
        points: vec![
            (0, 0.0, 1.0, 0.0), // marker, x, y, z
            (0, 0.0, 0.0, 0.0),
            (0, 1.0, 1.0, 0.0),
            (0, 0.0, 1.0, 1.0),
        ],
        facets: vec![
            (0, vec![0, 2, 1]), // marker, point indices
            (0, vec![0, 1, 3]),
            (0, vec![0, 3, 2]),
            (0, vec![1, 2, 3]),
        ],
        holes: vec![],                           // no holes
        regions: vec![(1, 0.1, 0.9, 0.1, None)], // region marker, x, y, z, max volume
    };

    // allocate generator from input data
    let tetgen = Tetgen::from_input_data(&input_data)?;

    // generate mesh
    let global_max_volume = Some(0.5);
    tetgen.generate_mesh(false, false, global_max_volume, None)?;

    // draw edges of tetrahedra
    if SAVE_FIGURE {
        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/doc_tetgen_mesh_2.svg")?;
    }

    assert_eq!(tetgen.out_ncell(), 7);
    assert_eq!(tetgen.out_npoint(), 10);
    Ok(())
}
