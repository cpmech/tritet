# Triangle and tetrahedron mesh generators

[![codecov](https://codecov.io/gh/cpmech/tritet/branch/main/graph/badge.svg?token=2ALVRVWJ5W)](https://codecov.io/gh/cpmech/tritet)
[![Test & Coverage](https://github.com/cpmech/tritet/actions/workflows/test_and_coverage.yml/badge.svg)](https://github.com/cpmech/tritet/actions/workflows/test_and_coverage.yml)
[![Windows & macOS](https://github.com/cpmech/tritet/actions/workflows/windows_and_macos.yml/badge.svg)](https://github.com/cpmech/tritet/actions/workflows/windows_and_macos.yml)

## Contents

* [Introduction](#introduction)
* [Installation](#installation)
* [Setting Cargo.toml](#cargo)
* [Examples](#examples)
* [For developers](#developers)

## <a name="introduction"></a> Introduction

This crate implements Triangle and Tetrahedron mesh generators by wrapping the best tools around, namely, [Triangle](https://www.cs.cmu.edu/~quake/triangle.html) and [Tetgen](http://tetgen.org/).

Here, all the data structures accessed by the C/C++ codes are allocated on the "C-side" by (carefully) using "malloc/new." 😅 We then make use of [Valgrind](https://valgrind.org/) and tests to make sure that there are no leaks. In this way, there is no performance loss of the C-code while enabling the convenience of Rust.

The resulting Rust interface to Triangle and Tetgen is somewhat low-level. However, other projects could use this interface to make higher-level functions.

The code works in multithreaded applications---not exhaustively verified but tested. See, for example, the comprehensive tests in [mem_check_triangle_build.rs](https://github.com/cpmech/tritet/blob/main/src/bin/mem_check_triangle_build.rs) and [mem_check_tetgen_build.rs](https://github.com/cpmech/tritet/blob/main/src/bin/mem_check_tetgen_build.rs)

A higher-level crate is available for mesh generation (and more): [Gemlab: Geometry, meshes, and numerical integration for finite element analyses](https://github.com/cpmech/gemlab).

See the documentation for further information:

- [Tritet documentation](https://docs.rs/tritet) - Contains the API reference and examples

## <a name="installation"></a> Installation

Install some libraries:

```bash
sudo apt install build-essential
```

## <a name="cargo"></a> Setting Cargo.toml

[![Crates.io](https://img.shields.io/crates/v/tritet.svg)](https://crates.io/crates/tritet)

👆 Check the crate version and update your Cargo.toml accordingly:

```toml
[dependencies]
tritet = "*"
```

## <a name="examples"></a> Examples

Note: set `SAVE_FIGURE` to true to generate the figures.

### 2D Delaunay triangulation

```rust
use plotpy::Plot;
use tritet::{StrError, Trigen};

const SAVE_FIGURE: bool = false;

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

    // draw triangles
    if SAVE_FIGURE {
        let mut plot = Plot::new();
        trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/doc_triangle_delaunay_1.svg")?;
    }
    Ok(())
}
```

![doc_triangle_delaunay_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_delaunay_1.svg)

### 2D Voronoi tessellation

```rust
use plotpy::Plot;
use tritet::{StrError, Trigen};

const SAVE_FIGURE: bool = false;

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

    // generate Voronoi tessellation
    trigen.generate_voronoi(false)?;

    // draw Voronoi diagram
    if SAVE_FIGURE {
        let mut plot = Plot::new();
        trigen.draw_voronoi(&mut plot);
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/doc_triangle_voronoi_1.svg")?;
    }
    Ok(())
}
```

![doc_triangle_voronoi_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_voronoi_1.svg)

### 2D mesh generation

```rust
use plotpy::Plot;
use tritet::{StrError, Trigen};

const SAVE_FIGURE: bool = false;

fn main() -> Result<(), StrError> {
    // allocate data for 12 points, 10 segments, 2 regions, and 1 hole
    let mut trigen = Trigen::new(12, Some(10), Some(2), Some(1))?;

    // set points
    trigen
        .set_point(0, 0, 0.0, 0.0)?
        .set_point(1, 0, 1.0, 0.0)?
        .set_point(2, 0, 1.0, 1.0)?
        .set_point(3, 0, 0.0, 1.0)?
        .set_point(4, 0, 0.2, 0.2)?
        .set_point(5, 0, 0.8, 0.2)?
        .set_point(6, 0, 0.8, 0.8)?
        .set_point(7, 0, 0.2, 0.8)?
        .set_point(8, 0, 0.0, 0.5)?
        .set_point(9, 0, 0.2, 0.5)?
        .set_point(10, 0, 0.8, 0.5)?
        .set_point(11, 0, 1.0, 0.5)?;

    // set segments
    trigen
        .set_segment(0, -1, 0, 1)?
        .set_segment(1, -1, 1, 2)?
        .set_segment(2, -1, 2, 3)?
        .set_segment(3, -1, 3, 0)?
        .set_segment(4, -1, 4, 5)?
        .set_segment(5, -1, 5, 6)?
        .set_segment(6, -1, 6, 7)?
        .set_segment(7, -1, 7, 4)?
        .set_segment(8, -1, 8, 9)?
        .set_segment(9, -1, 10, 11)?;

    // set regions
    trigen
        .set_region(0, 1, 0.1, 0.1, None)?
        .set_region(1, 2, 0.1, 0.9, None)?;

    // set holes
    trigen.set_hole(0, 0.5, 0.5)?;

    // generate o2 mesh without constraints
    trigen.generate_mesh(false, true, false, None, None)?;

    // draw mesh
    if SAVE_FIGURE {
        let mut plot = Plot::new();
        trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/doc_triangle_mesh_1.svg")?;
    }
    Ok(())
}
```

![doc_triangle_mesh_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_mesh_1.svg)

## 3D Delaunay triangulation

```rust
use plotpy::Plot;
use tritet::{StrError, Tetgen};

const SAVE_FIGURE: bool = false;

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

    // draw edges of tetrahedra
    if SAVE_FIGURE {
        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/example_tetgen_delaunay_1.svg")?;
    }
    Ok(())
}
```

![example_tetgen_delaunay_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/example_tetgen_delaunay_1.svg)

## 3D mesh generation

Note: set `SAVE_VTU_FILE` to true to generate Paraview file.

```rust
use plotpy::Plot;
use tritet::{StrError, Tetgen};

const SAVE_VTU_FILE: bool = false;
const SAVE_FIGURE: bool = false;

fn main() -> Result<(), StrError> {
    // allocate data for 16 points and 12 facets
    // (one cube/hole inside another cube)
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
        .set_point(0, 0, 0.0, 0.0, 0.0)?
        .set_point(1, 0, 1.0, 0.0, 0.0)?
        .set_point(2, 0, 1.0, 1.0, 0.0)?
        .set_point(3, 0, 0.0, 1.0, 0.0)?
        .set_point(4, 0, 0.0, 0.0, 1.0)?
        .set_point(5, 0, 1.0, 0.0, 1.0)?
        .set_point(6, 0, 1.0, 1.0, 1.0)?
        .set_point(7, 0, 0.0, 1.0, 1.0)?;

    // outer cube
    tetgen
        .set_point(8,  0, -1.0, -1.0, -1.0)?
        .set_point(9,  0, 2.0, -1.0, -1.0)?
        .set_point(10, 0, 2.0, 2.0, -1.0)?
        .set_point(11, 0, -1.0, 2.0, -1.0)?
        .set_point(12, 0, -1.0, -1.0, 2.0)?
        .set_point(13, 0, 2.0, -1.0, 2.0)?
        .set_point(14, 0, 2.0, 2.0, 2.0)?
        .set_point(15, 0, -1.0, 2.0, 2.0)?;

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

    // set region and hole
    tetgen.set_region(0, 1, -0.9, -0.9, -0.9, None)?;
    tetgen.set_hole(0, 0.5, 0.5, 0.5)?;

    // generate mesh
    tetgen.generate_mesh(false, false, None, None)?;

    // generate file for Paraview
    if SAVE_VTU_FILE {
        tetgen.write_vtu("/tmp/tritet/example_tetgen_mesh_1.vtu")?;
    }

    // draw edges of tetrahedra
    if SAVE_FIGURE {
        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        plot.set_equal_axes(true)
            .set_figure_size_points(600.0, 600.0)
            .save("/tmp/tritet/example_tetgen_mesh_1.svg")?;
    }
    Ok(())
}
```

![example_tetgen_mesh_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/example_tetgen_mesh_1.svg)

## <a name="developers"></a> For developers

Install cargo-valgrind:

```bash
cargo install cargo-valgrind
```

Then check for memory leaks (none ;-):

```bash
bash memcheck.bash
```
