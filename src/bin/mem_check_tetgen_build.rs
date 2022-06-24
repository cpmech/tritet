use tritet::{StrError, Tetgen};

fn main() -> Result<(), StrError> {
    println!("Running Mem Check on Tetgen\n");
    let _tet = Tetgen::new(4, None, None, None)?;
    println!("Done\n");
    Ok(())
}
