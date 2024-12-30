use crate::constants::{handle_status, DARK_COLORS, TRITET_TO_TETGEN};
use crate::conversion::to_i32;
use crate::StrError;
use plotpy::{Canvas, Plot, Text};
use std::collections::HashMap;

#[repr(C)]
pub(crate) struct ExtTetgen {
    data: [u8; 0],
    marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn tet_new_tetgen(npoint: i32, nfacet: i32, facet_npoint: *const i32, nregion: i32, nhole: i32) -> *mut ExtTetgen;
    fn tet_drop_tetgen(tetgen: *mut ExtTetgen);
    fn tet_set_point(tetgen: *mut ExtTetgen, index: i32, marker: i32, x: f64, y: f64, z: f64) -> i32;
    fn tet_set_facet_point(tetgen: *mut ExtTetgen, index: i32, m: i32, p: i32) -> i32;
    fn tet_set_facet_marker(tetgen: *mut ExtTetgen, index: i32, marker: i32) -> i32;
    fn tet_set_region(
        tetgen: *mut ExtTetgen,
        index: i32,
        attribute: i32,
        x: f64,
        y: f64,
        z: f64,
        max_volume: f64,
    ) -> i32;
    fn tet_set_hole(tetgen: *mut ExtTetgen, index: i32, x: f64, y: f64, z: f64) -> i32;
    fn tet_run_delaunay(tetgen: *mut ExtTetgen, verbose: i32) -> i32;
    fn tet_run_tetrahedralize(
        tetgen: *mut ExtTetgen,
        verbose: i32,
        o2: i32,
        global_max_volume: f64,
        global_min_angle: f64,
    ) -> i32;
    fn tet_out_npoint(tetgen: *mut ExtTetgen) -> i32;
    fn tet_out_ncell(tetgen: *mut ExtTetgen) -> i32;
    fn tet_out_cell_npoint(tetgen: *mut ExtTetgen) -> i32;
    fn tet_out_point(tetgen: *mut ExtTetgen, index: i32, dim: i32) -> f64;
    fn tet_out_point_marker(tetgen: *mut ExtTetgen, index: i32) -> i32;
    fn tet_out_cell_point(tetgen: *mut ExtTetgen, index: i32, corner: i32) -> i32;
    fn tet_out_cell_attribute(tetgen: *mut ExtTetgen, index: i32) -> i32;
    fn tet_out_n_marked_face(tetgen: *mut ExtTetgen) -> i32;
    fn tet_out_marked_face(
        tetgen: *mut ExtTetgen,
        index: i32,
        points_len_6: *mut i32,
        marker: *mut i32,
        cell: *mut i32,
    );
}

/// Implements high-level functions to call Si's Tetgen Cpp-Code
///
/// **Note:** All indices are are zero-based.
///
/// # Examples
///
/// ## Delaunay triangulation
///
/// ```
/// use plotpy::Plot;
/// use tritet::{StrError, Tetgen};
///
/// const SAVE_FIGURE: bool = false;
///
/// fn main() -> Result<(), StrError> {
///     // allocate data for 4 points
///     let mut tetgen = Tetgen::new(5, None, None, None)?;
///
///     // set points
///     tetgen
///         .set_point(0, 0, 0.0, 1.0, 0.0)?
///         .set_point(1, 0, 0.0, 0.0, 0.0)?
///         .set_point(2, 0, 1.0, 1.0, 0.0)?
///         .set_point(3, 0, 0.0, 1.0, 1.0)?
///         .set_point(4, 0, 1.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)?;
///
///     // generate Delaunay triangulation
///     tetgen.generate_delaunay(false)?;
///
///     // draw edges of tetrahedra
///     if SAVE_FIGURE{
///         let mut plot = Plot::new();
///         tetgen.draw_wireframe(&mut plot, true, true, true, false, None, None, None);
///         plot.set_equal_axes(true)
///            .set_figure_size_points(600.0, 600.0)
///            .save("/tmp/tritet/doc_tetgen_delaunay_1.svg")?;
///     }
///
///     assert_eq!(tetgen.out_ncell(), 3);
///     assert_eq!(tetgen.out_npoint(), 5);
///     Ok(())
/// }
/// ```
///
/// ![doc_tetgen_delaunay_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_tetgen_delaunay_1.svg)
///
/// ## Mesh generation
///
/// ```
/// use plotpy::Plot;
/// use tritet::{StrError, Tetgen};
///
/// const SAVE_FIGURE: bool = false;
///
/// fn main() -> Result<(), StrError> {
///     // allocate data for 4 points
///     let mut tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), Some(1), None)?;
///
///     // set points
///     tetgen
///         .set_point(0, 0, 0.0, 1.0, 0.0)?
///         .set_point(1, 0, 0.0, 0.0, 0.0)?
///         .set_point(2, 0, 1.0, 1.0, 0.0)?
///         .set_point(3, 0, 0.0, 1.0, 1.0)?;
///
///     // set facets
///     tetgen
///         .set_facet_point(0, 0, 0)?
///         .set_facet_point(0, 1, 2)?
///         .set_facet_point(0, 2, 1)?;
///     tetgen
///         .set_facet_point(1, 0, 0)?
///         .set_facet_point(1, 1, 1)?
///         .set_facet_point(1, 2, 3)?;
///     tetgen
///         .set_facet_point(2, 0, 0)?
///         .set_facet_point(2, 1, 3)?
///         .set_facet_point(2, 2, 2)?;
///     tetgen
///         .set_facet_point(3, 0, 1)?
///         .set_facet_point(3, 1, 2)?
///         .set_facet_point(3, 2, 3)?;
///
///     // set region
///     tetgen.set_region(0, 1, 0.1, 0.9, 0.1, None)?;
///
///     // generate mesh
///     let global_max_volume = Some(0.5);
///     tetgen.generate_mesh(false, false, global_max_volume, None)?;
///
///     // draw edges of tetrahedra
///     if SAVE_FIGURE{
///         let mut plot = Plot::new();
///         tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
///         plot.set_equal_axes(true)
///             .set_figure_size_points(600.0, 600.0)
///             .save("/tmp/tritet/doc_tetgen_mesh_1.svg")?;
///     }
///
///     assert_eq!(tetgen.out_ncell(), 7);
///     assert_eq!(tetgen.out_npoint(), 10);
///     Ok(())
/// }
/// ```
///
/// ![doc_tetgen_mesh_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_tetgen_mesh_1.svg)
pub struct Tetgen {
    ext_tetgen: *mut ExtTetgen,       // data allocate by the c-code
    npoint: usize,                    // number of points
    facet_npoint: Option<Vec<usize>>, // number of points on each facet
    total_facet_npoint: usize,        // total number of facet points
    facet_point_set_count: usize,     // counts the number of facet point already set
    nregion: Option<usize>,           // number of regions
    nhole: Option<usize>,             // number of holes
    all_points_set: bool,             // indicates that all points have been set
    all_facets_set: bool,             // indicates that all facets have been set
    all_regions_set: bool,            // indicates that all regions have been set
    all_holes_set: bool,              // indicates that all holes have been set
}

