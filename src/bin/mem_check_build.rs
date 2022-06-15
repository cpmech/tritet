use tritet::{StrError, Triangle};

fn main() -> Result<(), StrError> {
    println!("Running Mem Check\n");
    Triangle::new(3, 3, 1, 0)?;
    println!("Done\n");
    Ok(())
}
