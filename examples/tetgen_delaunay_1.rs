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

    // print the results
    println!("Number of points           = {}", tetgen.out_npoint());
    println!("Number of cells            = {}", tetgen.out_ncell());
    println!("Number of points in a cell = {}\n", tetgen.out_cell_npoint());
    let ndim = 3;
    for index in 0..tetgen.out_npoint() {
        print!("Point {}: (", index);
        for d in 0..ndim {
            if d > 0 {
                print!(", ");
            }
            print!("{}", tetgen.out_point(index, d));
        }
        println!(")");
    }
    println!();
    for index in 0..tetgen.out_ncell() {
        print!("Cell {} ({}): (", index, tetgen.out_cell_attribute(index));
        for m in 0..tetgen.out_cell_npoint() {
            if m > 0 {
                print!(", ");
            }
            print!("{}", tetgen.out_cell_point(index, m));
        }
        println!(")");
    }

    // draw edges of tetrahedra
    let mut plot = Plot::new();
    tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/tmp/tritet/example_tetgen_delaunay_1.svg")?;
    Ok(())
}