impl Drop for Tetgen {
    /// Tells the c-code to release memory
    fn drop(&mut self) {
        unsafe {
            tet_drop_tetgen(self.ext_tetgen);
        }
    }
}

impl Tetgen {
    /// Allocates a new instance
    pub fn new(
        npoint: usize,
        facet_npoint: Option<Vec<usize>>,
        nregion: Option<usize>,
        nhole: Option<usize>,
    ) -> Result<Self, StrError> {
        if npoint < 4 {
            return Err("npoint must be ≥ 4");
        }
        let npoint_i32: i32 = to_i32(npoint);
        let mut nfacet_i32: i32 = 0;
        let mut total_facet_npoint = 0;
        let mut facet_npoint_i32: Vec<i32> = Vec::new();
        if let Some(facets) = &facet_npoint {
            nfacet_i32 = to_i32(facets.len());
            if nfacet_i32 < 4 {
                return Err("nfacet must be ≥ 4");
            }
            for npoint in facets {
                if *npoint < 3 {
                    return Err("facet npoint must be ≥ 3");
                }
                total_facet_npoint += npoint;
                facet_npoint_i32.push(to_i32(*npoint));
            }
        }
        let nregion_i32: i32 = match nregion {
            Some(v) => to_i32(v),
            None => 0,
        };
        let nhole_i32: i32 = match nhole {
            Some(v) => to_i32(v),
            None => 0,
        };
        unsafe {
            let ext_tetgen = tet_new_tetgen(
                npoint_i32,
                nfacet_i32,
                facet_npoint_i32.as_ptr(),
                nregion_i32,
                nhole_i32,
            );
            if ext_tetgen.is_null() {
                return Err("INTERNAL ERROR: cannot allocate ExtTetgen");
            }
            Ok(Tetgen {
                ext_tetgen,
                npoint,
                facet_npoint,
                total_facet_npoint,
                facet_point_set_count: 0,
                nregion,
                nhole,
                all_points_set: false,
                all_facets_set: false,
                all_regions_set: false,
                all_holes_set: false,
            })
        }
    }

    /// Sets the point coordinates
    ///
    /// **Note:** TetGen automatically assigns the marker 1 for points on the boundary.
    /// Thus, we cannot use marker = 1.
    pub fn set_point(&mut self, index: usize, marker: i32, x: f64, y: f64, z: f64) -> Result<&mut Self, StrError> {
        unsafe {
            let status = tet_set_point(self.ext_tetgen, to_i32(index), marker, x, y, z);
            handle_status(status)?;
        }
        if index == self.npoint - 1 {
            self.all_points_set = true;
        } else {
            self.all_points_set = false;
        }
        Ok(self)
    }

    /// Sets the facet's point IDs
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the facet and goes from 0 to `nfacet` (passed down to `new`)
    /// * `m` -- is the local index of the point on the facet and goes from 0 to `facet_npoint`
    /// * `p` -- is the ID (index) of the point on the facet
    pub fn set_facet_point(&mut self, index: usize, m: usize, p: usize) -> Result<&mut Self, StrError> {
        match &self.facet_npoint {
            Some(n) => n,
            None => return Err("cannot set facet point because facet_npoint is None"),
        };
        unsafe {
            let status = tet_set_facet_point(self.ext_tetgen, to_i32(index), to_i32(m), to_i32(p));
            handle_status(status)?;
        }
        if index == 0 && m == 0 {
            self.facet_point_set_count = 0;
        }
        self.facet_point_set_count += 1;
        if self.facet_point_set_count == self.total_facet_npoint {
            self.all_facets_set = true;
        }
        Ok(self)
    }

