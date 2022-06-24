# Triangle and tetrahedron mesh generators

This project presents a Rust code to generate triangle and tetrahedron meshes by calling
[Triangle](https://www.cs.cmu.edu/~quake/triangle.html) and
[Tetgen](http://tetgen.org/).
However, the code here does not create a one-to-one interface to these great libraries.
Also, this create tries to keep it as simple as possible.

One important aspect of this crate is that all the data structures accessed by the C-code
are allocated on the "C-side", by (carefully) using "malloc." 😅
Therefore, there are no "pointers" going forth and back from Rust to C.
We then make use of [Valgrind](https://valgrind.org/) and tests to make sure that all is (hopefully) well.

The resulting Rust interface to Triangle and Tetgen is a lightweight and low-level set of functions
that can be used by other more "high-level" projects.

This crate is used by [Gemlab: Geometry, meshes, and numerical integration for finite element analyses](https://github.com/cpmech/gemlab).

## Installation

Install some libraries:

```bash
sudo apt install build-essential
```

Add this to your Cargo.toml:

```toml
[dependencies]
tritet = "0.1"
```

## Examples

### Delaunay triangulation

```rust
use plotpy::Plot;
use tritet::{StrError, Triangle};

fn main() -> Result<(), StrError> {
    // allocate data for 10 points
    let mut triangle = Triangle::new(10, None, None, None)?;

    // set points
    triangle
        .set_point(0, 0.478554, 0.00869692)?
        .set_point(1, 0.13928, 0.180603)?
        .set_point(2, 0.578587, 0.760349)?
        .set_point(3, 0.903726, 0.975904)?
        .set_point(4, 0.0980015, 0.981755)?
        .set_point(5, 0.133721, 0.348832)?
        .set_point(6, 0.648071, 0.369534)?
        .set_point(7, 0.230951, 0.558482)?
        .set_point(8, 0.0307942, 0.459123)?
        .set_point(9, 0.540745, 0.331184)?;

    // generate Delaunay triangulation
    triangle.generate_delaunay(false)?;

    // draw triangles
    let mut plot = Plot::new();
    triangle.draw_triangles(&mut plot, true, true, true, true, None, None, None);
    // plot.set_equal_axes(true)
    //     .set_figure_size_points(600.0, 600.0)
    //     .save("/tmp/tritet/doc_triangle_delaunay_1.svg")?;
    Ok(())
}
```

![doc_triangle_delaunay_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_delaunay_1.svg)

### Voronoi tessellation

```rust
use plotpy::Plot;
use tritet::{StrError, Triangle};

fn main() -> Result<(), StrError> {
    // allocate data for 10 points
    let mut triangle = Triangle::new(10, None, None, None)?;

    // set points
    triangle
        .set_point(0, 0.478554, 0.00869692)?
        .set_point(1, 0.13928, 0.180603)?
        .set_point(2, 0.578587, 0.760349)?
        .set_point(3, 0.903726, 0.975904)?
        .set_point(4, 0.0980015, 0.981755)?
        .set_point(5, 0.133721, 0.348832)?
        .set_point(6, 0.648071, 0.369534)?
        .set_point(7, 0.230951, 0.558482)?
        .set_point(8, 0.0307942, 0.459123)?
        .set_point(9, 0.540745, 0.331184)?;

    // generate Voronoi tessellation
    triangle.generate_voronoi(false)?;

    // draw Voronoi diagram
    let mut plot = Plot::new();
    triangle.draw_voronoi(&mut plot);
    // plot.set_equal_axes(true)
    //     .set_figure_size_points(600.0, 600.0)
    //     .save("/tmp/tritet/doc_triangle_voronoi_1.svg")?;
    Ok(())
}
```

![doc_triangle_voronoi_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_voronoi_1.svg)

### Mesh generation

```rust
use plotpy::Plot;
use tritet::{StrError, Triangle};

fn main() -> Result<(), StrError> {
    // allocate data for 12 points, 10 segments, 2 regions, and 1 hole
    let mut triangle = Triangle::new(12, Some(10), Some(2), Some(1))?;

    // set points
    triangle
        .set_point(0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0)?
        .set_point(2, 1.0, 1.0)?
        .set_point(3, 0.0, 1.0)?
        .set_point(4, 0.2, 0.2)?
        .set_point(5, 0.8, 0.2)?
        .set_point(6, 0.8, 0.8)?
        .set_point(7, 0.2, 0.8)?
        .set_point(8, 0.0, 0.5)?
        .set_point(9, 0.2, 0.5)?
        .set_point(10, 0.8, 0.5)?
        .set_point(11, 1.0, 0.5)?;

    // set segments
    triangle
        .set_segment(0, 0, 1)?
        .set_segment(1, 1, 2)?
        .set_segment(2, 2, 3)?
        .set_segment(3, 3, 0)?
        .set_segment(4, 4, 5)?
        .set_segment(5, 5, 6)?
        .set_segment(6, 6, 7)?
        .set_segment(7, 7, 4)?
        .set_segment(8, 8, 9)?
        .set_segment(9, 10, 11)?;

    // set regions
    triangle
        .set_region(0, 0.1, 0.1, 1, None)?
        .set_region(1, 0.1, 0.9, 2, None)?;

    // set holes
    triangle.set_hole(0, 0.5, 0.5)?;

    // generate o2 mesh without constraints
    triangle.generate_mesh(false, true, None, None)?;
    assert_eq!(triangle.ntriangle(), 14);

    // draw mesh
    let mut plot = Plot::new();
    triangle.draw_triangles(&mut plot, true, true, true, true, None, None, None);
    // plot.set_equal_axes(true)
    //     .set_figure_size_points(600.0, 600.0)
    //     .save("/tmp/tritet/doc_triangle_mesh_1.svg")?;
    Ok(())
}
```

![doc_triangle_mesh_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_mesh_1.svg)
