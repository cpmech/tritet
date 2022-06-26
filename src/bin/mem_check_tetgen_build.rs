use std::thread;
use tritet::{StrError, Tetgen};

fn main() {
    println!("Running Mem Check on Tetgen\n");
    let mut handles = Vec::new();

    for i in 0..20 {
        let handle = thread::spawn(move || {
            println!("..{}..", i);
            run_all().unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nDone\n");
}

fn run_all() -> Result<(), StrError> {
    let _tet = Tetgen::new(4, Some(vec![3, 3, 3, 3]), Some(1), Some(1))?;
    new_captures_some_errors();
    set_point_captures_some_errors()?;
    set_facet_point_captures_some_errors()?;
    set_region_captures_some_errors()?;
    set_hole_captures_some_errors()?;
    generate_methods_capture_some_errors()?;
    generate_delaunay_works()?;
    generate_mesh_works_1()?;
    Ok(())
}

fn new_captures_some_errors() {
    assert_eq!(
        Tetgen::new(3, None, None, None).err(),
        Some("npoint must be ≥ 4")
    );
    assert_eq!(
        Tetgen::new(4, Some(vec![3, 3, 3]), None, None).err(),
        Some("nfacet must be ≥ 4")
    );
    assert_eq!(
        Tetgen::new(4, Some(vec![3, 3, 3, 2]), None, None).err(),
        Some("facet npoint must be ≥ 3")
    );
}

fn set_point_captures_some_errors() -> Result<(), StrError> {
    let mut tetgen = Tetgen::new(4, None, None, None)?;
    assert_eq!(
        tetgen.set_point(5, 0.0, 0.0, 0.0).err(),
        Some("index of point is out of bounds")
    );
    Ok(())
}

fn set_facet_point_captures_some_errors() -> Result<(), StrError> {
    let mut tetgen = Tetgen::new(4, None, None, None)?;
    assert_eq!(
        tetgen.set_facet_point(0, 0, 0).err(),
        Some("cannot set facet point because facet_npoint is None")
    );
    let mut tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), None, None)?;
    assert_eq!(
        tetgen.set_facet_point(5, 0, 0).err(),
        Some("index of facet is out of bounds")
    );
    assert_eq!(
        tetgen.set_facet_point(0, 4, 0).err(),
        Some("index of facet point is out of bounds")
    );
    assert_eq!(
        tetgen.set_facet_point(0, 0, 5).err(),
        Some("id of facet point is out of bounds")
    );
    Ok(())
}

fn set_region_captures_some_errors() -> Result<(), StrError> {
    let mut tetgen = Tetgen::new(4, None, None, None)?;
    assert_eq!(
        tetgen.set_region(0, 0.33, 0.33, 0.33, 1, Some(0.1)).err(),
        Some("cannot set region because the number of regions is None")
    );
    let mut tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), Some(1), None)?;
    assert_eq!(
        tetgen.set_region(1, 0.33, 0.33, 0.33, 1, Some(0.1)).err(),
        Some("index of region is out of bounds")
    );
    Ok(())
}

fn set_hole_captures_some_errors() -> Result<(), StrError> {
    let mut tetgen = Tetgen::new(4, None, None, None)?;
    assert_eq!(
        tetgen.set_hole(0, 0.33, 0.33, 0.33).err(),
        Some("cannot set hole because the number of holes is None")
    );
    let mut tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), Some(1), Some(1))?;
    assert_eq!(
        tetgen.set_hole(1, 0.33, 0.33, 0.33).err(),
        Some("index of hole is out of bounds")
    );
    Ok(())
}

fn generate_methods_capture_some_errors() -> Result<(), StrError> {
    let mut tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), None, None)?;
    assert_eq!(
        tetgen.generate_delaunay(false).err(),
        Some("cannot generate Delaunay tetrahedralization because not all points are set")
    );
    assert_eq!(
        tetgen.generate_mesh(false, false, None, None).err(),
        Some("cannot generate mesh of tetrahedra because not all points are set")
    );
    tetgen
        .set_point(0, 0.0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0, 0.0)?
        .set_point(2, 0.0, 1.0, 0.0)?
        .set_point(3, 0.0, 0.0, 1.0)?;
    assert_eq!(
        tetgen.generate_mesh(false, false, None, None).err(),
        Some("cannot generate mesh of tetrahedra because not all facets are set")
    );
    Ok(())
}

fn generate_delaunay_works() -> Result<(), StrError> {
    let mut tetgen = Tetgen::new(4, None, None, None)?;
    tetgen
        .set_point(0, 0.0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0, 0.0)?
        .set_point(2, 0.0, 1.0, 0.0)?
        .set_point(3, 0.0, 0.0, 1.0)?;
    tetgen.generate_delaunay(false)?;
    assert_eq!(tetgen.ntet(), 1);
    assert_eq!(tetgen.npoint(), 4);
    Ok(())
}

fn generate_mesh_works_1() -> Result<(), StrError> {
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
    tetgen.set_region(0, -0.9, -0.9, -0.9, 1, None)?;
    tetgen.set_hole(0, 0.5, 0.5, 0.5)?;
    tetgen.generate_mesh(false, false, None, None)?;
    assert_eq!(tetgen.ntet(), 116);
    assert_eq!(tetgen.npoint(), 50);
    Ok(())
}
