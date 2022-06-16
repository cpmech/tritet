use tritet::{StrError, Triangle};

fn main() -> Result<(), StrError> {
    println!("Running Mem Check\n");

    let mut delaunay = Triangle::new(3, None, None, None)?;
    delaunay
        .set_point(0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0)?
        .set_point(2, 0.0, 1.0)?;
    delaunay.generate_delaunay(false)?;

    let mut voronoi = Triangle::new(3, None, None, None)?;
    voronoi
        .set_point(0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0)?
        .set_point(2, 0.0, 1.0)?;
    voronoi.generate_voronoi(false)?;

    let mut mesh = Triangle::new(3, Some(3), None, None)?;
    mesh.set_point(0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0)?
        .set_point(2, 0.0, 1.0)?;
    mesh.set_segment(0, 0, 1)?
        .set_segment(1, 1, 2)?
        .set_segment(2, 2, 0)?;
    mesh.generate_mesh(false, false, None, None)?;
    println!("Done\n");
    Ok(())
}