    /// Sets the facet's marker (OPTIONAL)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the facet and goes from 0 to `nfacet` (passed down to `new`)
    /// * `marker` -- is the marker
    pub fn set_facet_marker(&mut self, index: usize, marker: i32) -> Result<&mut Self, StrError> {
        match &self.facet_npoint {
            Some(n) => n,
            None => return Err("cannot set facet marker because facet_npoint is None"),
        };
        unsafe {
            let status = tet_set_facet_marker(self.ext_tetgen, to_i32(index), marker);
            handle_status(status)?;
        }
        Ok(self)
    }

    /// Marks a region within the Piecewise Linear Complexes (PLCs)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the region and goes from 0 to `nregion` (passed down to `new`)
    /// * `attribute` -- is the attribute ID to group the tetrahedra belonging to this region
    /// * `x` -- is the x-coordinate of the region
    /// * `y` -- is the y-coordinate of the region
    /// * `z` -- is the z-coordinate of the region
    /// * `max_volume` -- is the maximum volume constraint for the tetrahedra belonging to this region
    pub fn set_region(
        &mut self,
        index: usize,
        attribute: usize,
        x: f64,
        y: f64,
        z: f64,
        max_volume: Option<f64>,
    ) -> Result<&mut Self, StrError> {
        let nregion = match self.nregion {
            Some(n) => n,
            None => return Err("cannot set region because the number of regions is None"),
        };
        let volume_constraint = match max_volume {
            Some(v) => v,
            None => -1.0,
        };
        unsafe {
            let status = tet_set_region(
                self.ext_tetgen,
                to_i32(index),
                to_i32(attribute),
                x,
                y,
                z,
                volume_constraint,
            );
            handle_status(status)?;
        }
        if index == nregion - 1 {
            self.all_regions_set = true;
        } else {
            self.all_regions_set = false;
        }
        Ok(self)
    }

    /// Marks a hole within the Piecewise Linear Complexes (PLCs)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the hole and goes from 0 to `nhole` (passed down to `new`)
    /// * `x` -- is the x-coordinate of the hole
    /// * `y` -- is the y-coordinate of the hole
    /// * `z` -- is the z-coordinate of the hole
    pub fn set_hole(&mut self, index: usize, x: f64, y: f64, z: f64) -> Result<&mut Self, StrError> {
        let nhole = match self.nhole {
            Some(n) => n,
            None => return Err("cannot set hole because the number of holes is None"),
        };
        unsafe {
            let status = tet_set_hole(self.ext_tetgen, to_i32(index), x, y, z);
            handle_status(status)?;
        }
        if index == nhole - 1 {
            self.all_holes_set = true;
        } else {
            self.all_holes_set = false;
        }
        Ok(self)
    }

