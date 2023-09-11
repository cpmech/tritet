use plotpy::Plot;
use tritet::{StrError, Trigen};

fn main() -> Result<(), StrError> {
    // allocate data for 10 points
    let mut trigen = Trigen::new(10, None, None, None)?;

    // set points
    trigen
        .set_point(0, 0, 0.478554, 0.00869692)?
        .set_point(1, 0, 0.13928, 0.180603)?
        .set_point(2, 0, 0.578587, 0.760349)?
        .set_point(3, 0, 0.903726, 0.975904)?
        .set_point(4, 0, 0.0980015, 0.981755)?
        .set_point(5, 0, 0.133721, 0.348832)?
        .set_point(6, 0, 0.648071, 0.369534)?
        .set_point(7, 0, 0.230951, 0.558482)?
        .set_point(8, 0, 0.0307942, 0.459123)?
        .set_point(9, 0, 0.540745, 0.331184)?;

    // generate Delaunay triangulation
    trigen.generate_delaunay(false)?;

    // print coordinates
    println!("vector<vector<vector<double>>> triangles = {{");
    for index in 0..trigen.ntriangle() {
        if index != 0 {
            print!(",\n");
        }
        print!("    {{");
        for m in 0..3 {
            if m != 0 {
                print!(", ");
            }
            let p = trigen.triangle_node(index, m);
            let (_, x, y) = trigen.point(p);
            print!("{{{},{}}}", x, y);
        }
        print!("}}");
    }
    println!("}};");

    // draw triangles
    let mut plot = Plot::new();
    trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save("/tmp/tritet/example_triangles_print_coords.svg")?;
    Ok(())
}
