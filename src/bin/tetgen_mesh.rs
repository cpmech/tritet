use plotpy::Plot;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use tritet::{InputDataTetMesh, StrError, Tetgen};

/// Command line options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "tetgen_mesh",
    about = "Generate tetrahedral mesh using TetGen and export to VTU format"
)]
struct Options {
    /// Input JSON file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output directory
    out_dir: String,

    /// Verbose mode
    #[structopt(short = "V", long)]
    verbose: bool,

    /// Generate second order tetrahedra
    #[structopt(short, long)]
    o2: bool,

    /// Generate SVG figure with the wireframe of the mesh
    #[structopt(short, long)]
    svg_figure: bool,

    /// Global maximum volume for tetrahedra
    #[structopt(short = "v", long)]
    max_volume: Option<f64>,

    /// Global minimum angle between faces of tetrahedra
    #[structopt(short = "a", long)]
    min_angle: Option<f64>,
}

fn main() -> Result<(), StrError> {
    // parse options
    let options = Options::from_args();

    // load input data from JSON file
    let in_path = Path::new(&options.input);
    let input_data = InputDataTetMesh::read_json(&in_path)?;
    let fn_stem = in_path
        .file_stem()
        .ok_or("cannot get file stem")?
        .to_str()
        .ok_or("cannot convert file stem to str")?;

    // allocate generator from input data
    let tetgen = Tetgen::from_input_data(&input_data)?;

    // generate mesh
    let verbose = options.verbose;
    let o2 = options.o2;
    tetgen.generate_mesh(verbose, o2, options.max_volume, options.min_angle)?;

    // write SVG file with wireframe
    if options.svg_figure {
        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, false, false, false, None, None, None);
        plot.set_equal_axes(true)
            .set_figure_size_points(800.0, 800.0)
            .save(&format!("{}/{}.svg", options.out_dir, fn_stem))?;
        println!("\nGenerated SVG file: {}/{}.svg", options.out_dir, fn_stem);
    }

    // write VTU file
    let path_vtu = format!("{}/{}.vtu", options.out_dir, fn_stem);
    tetgen.write_vtu(&path_vtu)?;
    println!("\nGenerated VTU file: {}/{}.vtu", options.out_dir, fn_stem);
    Ok(())
}