    /// Generates a Delaunay triangulation
    ///
    /// # Input
    ///
    /// * `verbose` -- Prints Tetgen's messages to the console
    pub fn generate_delaunay(&self, verbose: bool) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("cannot generate Delaunay tetrahedralization because not all points are set");
        }
        unsafe {
            let status = tet_run_delaunay(self.ext_tetgen, if verbose { 1 } else { 0 });
            handle_status(status)?;
        }
        Ok(())
    }

    /// Generates a conforming constrained Delaunay triangulation with some quality constraints
    ///
    /// # Input
    ///
    /// * `verbose` -- Prints Tetgen's messages to the console
    /// * `o2` -- Generates the middle nodes; e.g., nnode = 10
    /// * `global_max_volume` -- The maximum volume constraint for all generated tetrahedra
    /// * `global_min_angle` -- The minimum angle constraint is given in degrees (the default minimum angle is TODO degrees)
    ///
    /// **Note:** TetGen automatically assigns the marker 1 for points on the boundary.
    pub fn generate_mesh(
        &self,
        verbose: bool,
        o2: bool,
        global_max_volume: Option<f64>,
        global_min_angle: Option<f64>,
    ) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("cannot generate mesh of tetrahedra because not all points are set");
        }
        if !self.all_facets_set {
            return Err("cannot generate mesh of tetrahedra because not all facets are set");
        }
        let max_volume = match global_max_volume {
            Some(v) => v,
            None => 0.0,
        };
        let min_angle = match global_min_angle {
            Some(v) => v,
            None => 0.0,
        };
        unsafe {
            let status = tet_run_tetrahedralize(
                self.ext_tetgen,
                if verbose { 1 } else { 0 },
                if o2 { 1 } else { 0 },
                max_volume,
                min_angle,
            );
            handle_status(status)?;
        }
        Ok(())
    }

    /// Returns the number of (output) points of the Delaunay triangulation (constrained or not)
    pub fn out_npoint(&self) -> usize {
        unsafe { tet_out_npoint(self.ext_tetgen) as usize }
    }

    /// Returns the number of (output) tetrahedra (aka cell) on the Delaunay triangulation (constrained or not)
    pub fn out_ncell(&self) -> usize {
        unsafe { tet_out_ncell(self.ext_tetgen) as usize }
    }

    /// Returns the number of points on a (output) tetrahedron (e.g., 4 or 10)
    pub fn out_cell_npoint(&self) -> usize {
        unsafe { tet_out_cell_npoint(self.ext_tetgen) as usize }
    }

    /// Returns the x-y-z coordinates of an output point
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from `0` to `out_npoint`
    /// * `dim` -- is the space dimension index: 0, 1, or 2
    ///
    /// # Warning
    ///
    /// This function will return 0.0 if `index` or `dim` is out of range.
    pub fn out_point(&self, index: usize, dim: usize) -> f64 {
        unsafe { tet_out_point(self.ext_tetgen, to_i32(index), to_i32(dim)) }
    }

    /// Returns the marker of an output point
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from `0` to `out_npoint`
    ///
    /// # Warning
    ///
    /// This function will return zero values if either `index` is out of range.
    pub fn out_point_marker(&self, index: usize) -> i32 {
        unsafe { tet_out_point_marker(self.ext_tetgen, to_i32(index)) }
    }

    /// Returns the ID of a point defining an output cell (aka tetrahedron)
    ///
    /// ```text
    ///       This library (tritet)
    ///               NODES
    ///             |
    ///             3
    ///            /|`.
    ///            ||  `,
    ///           / |    ',
    ///           | |      \
    ///          /  |       `.
    ///          |  |         `,
    ///         /   7            9
    ///         |   |             \
    ///        /    |              `.
    ///        |    |                ',
    ///       8     |                  \
    ///       |     0 ,,_               `.
    ///      |     /     ``'-., 6         `.
    ///      |    /               `''-.,,_  ',
    ///     |    /                        ``'2 ,,
    ///     |   '                       ,.-``
    ///    |   4                   _,-'`
    ///    ' /                 ,.'`
    ///   | /             _ 5 `
    ///   '/          ,-'`
    ///  |/      ,.-``
    ///  /  _,-``
    /// 1 '`
    /// ```
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the tetrahedron and goes from `0` to `out_ncell`
    /// * `m` -- is the local index of the node and goes from `0` to `out_cell_npoint`
    ///
    /// # Warning
    ///
    /// This function will return 0 if `index` or `m` is out of range.
    pub fn out_cell_point(&self, index: usize, m: usize) -> usize {
        unsafe {
            let corner = TRITET_TO_TETGEN[m];
            tet_out_cell_point(self.ext_tetgen, to_i32(index), to_i32(corner)) as usize
        }
    }

    /// Returns the attribute ID of an output cell (aka tetrahedron)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the tetrahedron and goes from `0` to `out_ncell`
    ///
    /// # Warning
    ///
    /// This function will return 0 if `index` is out of range.
    pub fn out_cell_attribute(&self, index: usize) -> usize {
        unsafe { tet_out_cell_attribute(self.ext_tetgen, to_i32(index)) as usize }
    }

    /// Returns the number of marked faces
    pub fn out_n_marked_face(&self) -> usize {
        unsafe { tet_out_n_marked_face(self.ext_tetgen) as usize }
    }

    /// Returns a marked face
    ///
    /// # Input
    ///
    /// * `index` -- is index of a marked face and goes from `0` to `out_n_marked_face`
    ///
    /// # Output
    ///
    /// Returns:
    ///
    /// * `points` -- (len = 6) the global IDs of the points on the face; 3 or 6 (if o2)
    ///   **Note:** The points are ordered as in the figure below; but the face normals
    ///   are **not** inward or outward, i.e., the face normals are random.
    ///
    /// ```text
    ///     NODES
    ///       2
    ///      / \     The middle nodes are
    ///     /   \    only generated if the
    ///    5     4   quadratic flag is true
    ///   /       \
    ///  /         \
    /// 0-----3-----1
    /// ```
    ///
    /// Also returns `(marker, cell)`, where:
    ///
    /// * `marker` -- is the marker associated with the face
    /// * `cell` -- is the global ID of the cell touching this face
    ///
    /// # Warning
    ///
    /// This function will return zero values if `index` is out of range.
    ///
    /// # Examples
    ///
    /// The following code snippet illustrates how to collect all marked faces.
    /// Each face is identified by a key with 3 sorted indices (global point ids).
    ///
    /// ```text
    /// struct Face {
    ///     key: [i32; 3],
    ///     points: [i32; 6],
    ///     marker: i32,
    /// }
    ///
    /// let mut marked_faces: Vec<_> = (0..12)
    ///     .map(|i| {
    ///         let mut face = Face {
    ///             key: [0; 3],
    ///             points: [0; 6],
    ///             marker: 0,
    ///         };
    ///         (face.marker, _) = tetgen.out_marked_face(i, &mut face.points);
    ///         face.key[0] = face.points[0];
    ///         face.key[1] = face.points[1];
    ///         face.key[2] = face.points[2];
    ///         face.key.sort();
    ///         face
    ///     })
    ///     .collect();
    ///
    /// marked_faces.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
    /// ```
    pub fn out_marked_face(&self, index: usize, points: &mut [i32; 6]) -> (i32, usize) {
        let mut marker: i32 = 0;
        let mut cell: i32 = 0;
        unsafe {
            tet_out_marked_face(
                self.ext_tetgen,
                to_i32(index),
                points.as_mut_ptr(),
                &mut marker,
                &mut cell,
            );
        }
        (marker, cell as usize)
    }

    /// Draws wireframe representing the edges of tetrahedra
    pub fn draw_wireframe(
        &self,
        plot: &mut Plot,
        set_range: bool,
        with_point_ids: bool,
        with_triangle_ids: bool,
        with_attribute_ids: bool,
        fontsize_point_ids: Option<f64>,
        fontsize_triangle_ids: Option<f64>,
        fontsize_attribute_ids: Option<f64>,
    ) {
        let ntet = self.out_ncell();
        if ntet < 1 {
            return;
        }
        let mut canvas = Canvas::new();
        let mut point_ids = Text::new();
        let mut tetrahedron_ids = Text::new();
        let mut attribute_ids = Text::new();
        if with_point_ids {
            point_ids
                .set_color("red")
                .set_align_horizontal("center")
                .set_align_vertical("center")
                .set_bbox(true)
                .set_bbox_facecolor("white")
                .set_bbox_alpha(0.8)
                .set_bbox_style("round,pad=0.15");
            if let Some(fsz) = fontsize_point_ids {
                point_ids.set_fontsize(fsz);
            }
        }
        if with_triangle_ids {
            tetrahedron_ids
                .set_color("blue")
                .set_align_horizontal("center")
                .set_align_vertical("center");
            if let Some(fsz) = fontsize_triangle_ids {
                tetrahedron_ids.set_fontsize(fsz);
            }
        }
        if with_attribute_ids {
            attribute_ids
                .set_color("black")
                .set_align_horizontal("center")
                .set_align_vertical("center");
            if let Some(fsz) = fontsize_attribute_ids {
                attribute_ids.set_fontsize(fsz);
            }
        }
        const EDGES: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let mut x = vec![0.0; 3];
        let mut xa = vec![0.0; 3];
        let mut xb = vec![0.0; 3];
        let mut xcen = vec![0.0; 3];
        let mut xatt = vec![0.0; 3];
        let mut min = vec![f64::MAX; 3];
        let mut max = vec![f64::MIN; 3];
        let mut colors: HashMap<usize, &'static str> = HashMap::new();
        let mut index_color = 0;
        let clr = DARK_COLORS;
        for tet in 0..ntet {
            let attribute = self.out_cell_attribute(tet);
            let color = match colors.get(&attribute) {
                Some(c) => c,
                None => {
                    let c = clr[index_color % clr.len()];
                    colors.insert(attribute, c);
                    index_color += 1;
                    c
                }
            };
            canvas.set_edge_color(color);
            for dim in 0..3 {
                xcen[dim] = 0.0;
            }
            for m in 0..4 {
                let p = self.out_cell_point(tet, m);
                for dim in 0..3 {
                    x[dim] = self.out_point(p, dim);
                    min[dim] = f64::min(min[dim], x[dim]);
                    max[dim] = f64::max(max[dim], x[dim]);
                    xcen[dim] += x[dim] / 4.0;
                }
            }
            for (ma, mb) in &EDGES {
                let a = self.out_cell_point(tet, *ma);
                let b = self.out_cell_point(tet, *mb);
                for dim in 0..3 {
                    xa[dim] = self.out_point(a, dim);
                    xb[dim] = self.out_point(b, dim);
                }
                canvas.polyline_3d_begin();
                canvas.polyline_3d_add(xa[0], xa[1], xa[2]);
                canvas.polyline_3d_add(xb[0], xb[1], xb[2]);
                canvas.polyline_3d_end();
            }
            if with_triangle_ids {
                tetrahedron_ids.draw_3d(xcen[0], xcen[1], xcen[2], format!("{}", tet).as_str());
            }
            if with_attribute_ids {
                for dim in 0..3 {
                    x[dim] = self.out_point(self.out_cell_point(tet, 0), dim);
                    xatt[dim] = (x[dim] + xcen[dim]) / 2.0;
                }
                attribute_ids.draw_3d(xatt[0], xatt[1], xatt[2], format!("[{}]", attribute).as_str());
            }
        }
        if with_point_ids {
            for p in 0..self.out_npoint() {
                let x = self.out_point(p, 0);
                let y = self.out_point(p, 1);
                let z = self.out_point(p, 2);
                let m = self.out_point_marker(p);
                let msg = if m != 0 {
                    format!("{}({})", p, m)
                } else {
                    format!("{}", p)
                };
                point_ids.draw_3d(x, y, z, &msg);
            }
        }
        plot.add(&canvas);
        if with_triangle_ids {
            plot.add(&tetrahedron_ids);
        }
        if with_point_ids {
            plot.add(&point_ids);
        }
        if with_attribute_ids {
            plot.add(&attribute_ids);
        }
        if set_range {
            plot.set_range_3d(min[0], max[0], min[1], max[1], min[2], max[2]);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Tetgen;
    use crate::StrError;
    use plotpy::Plot;

    const SAVE_FIGURE: bool = false;

    #[test]
    fn new_captures_some_errors() {
        assert_eq!(Tetgen::new(3, None, None, None).err(), Some("npoint must be ≥ 4"));
        assert_eq!(
            Tetgen::new(4, Some(vec![]), None, None).err(),
            Some("nfacet must be ≥ 4")
        );
        assert_eq!(
            Tetgen::new(4, Some(vec![3, 3, 3, 2]), None, None).err(),
            Some("facet npoint must be ≥ 3")
        );
    }

    #[test]
    fn new_works() -> Result<(), StrError> {
        let tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), None, None)?;
        assert_eq!(tetgen.ext_tetgen.is_null(), false);
        assert_eq!(tetgen.npoint, 4);
        assert_eq!(tetgen.facet_npoint, Some(vec![3, 3, 3, 3]));
        assert_eq!(tetgen.nregion, None);
        assert_eq!(tetgen.nhole, None);
        assert_eq!(tetgen.all_points_set, false);
        assert_eq!(tetgen.all_facets_set, false);
        assert_eq!(tetgen.all_regions_set, false);
        assert_eq!(tetgen.all_holes_set, false);
        Ok(())
    }

    #[test]
    fn set_point_captures_some_errors() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(4, None, None, None)?;
        assert_eq!(
            tetgen.set_point(5, 0, 0.0, 0.0, 0.0).err(),
            Some("index of point is out of bounds")
        );
        Ok(())
    }

    #[test]
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

    #[test]
    fn set_region_captures_some_errors() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(4, None, None, None)?;
        assert_eq!(
            tetgen.set_region(0, 1, 0.33, 0.33, 0.33, Some(0.1)).err(),
            Some("cannot set region because the number of regions is None")
        );
        let mut tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), Some(1), None)?;
        assert_eq!(
            tetgen.set_region(1, 1, 0.33, 0.33, 0.33, Some(0.1)).err(),
            Some("index of region is out of bounds")
        );
        Ok(())
    }

    #[test]
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

    #[test]
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
            .set_point(0, 0, 0.0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0, 0.0)?
            .set_point(3, 0, 0.0, 0.0, 1.0)?;
        assert_eq!(
            tetgen.generate_mesh(false, false, None, None).err(),
            Some("cannot generate mesh of tetrahedra because not all facets are set")
        );
        Ok(())
    }

    #[test]
    fn generate_delaunay_works() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(4, None, None, None)?;
        tetgen
            .set_point(0, 0, 0.0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0, 0.0)?
            .set_point(3, 0, 0.0, 0.0, 1.0)?;
        tetgen.generate_delaunay(false)?;
        assert_eq!(tetgen.out_ncell(), 1);
        assert_eq!(tetgen.out_npoint(), 4);
        Ok(())
    }

    #[test]
    fn draw_wireframe_works() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(4, None, None, None)?;
        tetgen
            .set_point(0, 0, 0.0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0, 0.0)?
            .set_point(3, 0, 0.0, 0.0, 1.0)?;
        tetgen.generate_delaunay(false)?;
        assert_eq!(tetgen.out_ncell(), 1);
        assert_eq!(tetgen.out_npoint(), 4);
        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        if SAVE_FIGURE {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/tetgen_draw_wireframe_works.svg")?;
        }
        Ok(())
    }

    #[test]
    fn generate_delaunay_works_1() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(8, None, None, None)?;
        tetgen
            .set_point(0, -100, 0.0, 0.0, 0.0)?
            .set_point(1, -200, 1.0, 0.0, 0.0)?
            .set_point(2, -300, 1.0, 1.0, 0.0)?
            .set_point(3, -400, 0.0, 1.0, 0.0)?
            .set_point(4, -500, 0.0, 0.0, 1.0)?
            .set_point(5, -600, 1.0, 0.0, 1.0)?
            .set_point(6, -700, 1.0, 1.0, 1.0)?
            .set_point(7, -800, 0.0, 1.0, 1.0)?;
        tetgen.generate_delaunay(false)?;
        assert_eq!(tetgen.out_ncell(), 6);
        assert_eq!(tetgen.out_npoint(), 8);
        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        if SAVE_FIGURE {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/tetgen_test_delaunay_1.svg")?;
        }
        Ok(())
    }

    #[test]
    fn generate_mesh_works_1() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(8, Some(vec![4, 4, 4, 4, 4, 4]), Some(1), None)?;
        tetgen
            .set_point(0, -1, 0.0, 0.0, 0.0)?
            .set_point(1, -2, 1.0, 0.0, 0.0)?
            .set_point(2, -3, 1.0, 1.0, 0.0)?
            .set_point(3, -4, 0.0, 1.0, 0.0)?
            .set_point(4, -5, 0.0, 0.0, 1.0)?
            .set_point(5, -6, 1.0, 0.0, 1.0)?
            .set_point(6, -7, 1.0, 1.0, 1.0)?
            .set_point(7, -8, 0.0, 1.0, 1.0)?;
        tetgen
            .set_facet_point(0, 0, 0)?
            .set_facet_point(0, 1, 4)?
            .set_facet_point(0, 2, 7)?
            .set_facet_point(0, 3, 3)?; // -x
        tetgen
            .set_facet_point(1, 0, 1)?
            .set_facet_point(1, 1, 2)?
            .set_facet_point(1, 2, 6)?
            .set_facet_point(1, 3, 5)?; // +x
        tetgen
            .set_facet_point(2, 0, 0)?
            .set_facet_point(2, 1, 1)?
            .set_facet_point(2, 2, 5)?
            .set_facet_point(2, 3, 4)?; // -y
        tetgen
            .set_facet_point(3, 0, 2)?
            .set_facet_point(3, 1, 3)?
            .set_facet_point(3, 2, 7)?
            .set_facet_point(3, 3, 6)?; // +y
        tetgen
            .set_facet_point(4, 0, 0)?
            .set_facet_point(4, 1, 3)?
            .set_facet_point(4, 2, 2)?
            .set_facet_point(4, 3, 1)?; // -z
        tetgen
            .set_facet_point(5, 0, 4)?
            .set_facet_point(5, 1, 5)?
            .set_facet_point(5, 2, 6)?
            .set_facet_point(5, 3, 7)?; // +z
        tetgen
            .set_facet_marker(0, -10)? // -x
            .set_facet_marker(1, 10)? // +x
            .set_facet_marker(2, -20)? // -y
            .set_facet_marker(3, 20)? // +y
            .set_facet_marker(4, -30)? // -z
            .set_facet_marker(5, 30)?; // +z

        tetgen.set_region(0, 1, 0.5, 0.5, 0.5, None)?;
        tetgen.generate_mesh(false, false, None, None)?;

        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        if SAVE_FIGURE {
            tetgen.write_vtu("/tmp/tritet/tetgen_test_mesh_1.vtu")?;
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/tetgen_test_mesh_1.svg")?;
        }

        assert_eq!(tetgen.out_ncell(), 6);
        assert_eq!(tetgen.out_npoint(), 8);
        assert_eq!(tetgen.out_point_marker(0), -1);
        assert_eq!(tetgen.out_point_marker(1), -2);
        assert_eq!(tetgen.out_point_marker(2), -3);
        assert_eq!(tetgen.out_point_marker(3), -4);
        assert_eq!(tetgen.out_point_marker(4), -5);
        assert_eq!(tetgen.out_point_marker(5), -6);
        assert_eq!(tetgen.out_point_marker(6), -7);
        assert_eq!(tetgen.out_point_marker(7), -8);

        let z4 = [0, 1, 2, 3];

        let pp0: Vec<_> = z4.iter().map(|m| tetgen.out_cell_point(0, *m)).collect();
        let pp1: Vec<_> = z4.iter().map(|m| tetgen.out_cell_point(1, *m)).collect();
        let pp2: Vec<_> = z4.iter().map(|m| tetgen.out_cell_point(2, *m)).collect();
        let pp3: Vec<_> = z4.iter().map(|m| tetgen.out_cell_point(3, *m)).collect();
        let pp4: Vec<_> = z4.iter().map(|m| tetgen.out_cell_point(4, *m)).collect();
        let pp5: Vec<_> = z4.iter().map(|m| tetgen.out_cell_point(5, *m)).collect();
        assert_eq!(pp0, &[0, 3, 7, 2]);
        assert_eq!(pp1, &[0, 7, 4, 6]);
        assert_eq!(pp2, &[5, 0, 4, 6]);
        assert_eq!(pp3, &[0, 7, 6, 2]);
        assert_eq!(pp4, &[5, 0, 6, 1]);
        assert_eq!(pp5, &[6, 0, 2, 1]);

        struct Face {
            key: [i32; 3],
            points: [i32; 6],
            marker: i32,
        }

        let mut marked_faces: Vec<_> = (0..12)
            .map(|i| {
                let mut face = Face {
                    key: [0; 3],
                    points: [0; 6],
                    marker: 0,
                };
                (face.marker, _) = tetgen.out_marked_face(i, &mut face.points);
                face.key[0] = face.points[0];
                face.key[1] = face.points[1];
                face.key[2] = face.points[2];
                face.key.sort();
                face
            })
            .collect();
        marked_faces.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());

        // key, points, marker
        let correct = [
            ([0, 1, 2], [1, 2, 0, 0, 0, 0], -30),
            ([0, 1, 5], [0, 5, 1, 0, 0, 0], -20),
            ([0, 2, 3], [3, 0, 2, 0, 0, 0], -30),
            ([0, 3, 7], [3, 7, 0, 0, 0, 0], -10),
            ([0, 4, 5], [0, 4, 5, 0, 0, 0], -20),
            ([0, 4, 7], [7, 4, 0, 0, 0, 0], -10),
            ([1, 2, 6], [1, 6, 2, 0, 0, 0], 10),
            ([1, 5, 6], [1, 5, 6, 0, 0, 0], 10),
            ([2, 3, 7], [2, 7, 3, 0, 0, 0], 20),
            ([2, 6, 7], [2, 6, 7, 0, 0, 0], 20),
            ([4, 5, 6], [6, 5, 4, 0, 0, 0], 30),
            ([4, 6, 7], [6, 4, 7, 0, 0, 0], 30),
        ];

        for i in 0..12 {
            assert_eq!(marked_faces[i].key, correct[i].0);
            assert_eq!(marked_faces[i].points, correct[i].1);
            assert_eq!(marked_faces[i].marker, correct[i].2);
        }
        Ok(())
    }

    #[test]
    fn generate_mesh_works_2() -> Result<(), StrError> {
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
            .set_point(0, -100, 0.0, 0.0, 0.0)?
            .set_point(1, -200, 1.0, 0.0, 0.0)?
            .set_point(2, -300, 1.0, 1.0, 0.0)?
            .set_point(3, -400, 0.0, 1.0, 0.0)?
            .set_point(4, -500, 0.0, 0.0, 1.0)?
            .set_point(5, -600, 1.0, 0.0, 1.0)?
            .set_point(6, -700, 1.0, 1.0, 1.0)?
            .set_point(7, -800, 0.0, 1.0, 1.0)?;
        // outer cube
        tetgen
            .set_point(8, 0, -1.0, -1.0, -1.0)?
            .set_point(9, 0, 2.0, -1.0, -1.0)?
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
        tetgen.set_region(0, 1, -0.9, -0.9, -0.9, None)?;
        tetgen.set_hole(0, 0.5, 0.5, 0.5)?;
        tetgen.generate_mesh(false, false, None, Some(1.0))?;

        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        if SAVE_FIGURE {
            tetgen.write_vtu("/tmp/tritet/tetgen_test_mesh_2.vtu")?;
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/tetgen_test_mesh_2.svg")?;
        }

        assert_eq!(tetgen.out_ncell(), 84);
        assert_eq!(tetgen.out_npoint(), 34);
        assert_eq!(tetgen.out_point_marker(0), -100);
        assert_eq!(tetgen.out_point_marker(1), -200);
        assert_eq!(tetgen.out_point_marker(2), -300);
        Ok(())
    }

    #[test]
    fn marked_faces_o2_works() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(8, Some(vec![4, 4, 4, 4, 4, 4]), Some(1), None)?;
        tetgen
            .set_point(0, -1, 0.0, 0.0, 0.0)?
            .set_point(1, -2, 1.0, 0.0, 0.0)?
            .set_point(2, -3, 1.0, 1.0, 0.0)?
            .set_point(3, -4, 0.0, 1.0, 0.0)?
            .set_point(4, -5, 0.0, 0.0, 1.0)?
            .set_point(5, -6, 1.0, 0.0, 1.0)?
            .set_point(6, -7, 1.0, 1.0, 1.0)?
            .set_point(7, -8, 0.0, 1.0, 1.0)?;
        tetgen
            .set_facet_point(0, 0, 0)?
            .set_facet_point(0, 1, 4)?
            .set_facet_point(0, 2, 7)?
            .set_facet_point(0, 3, 3)?; // -x
        tetgen
            .set_facet_point(1, 0, 1)?
            .set_facet_point(1, 1, 2)?
            .set_facet_point(1, 2, 6)?
            .set_facet_point(1, 3, 5)?; // +x
        tetgen
            .set_facet_point(2, 0, 0)?
            .set_facet_point(2, 1, 1)?
            .set_facet_point(2, 2, 5)?
            .set_facet_point(2, 3, 4)?; // -y
        tetgen
            .set_facet_point(3, 0, 2)?
            .set_facet_point(3, 1, 3)?
            .set_facet_point(3, 2, 7)?
            .set_facet_point(3, 3, 6)?; // +y
        tetgen
            .set_facet_point(4, 0, 0)?
            .set_facet_point(4, 1, 3)?
            .set_facet_point(4, 2, 2)?
            .set_facet_point(4, 3, 1)?; // -z
        tetgen
            .set_facet_point(5, 0, 4)?
            .set_facet_point(5, 1, 5)?
            .set_facet_point(5, 2, 6)?
            .set_facet_point(5, 3, 7)?; // +z
        tetgen
            .set_facet_marker(0, -10)? // -x
            .set_facet_marker(1, 10)? // +x
            .set_facet_marker(2, -20)? // -y
            .set_facet_marker(3, 20)? // +y
            .set_facet_marker(4, -30)? // -z
            .set_facet_marker(5, 30)?; // +z

        tetgen.set_region(0, 1, 0.5, 0.5, 0.5, None)?;
        tetgen.generate_mesh(false, true, None, None)?;

        let mut plot = Plot::new();
        tetgen.draw_wireframe(&mut plot, true, true, true, true, None, None, None);
        if SAVE_FIGURE {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/tetgen_test_marked_faces_o2.svg")?;
        }

        assert_eq!(tetgen.out_n_marked_face(), 12);

        struct Face {
            key: [i32; 3],
            points: [i32; 6],
            marker: i32,
        }

        let mut marked_faces: Vec<_> = (0..12)
            .map(|i| {
                let mut face = Face {
                    key: [0; 3],
                    points: [0; 6],
                    marker: 0,
                };
                (face.marker, _) = tetgen.out_marked_face(i, &mut face.points);
                face.key[0] = face.points[0];
                face.key[1] = face.points[1];
                face.key[2] = face.points[2];
                face.key.sort();
                face
            })
            .collect();
        marked_faces.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());

        // key, points, marker
        let correct = [
            ([0, 1, 2], [1, 2, 0, 26, 9, 25], -30),
            ([0, 1, 5], [0, 5, 1, 20, 24, 25], -20),
            ([0, 2, 3], [3, 0, 2, 10, 9, 12], -30),
            ([0, 3, 7], [3, 7, 0, 11, 13, 10], -10),
            ([0, 4, 5], [0, 4, 5, 18, 21, 20], -20),
            ([0, 4, 7], [7, 4, 0, 16, 18, 13], -10),
            ([1, 2, 6], [1, 6, 2, 23, 22, 26], 10),
            ([1, 5, 6], [1, 5, 6, 24, 19, 23], 10),
            ([2, 3, 7], [2, 7, 3, 8, 11, 12], 20),
            ([2, 6, 7], [2, 6, 7, 22, 17, 8], 20),
            ([4, 5, 6], [6, 5, 4, 19, 21, 14], 30),
            ([4, 6, 7], [6, 4, 7, 14, 16, 17], 30),
        ];

        for i in 0..12 {
            assert_eq!(marked_faces[i].key, correct[i].0);
            assert_eq!(marked_faces[i].points, correct[i].1);
            assert_eq!(marked_faces[i].marker, correct[i].2);
        }
        Ok(())
    }

    #[test]
    fn handle_coplanar_points() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(4, None, None, None)?;
        tetgen.set_point(0, 0, -1.0, 0.0, 0.0)?; // z=0
        tetgen.set_point(1, 0, 0.0, 0.0, 0.0)?; // z=0
        tetgen.set_point(2, 0, 1.0, 0.0, 0.0)?; // z=0
        tetgen.set_point(3, 0, 0.0, 1.0, 0.0)?; // z=0, thus, all points are coplanar
        assert_eq!(
            tetgen.generate_delaunay(false).err(),
            Some("TetGen failed: points are probably coplanar")
        );
        Ok(())
    }
}
