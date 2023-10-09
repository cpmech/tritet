use std::thread;
use std::time::Duration;
use tritet::{StrError, Trigen};

fn main() {
    println!("Running Mem Check on Triangle\n");
    let mut handles = Vec::new();

    for i in 0..20 {
        let handle = thread::spawn(move || {
            println!("..{}..", i);
            new_captures_some_errors();
            set_point_captures_some_errors().unwrap();
            set_segment_captures_some_errors().unwrap();
            set_region_captures_some_errors().unwrap();
            set_hole_captures_some_errors().unwrap();
            generate_methods_capture_some_errors().unwrap();
            delaunay().unwrap();
            voronoi().unwrap();
            mesh().unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    thread::sleep(Duration::from_millis(250));
    println!("Done\n");
}

fn new_captures_some_errors() {
    assert_eq!(Trigen::new(2, None, None, None).err(), Some("npoint must be ≥ 3"));
    assert_eq!(Trigen::new(3, Some(2), None, None).err(), Some("nsegment must be ≥ 3"));
}

fn set_point_captures_some_errors() -> Result<(), StrError> {
    let mut trigen = Trigen::new(3, None, None, None)?;
    assert_eq!(
        trigen.set_point(4, 0, 0.0, 0.0).err(),
        Some("index of point is out of bounds")
    );
    Ok(())
}

fn set_segment_captures_some_errors() -> Result<(), StrError> {
    let mut trigen = Trigen::new(3, None, None, None)?;
    assert_eq!(
        trigen.set_segment(0, -10, 0, 1).err(),
        Some("cannot set segment because the number of segments is None")
    );
    let mut trigen = Trigen::new(3, Some(3), None, None)?;
    assert_eq!(
        trigen.set_segment(4, -20, 0, 1).err(),
        Some("index of segment is out of bounds")
    );
    assert_eq!(
        trigen.set_segment(0, -30, 0, 4).err(),
        Some("id of segment point is out of bounds")
    );
    Ok(())
}

fn set_region_captures_some_errors() -> Result<(), StrError> {
    let mut trigen = Trigen::new(3, None, None, None)?;
    assert_eq!(
        trigen.set_region(0, 1, 0.33, 0.33, Some(0.1)).err(),
        Some("cannot set region because the number of regions is None")
    );
    let mut trigen = Trigen::new(3, Some(3), Some(1), None)?;
    assert_eq!(
        trigen.set_region(1, 1, 0.33, 0.33, Some(0.1)).err(),
        Some("index of region is out of bounds")
    );
    Ok(())
}

fn set_hole_captures_some_errors() -> Result<(), StrError> {
    let mut trigen = Trigen::new(3, None, None, None)?;
    assert_eq!(
        trigen.set_hole(0, 0.33, 0.33).err(),
        Some("cannot set hole because the number of holes is None")
    );
    let mut triangle = Trigen::new(3, Some(3), Some(1), Some(1))?;
    assert_eq!(
        triangle.set_hole(1, 0.33, 0.33).err(),
        Some("index of hole is out of bounds")
    );
    Ok(())
}

fn generate_methods_capture_some_errors() -> Result<(), StrError> {
    let mut trigen = Trigen::new(3, Some(3), None, None)?;
    assert_eq!(
        trigen.generate_delaunay(false).err(),
        Some("cannot generate Delaunay triangulation because not all points are set")
    );
    assert_eq!(
        trigen.generate_voronoi(false).err(),
        Some("cannot generate Voronoi tessellation because not all points are set")
    );
    assert_eq!(
        trigen.generate_mesh(false, false, true, None, None).err(),
        Some("cannot generate mesh of triangles because not all points are set")
    );
    trigen
        .set_point(0, 0, 0.0, 0.0)?
        .set_point(1, 0, 1.0, 0.0)?
        .set_point(2, 0, 0.0, 1.0)?;
    assert_eq!(
        trigen.generate_mesh(false, false, true, None, None).err(),
        Some("cannot generate mesh of triangles because not all segments are set")
    );
    Ok(())
}

fn delaunay() -> Result<(), StrError> {
    let mut delaunay = Trigen::new(15, None, None, None)?;
    delaunay
        .set_point(0, 0, 0.0, 0.0)?
        .set_point(1, 0, -0.416, 0.909)?
        .set_point(2, 0, -1.35, 0.436)?
        .set_point(3, 0, -1.64, -0.549)?
        .set_point(4, 0, -1.31, -1.51)?
        .set_point(5, 0, -0.532, -2.17)?
        .set_point(6, 0, 0.454, -2.41)?
        .set_point(7, 0, 1.45, -2.21)?
        .set_point(8, 0, 2.29, -1.66)?
        .set_point(9, 0, 2.88, -0.838)?
        .set_point(10, 0, 3.16, 0.131)?
        .set_point(11, 0, 3.12, 1.14)?
        .set_point(12, 0, 2.77, 2.08)?
        .set_point(13, 0, 2.16, 2.89)?
        .set_point(14, 0, 1.36, 3.49)?;
    delaunay.generate_delaunay(false)
}

fn voronoi() -> Result<(), StrError> {
    let mut voronoi = Trigen::new(100, None, None, None)?;
    voronoi
        .set_point(0, 0, 0.0476694, 0.809168)?
        .set_point(1, 0, -0.0412985, 0.0934087)?
        .set_point(2, 0, 0.771124, -0.145541)?
        .set_point(3, 0, -0.00285913, -0.0054207)?
        .set_point(4, 0, 0.0121534, 0.391051)?
        .set_point(5, 0, 0.189257, -0.721248)?
        .set_point(6, 0, 0.00346951, -0.117197)?
        .set_point(7, 0, -0.0557166, -0.0167348)?
        .set_point(8, 0, 0.0914024, -0.764985)?
        .set_point(9, 0, -0.732465, -0.0296379)?
        .set_point(10, 0, 0.620321, 0.456789)?
        .set_point(11, 0, -0.00897789, -0.0231625)?
        .set_point(12, 0, 0.611961, -0.736103)?
        .set_point(13, 0, -0.586524, 0.587304)?
        .set_point(14, 0, 0.0434815, -0.0359369)?
        .set_point(15, 0, -0.235574, -0.759667)?
        .set_point(16, 0, -0.311492, -0.401672)?
        .set_point(17, 0, 0.00879549, -0.00548149)?
        .set_point(18, 0, 0.214277, -0.176567)?
        .set_point(19, 0, -0.576379, 0.654919)?
        .set_point(20, 0, 0.329429, 0.314783)?
        .set_point(21, 0, 0.0272183, -0.0335721)?
        .set_point(22, 0, 0.651159, 0.0837685)?
        .set_point(23, 0, 0.00448275, 0.00783356)?
        .set_point(24, 0, 0.372467, 0.586735)?
        .set_point(25, 0, 0.0200959, -0.0736717)?
        .set_point(26, 0, -0.0671954, 0.534502)?
        .set_point(27, 0, 0.163769, 0.104278)?
        .set_point(28, 0, -0.00430444, -0.00429822)?
        .set_point(29, 0, 0.0697276, 0.145652)?
        .set_point(30, 0, -0.0501914, -0.516296)?
        .set_point(31, 0, 0.0954772, -0.22419)?
        .set_point(32, 0, -0.0131771, -0.0113541)?
        .set_point(33, 0, 0.144833, -0.0414348)?
        .set_point(34, 0, -0.1656, -0.109273)?
        .set_point(35, 0, 0.0294145, -0.119617)?
        .set_point(36, 0, -0.388868, 0.174542)?
        .set_point(37, 0, 0.0216939, -0.00054628)?
        .set_point(38, 0, 0.449451, 0.73811)?
        .set_point(39, 0, 0.559539, -0.376405)?
        .set_point(40, 0, -0.805688, -0.196454)?
        .set_point(41, 0, -0.0523838, -0.357019)?
        .set_point(42, 0, 0.0471204, -0.134888)?
        .set_point(43, 0, 0.0428721, -0.0261849)?
        .set_point(44, 0, 0.0368263, 0.0935173)?
        .set_point(45, 0, 0.779577, -0.215466)?
        .set_point(46, 0, -0.682904, -0.479713)?
        .set_point(47, 0, 0.259023, 0.462227)?
        .set_point(48, 0, 0.110553, 0.185891)?
        .set_point(49, 0, 0.21271, 0.40305)?
        .set_point(50, 0, 0.310775, 0.0032405)?
        .set_point(51, 0, -0.0799817, 0.747664)?
        .set_point(52, 0, -0.431582, 0.100479)?
        .set_point(53, 0, -0.207633, -0.0535168)?
        .set_point(54, 0, -0.103873, -0.16392)?
        .set_point(55, 0, -0.0808649, -0.0833543)?
        .set_point(56, 0, -0.0482698, 0.00926695)?
        .set_point(57, 0, -0.112805, -0.206202)?
        .set_point(58, 0, 0.0928734, -0.0960191)?
        .set_point(59, 0, -0.631549, -0.00643761)?
        .set_point(60, 0, -0.227293, -0.835806)?
        .set_point(61, 0, -0.0333289, 0.0616227)?
        .set_point(62, 0, -0.0942452, -0.332817)?
        .set_point(63, 0, 0.199281, 0.0817346)?
        .set_point(64, 0, 0.0413125, 0.874436)?
        .set_point(65, 0, -6.9375e-05, -9.5e-06)?
        .set_point(66, 0, -0.424367, -0.241631)?
        .set_point(67, 0, 0.56258, -0.439865)?
        .set_point(68, 0, 0.274475, 0.234625)?
        .set_point(69, 0, 0.0499112, 0.30348)?
        .set_point(70, 0, 0.00860505, 0.139826)?
        .set_point(71, 0, -0.106809, -0.610516)?
        .set_point(72, 0, -0.219089, -0.0453384)?
        .set_point(73, 0, -0.349079, 0.275986)?
        .set_point(74, 0, 0.382869, -0.735405)?
        .set_point(75, 0, -0.0614569, 0.109208)?
        .set_point(76, 0, -0.822608, -0.478913)?
        .set_point(77, 0, 0.0456648, -0.115802)?
        .set_point(78, 0, 0.244877, 0.00235373)?
        .set_point(79, 0, 0.272695, -0.160362)?
        .set_point(80, 0, 0.64381, -0.539716)?
        .set_point(81, 0, -0.000474647, -0.00122888)?
        .set_point(82, 0, -0.316246, -0.428132)?
        .set_point(83, 0, 0.180288, -0.0356826)?
        .set_point(84, 0, 0.134306, 0.120321)?
        .set_point(85, 0, -0.580926, -0.297724)?
        .set_point(86, 0, -0.0734621, 0.287079)?
        .set_point(87, 0, 0.0152062, 0.389861)?
        .set_point(88, 0, -0.0904595, -0.318536)?
        .set_point(89, 0, -0.157713, 0.0694107)?
        .set_point(90, 0, -0.00940586, -0.0319491)?
        .set_point(91, 0, -0.784887, -0.0922512)?
        .set_point(92, 0, 0.0435008, -0.0997158)?
        .set_point(93, 0, 0.363509, -0.68881)?
        .set_point(94, 0, 0.22618, 0.39209)?
        .set_point(95, 0, 0.264525, -0.326457)?
        .set_point(96, 0, 0.154736, 0.0507695)?
        .set_point(97, 0, -0.150901, 0.717167)?
        .set_point(98, 0, 0.0532971, -0.800056)?
        .set_point(99, 0, 0.17173, 0.0431868)?;
    voronoi.generate_voronoi(false)
}

fn mesh() -> Result<(), StrError> {
    // allocate data for 26 points, 22 segments, 1 region, and 3 holes
    let mut mesh = Trigen::new(26, Some(22), Some(1), Some(3))?;

    // the outer polyhedron
    mesh.set_point(0, 0, 80.0, 0.0)?
        .set_point(1, 0, 100.0, 50.0)?
        .set_point(2, 0, 0.0, 100.0)?
        .set_point(3, 0, -100.0, 50.0)?
        .set_point(4, 0, -80.0, 0.0)?
        .set_point(5, 0, -100.0, -50.0)?
        .set_point(6, 0, 0.0, -100.0)?
        .set_point(7, 0, 100.0, -50.0)?;
    // the mouth
    mesh.set_point(8, 0, 0.0, -90.0)?
        .set_point(9, 0, 80.0, -50.0)?
        .set_point(10, 0, 0.0, -10.0)?
        .set_point(11, 0, -80.0, -50.0)?;
    // the left eye
    mesh.set_point(12, 0, -70.0, 50.0)?
        .set_point(13, 0, -60.0, 30.0)?
        .set_point(14, 0, -10.0, 55.0)?
        .set_point(15, 0, -40.0, 55.0)?;
    // the right eye
    mesh.set_point(16, 0, 70.0, 50.0)?
        .set_point(17, 0, 60.0, 30.0)?
        .set_point(18, 0, 10.0, 55.0)?
        .set_point(19, 0, 40.0, 55.0)?;
    // two nostril segments
    mesh.set_point(20, 0, -10.0, 25.0)?
        .set_point(21, 0, -20.0, -10.0)?
        .set_point(22, 0, 10.0, 25.0)?
        .set_point(23, 0, 20.0, -10.0)?;
    // two dimples
    mesh.set_point(24, 0, -50.0, 0.0)?.set_point(25, 0, 50.0, 0.0)?;

    // the outer polyhedron
    mesh.set_segment(0, 0, 0, 1)?
        .set_segment(1, 0, 1, 2)?
        .set_segment(2, 0, 2, 3)?
        .set_segment(3, 0, 3, 4)?
        .set_segment(4, 0, 4, 5)?
        .set_segment(5, 0, 5, 6)?
        .set_segment(6, 0, 6, 7)?
        .set_segment(7, 0, 7, 0)?;
    // the mouth
    mesh.set_segment(8, 0, 8, 9)?
        .set_segment(9, 0, 9, 10)?
        .set_segment(10, 0, 10, 11)?
        .set_segment(11, 0, 11, 8)?;
    // the left eye
    mesh.set_segment(12, 0, 12, 13)?
        .set_segment(13, 0, 13, 14)?
        .set_segment(14, 0, 14, 15)?
        .set_segment(15, 0, 15, 12)?;
    // the right eye
    mesh.set_segment(16, 0, 16, 17)?
        .set_segment(17, 0, 17, 18)?
        .set_segment(18, 0, 18, 19)?
        .set_segment(19, 0, 19, 16)?;
    // two nostril segments
    mesh.set_segment(20, 0, 20, 21)?.set_segment(21, 0, 22, 23)?;

    // region
    mesh.set_region(0, 1, 0.0, 0.0, None)?;

    // three holes
    mesh.set_hole(0, 0.0, -50.0)? // mouth
        .set_hole(1, -50.0, 50.0)? // left eye
        .set_hole(2, 50.0, 50.0)?; // right eye

    // generate mesh without constraints
    mesh.generate_mesh(false, true, true, None, None)
}
