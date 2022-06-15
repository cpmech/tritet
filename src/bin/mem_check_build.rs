use tritet::{StrError, Triangle};

fn main() -> Result<(), StrError> {
    println!("Running Mem Check\n");
    let mut triangle = Triangle::new(3, 3, 1, 0)?;
    triangle
        .set_point(0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0)?
        .set_point(2, 0.0, 1.0)?;
    triangle
        .set_segment(0, 0, 1)?
        .set_segment(1, 1, 2)?
        .set_segment(2, 2, 0)?;
    triangle.generate(false, false, None, None);
    println!("Done\n");
    Ok(())
}
