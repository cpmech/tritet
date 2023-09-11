use crate::constants;
use crate::conversion::to_i32;
use crate::StrError;
use plotpy::{Canvas, Curve, Plot, PolyCode, Text};
use std::collections::HashMap;

#[repr(C)]
pub(crate) struct ExtTrigen {
    data: [u8; 0],
    marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn tri_new_trigen(npoint: i32, nsegment: i32, nregion: i32, nhole: i32) -> *mut ExtTrigen;
    fn tri_drop_trigen(trigen: *mut ExtTrigen);
    fn tri_set_point(trigen: *mut ExtTrigen, index: i32, marker: i32, x: f64, y: f64) -> i32;
    fn tri_set_segment(trigen: *mut ExtTrigen, index: i32, marker: i32, a: i32, b: i32) -> i32;
    fn tri_set_region(trigen: *mut ExtTrigen, index: i32, attribute: i32, x: f64, y: f64, max_area: f64) -> i32;
    fn tri_set_hole(trigen: *mut ExtTrigen, index: i32, x: f64, y: f64) -> i32;
    fn tri_run_delaunay(trigen: *mut ExtTrigen, verbose: i32) -> i32;
    fn tri_run_voronoi(trigen: *mut ExtTrigen, verbose: i32) -> i32;
    fn tri_run_triangulate(
        trigen: *mut ExtTrigen,
        verbose: i32,
        quadratic: i32,
        allow_new_points_on_bry: i32,
        global_max_area: f64,
        global_min_angle: f64,
    ) -> i32;
    fn tri_out_npoint(trigen: *mut ExtTrigen) -> i32;
    fn tri_out_nsegment(trigen: *mut ExtTrigen) -> i32;
    fn tri_out_ncell(trigen: *mut ExtTrigen) -> i32;
    fn tri_out_cell_npoint(trigen: *mut ExtTrigen) -> i32;
    fn tri_out_point(trigen: *mut ExtTrigen, index: i32, dim: i32) -> f64;
    fn tri_out_point_marker(trigen: *mut ExtTrigen, index: i32) -> i32;
    fn tri_out_segment_point(trigen: *mut ExtTrigen, index: i32, side: i32) -> i32;
    fn tri_out_segment_marker(trigen: *mut ExtTrigen, index: i32) -> i32;
    fn tri_out_cell_point(trigen: *mut ExtTrigen, index: i32, corner: i32) -> i32;
    fn tri_out_cell_attribute(trigen: *mut ExtTrigen, index: i32) -> i32;
    fn tri_out_voronoi_npoint(trigen: *mut ExtTrigen) -> i32;
    fn tri_out_voronoi_point(trigen: *mut ExtTrigen, index: i32, dim: i32) -> f64;
    fn tri_out_voronoi_nedge(trigen: *mut ExtTrigen) -> i32;
    fn tri_out_voronoi_edge_point(trigen: *mut ExtTrigen, index: i32, side: i32) -> i32;
    fn tri_out_voronoi_edge_point_b_direction(trigen: *mut ExtTrigen, index: i32, dim: i32) -> f64;
}

/// Holds the index of an endpoint on a Voronoi edge or the direction of the Voronoi edge
#[derive(Clone, Debug)]
pub enum VoronoiEdgePoint {
    /// The index of the endpoint
    Index(usize),

    /// The direction of the infinite ray
    Direction(f64, f64),
}

/// Implements high-level functions to call Shewchuk's Triangle C-Code
///
/// **Note:** All indices are are zero-based.
///
/// # Examples
///
/// ## Delaunay triangulation
///
/// ```
/// use plotpy::Plot;
/// use tritet::{StrError, Trigen};
///
/// fn main() -> Result<(), StrError> {
///     // allocate data for 10 points
///     let mut trigen = Trigen::new(10, None, None, None)?;
///
///     // set points
///     trigen
///         .set_point(0, 0, 0.478554, 0.00869692)?
///         .set_point(1, 0, 0.13928, 0.180603)?
///         .set_point(2, 0, 0.578587, 0.760349)?
///         .set_point(3, 0, 0.903726, 0.975904)?
///         .set_point(4, 0, 0.0980015, 0.981755)?
///         .set_point(5, 0, 0.133721, 0.348832)?
///         .set_point(6, 0, 0.648071, 0.369534)?
///         .set_point(7, 0, 0.230951, 0.558482)?
///         .set_point(8, 0, 0.0307942, 0.459123)?
///         .set_point(9, 0, 0.540745, 0.331184)?;
///
///     // generate Delaunay triangulation
///     trigen.generate_delaunay(false)?;
///
///     // draw triangles
///     let mut plot = Plot::new();
///     // trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
///     // plot.set_equal_axes(true)
///     //     .set_figure_size_points(600.0, 600.0)
///     //     .save("/tmp/tritet/doc_triangle_delaunay_1.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_triangle_delaunay_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_delaunay_1.svg)
///
/// ## Voronoi tessellation
///
/// ```
/// use plotpy::Plot;
/// use tritet::{StrError, Trigen};
///
/// fn main() -> Result<(), StrError> {
///     // allocate data for 10 points
///     let mut trigen = Trigen::new(10, None, None, None)?;
///
///     // set points
///     trigen
///         .set_point(0, 0, 0.478554, 0.00869692)?
///         .set_point(1, 0, 0.13928, 0.180603)?
///         .set_point(2, 0, 0.578587, 0.760349)?
///         .set_point(3, 0, 0.903726, 0.975904)?
///         .set_point(4, 0, 0.0980015, 0.981755)?
///         .set_point(5, 0, 0.133721, 0.348832)?
///         .set_point(6, 0, 0.648071, 0.369534)?
///         .set_point(7, 0, 0.230951, 0.558482)?
///         .set_point(8, 0, 0.0307942, 0.459123)?
///         .set_point(9, 0, 0.540745, 0.331184)?;
///
///     // generate Voronoi tessellation
///     trigen.generate_voronoi(false)?;
///
///     // draw Voronoi diagram
///     let mut plot = Plot::new();
///     // trigen.draw_voronoi(&mut plot);
///     // plot.set_equal_axes(true)
///     //     .set_figure_size_points(600.0, 600.0)
///     //     .save("/tmp/tritet/doc_triangle_voronoi_1.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_triangle_voronoi_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_voronoi_1.svg)
///
/// ## Mesh generation
///
/// ```
/// use plotpy::Plot;
/// use tritet::{StrError, Trigen};
///
/// fn main() -> Result<(), StrError> {
///     // allocate data for 12 points, 10 segments, 2 regions, and 1 hole
///     let mut trigen = Trigen::new(12, Some(10), Some(2), Some(1))?;
///
///     // set points
///     trigen
///         .set_point(0, 0, 0.0, 0.0)?
///         .set_point(1, 0, 1.0, 0.0)?
///         .set_point(2, 0, 1.0, 1.0)?
///         .set_point(3, 0, 0.0, 1.0)?
///         .set_point(4, 0, 0.2, 0.2)?
///         .set_point(5, 0, 0.8, 0.2)?
///         .set_point(6, 0, 0.8, 0.8)?
///         .set_point(7, 0, 0.2, 0.8)?
///         .set_point(8, 0, 0.0, 0.5)?
///         .set_point(9, 0, 0.2, 0.5)?
///         .set_point(10, 0, 0.8, 0.5)?
///         .set_point(11, 0, 1.0, 0.5)?;
///
///     // set segments
///     trigen
///         .set_segment(0, -1, 0, 1)?
///         .set_segment(1, -1, 1, 2)?
///         .set_segment(2, -1, 2, 3)?
///         .set_segment(3, -1, 3, 0)?
///         .set_segment(4, -1, 4, 5)?
///         .set_segment(5, -1, 5, 6)?
///         .set_segment(6, -1, 6, 7)?
///         .set_segment(7, -1, 7, 4)?
///         .set_segment(8, -1, 8, 9)?
///         .set_segment(9, -1, 10, 11)?;
///
///     // set regions
///     trigen
///         .set_region(0, 1, 0.1, 0.1, None)?
///         .set_region(1, 2, 0.1, 0.9, None)?;
///
///     // set holes
///     trigen.set_hole(0, 0.5, 0.5)?;
///
///     // generate o2 mesh without constraints
///     trigen.generate_mesh(false, true, false, None, None)?;
///     assert_eq!(trigen.out_ncell(), 12);
///
///     // draw mesh
///     let mut plot = Plot::new();
///     // trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
///     // plot.set_equal_axes(true)
///     //      .set_figure_size_points(600.0, 600.0)
///     //      .save("/tmp/tritet/doc_triangle_mesh_1.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_triangle_mesh_1.svg](https://raw.githubusercontent.com/cpmech/tritet/main/data/figures/doc_triangle_mesh_1.svg)
///
/// # Definition of geometric terms -- by J.R.Shewchuk
///
/// For convenience, the following definitions are mirrored from [J. R. Shewchuk' Triangle Website](https://www.cs.cmu.edu/~quake/triangle.defs.html#ccdt).
///
/// A Delaunay triangulation of a vertex set is a triangulation of the vertex set with the property that no vertex in the vertex set falls in the interior of the circumcircle (circle that passes through all three vertices) of any triangle in the triangulation.
///
/// A Voronoi diagram of a vertex set is a subdivision of the plane into polygonal regions (some of which may be infinite), where each region is the set of points in the plane that are closer to some input vertex than to any other input vertex. (The Voronoi diagram is the geometric dual of the Delaunay triangulation.)
///
/// A Planar Straight Line Graph (PSLG) is a collection of vertices and segments. Segments are edges whose endpoints are vertices in the PSLG, and whose presence in any mesh generated from the PSLG is enforced.
///
/// A constrained Delaunay triangulation of a PSLG is similar to a Delaunay triangulation, but each PSLG segment is present as a single edge in the triangulation. A constrained Delaunay triangulation is not truly a Delaunay triangulation. Some of its triangles might not be Delaunay, but they are all constrained Delaunay.
///
/// A conforming Delaunay triangulation (CDT) of a PSLG is a true Delaunay triangulation in which each PSLG segment may have been subdivided into several edges by the insertion of additional vertices, called Steiner points. Steiner points are necessary to allow the segments to exist in the mesh while maintaining the Delaunay property. Steiner points are also inserted to meet constraints on the minimum angle and maximum triangle area.
///
/// A constrained conforming Delaunay triangulation (CCDT) of a PSLG is a constrained Delaunay triangulation that includes Steiner points. It usually takes fewer vertices to make a good-quality CCDT than a good-quality CDT, because the triangles do not need to be Delaunay (although they still must be constrained Delaunay).
///
/// # Boundary markers -- by J.R.Shewchuk
///
/// Boundary markers are tags used mainly to identify which output vertices and edges are associated with which PSLG segment, and to identify which vertices and edges occur on a boundary of the triangulation. A common use is to determine where boundary conditions should be applied to a finite element mesh. You can prevent boundary markers from being written into files produced by Triangle by using the -B switch.
///
/// The boundary marker associated with each segment in an output .poly file and each edge in an output .edge file is chosen as follows:
///
/// * If an output edge is part or all of a PSLG segment with a nonzero boundary marker, then the edge is assigned the same marker as the segment.
/// * Otherwise, if the edge occurs on a boundary of the triangulation (including boundaries of holes), then the edge is assigned the marker one (1).
/// * Otherwise, the edge is assigned the marker zero (0).
///
/// The boundary marker associated with each vertex in an output .node file is chosen as follows:
///
/// * If a vertex is assigned a nonzero boundary marker in the input file, then it is assigned the same marker in the output .node file.
/// * Otherwise, if the vertex lies on a PSLG segment (including the segment's endpoints) with a nonzero boundary marker, then the vertex is assigned the same marker. If the vertex lies on several such segments, one of the markers is chosen arbitrarily.
/// * Otherwise, if the vertex occurs on a boundary of the triangulation, then the vertex is assigned the marker one (1).
/// * Otherwise, the vertex is assigned the marker zero (0).
///
/// If you want Triangle to determine for you which vertices and edges are on the boundary, assign them the boundary marker zero (or use no markers at all) in your input files. In the output files, all boundary vertices, edges, and segments will be assigned the value one.
///
/// # References
///
/// See also [J. R. Shewchuk' Triangle Website](https://www.cs.cmu.edu/~quake/triangle.html).
///
/// * **Jonathan Richard Shewchuk**, Triangle: Engineering a 2D Quality Mesh Generator and Delaunay Triangulator, in Applied Computational Geometry: Towards Geometric Engineering (Ming C. Lin and Dinesh Manocha, editors), volume 1148 of Lecture Notes in Computer Science, pages 203-222, Springer-Verlag, Berlin, May 1996.
/// * **Jonathan Richard Shewchuk**, Delaunay Refinement Algorithms for Triangular Mesh Generation, Computational Geometry: Theory and Applications 22(1-3):21-74, May 2002.
pub struct Trigen {
    ext_trigen: *mut ExtTrigen, // data allocated by the c-code
    npoint: usize,              // number of input points in the PSLG
    nsegment: Option<usize>,    // number of input segments in the PSLG
    nregion: Option<usize>,     // number of input regions in the PSLG
    nhole: Option<usize>,       // number of input holes in the PSLG
    all_points_set: bool,       // indicates that all points have been set
    all_segments_set: bool,     // indicates that all segments have been set
    all_regions_set: bool,      // indicates that all regions have been set
    all_holes_set: bool,        // indicates that all holes have been set
}

impl Drop for Trigen {
    /// Tells the c-code to release memory
    fn drop(&mut self) {
        unsafe {
            tri_drop_trigen(self.ext_trigen);
        }
    }
}

impl Trigen {
    /// Allocates a new instance
    ///
    /// # Input
    ///
    /// * `npoint` -- is the number of points in the input PSLG
    /// * `nsegment` -- (only for [Trigen::generate_mesh]) is the number of segments in the input PSLG
    /// * `nregion` -- (only for [Trigen::generate_mesh]) is the number of regions in the input PSLG
    /// * `nhole` -- (only for [Trigen::generate_mesh]) is the number of holes in the input PSLG
    pub fn new(
        npoint: usize,
        nsegment: Option<usize>,
        nregion: Option<usize>,
        nhole: Option<usize>,
    ) -> Result<Self, StrError> {
        if npoint < 3 {
            return Err("npoint must be ≥ 3");
        }
        if let Some(ns) = nsegment {
            if ns < 3 {
                return Err("nsegment must be ≥ 3");
            }
        }
        let npoint_i32: i32 = to_i32(npoint);
        let nsegment_i32: i32 = match nsegment {
            Some(v) => to_i32(v),
            None => 0,
        };
        let nregion_i32: i32 = match nregion {
            Some(v) => to_i32(v),
            None => 0,
        };
        let nhole_i32: i32 = match nhole {
            Some(v) => to_i32(v),
            None => 0,
        };
        unsafe {
            let ext_triangle = tri_new_trigen(npoint_i32, nsegment_i32, nregion_i32, nhole_i32);
            if ext_triangle.is_null() {
                return Err("INTERNAL ERROR: cannot allocate ExtTriangle");
            }
            Ok(Trigen {
                ext_trigen: ext_triangle,
                npoint,
                nsegment,
                nregion,
                nhole,
                all_points_set: false,
                all_segments_set: false,
                all_regions_set: false,
                all_holes_set: false,
            })
        }
    }

    /// Sets the point coordinates
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from `0` to `npoint` (specified in [Trigen::new])
    /// * `marker` -- is a marker for the point
    /// * `x` -- is x-coordinate of the point
    /// * `y` -- is y-coordinate of the point
    ///
    /// # Note about boundary markers -- by J.R.Shewchuk
    ///
    /// The boundary marker associated with each vertex in the output is chosen as follows:
    ///
    /// * If a vertex is assigned a nonzero boundary marker in the input, then it is assigned the same marker in the output.
    /// * Otherwise, if the vertex lies on a PSLG segment (including the segment's endpoints) with a nonzero boundary marker, then the vertex is assigned the same marker. If the vertex lies on several such segments, one of the markers is chosen arbitrarily.
    /// * Otherwise, if the vertex occurs on a boundary of the triangulation, then the vertex is assigned the marker one (1).
    /// * Otherwise, the vertex is assigned the marker zero (0).
    pub fn set_point(&mut self, index: usize, marker: i32, x: f64, y: f64) -> Result<&mut Self, StrError> {
        unsafe {
            let status = tri_set_point(self.ext_trigen, to_i32(index), marker, x, y);
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: found NULL point list");
                }
                if status == constants::TRITET_ERROR_INVALID_POINT_INDEX {
                    return Err("index of point is out of bounds");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        if index == self.npoint - 1 {
            self.all_points_set = true;
        } else {
            self.all_points_set = false;
        }
        Ok(self)
    }

    /// Sets the segment endpoint IDs
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the segment and goes from 0 to `nsegment` (passed down to `new`)
    /// * `marker` -- a marker to identify the segment (e.g., a boundary segment)
    /// * `a` -- is the ID (index) of the first point on the segment
    /// * `b` -- is the ID (index) of the second point on the segment
    ///
    /// # Note about boundary markers -- by J.R.Shewchuk
    ///
    /// The boundary marker associated with each segment in the output is chosen as follows:
    ///
    /// * If an output edge is part or all of a PSLG segment with a nonzero boundary marker, then the edge is assigned the same marker as the segment.
    /// * Otherwise, if the edge occurs on a boundary of the triangulation (including boundaries of holes), then the edge is assigned the marker one (1).
    /// * Otherwise, the edge is assigned the marker zero (0).
    pub fn set_segment(&mut self, index: usize, marker: i32, a: usize, b: usize) -> Result<&mut Self, StrError> {
        let nsegment = match self.nsegment {
            Some(n) => n,
            None => return Err("cannot set segment because the number of segments is None"),
        };
        unsafe {
            let status = tri_set_segment(self.ext_trigen, to_i32(index), marker, to_i32(a), to_i32(b));
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_SEGMENT_LIST {
                    return Err("INTERNAL ERROR: found NULL segment list");
                }
                if status == constants::TRITET_ERROR_INVALID_SEGMENT_INDEX {
                    return Err("index of segment is out of bounds");
                }
                if status == constants::TRITET_ERROR_INVALID_SEGMENT_POINT_ID {
                    return Err("id of segment point is out of bounds");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        if index == nsegment - 1 {
            self.all_segments_set = true;
        } else {
            self.all_segments_set = false;
        }
        Ok(self)
    }

    /// Marks a region within the Planar Straight Line Graph (PSLG)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the region and goes from 0 to `nregion` (passed down to `new`)
    /// * `attribute` -- is the attribute ID to group the triangles belonging to this region
    /// * `x` -- is the x-coordinate of the region
    /// * `y` -- is the y-coordinate of the region
    /// * `max_area` -- is the maximum area constraint for the triangles belonging to this region
    pub fn set_region(
        &mut self,
        index: usize,
        attribute: usize,
        x: f64,
        y: f64,
        max_area: Option<f64>,
    ) -> Result<&mut Self, StrError> {
        let nregion = match self.nregion {
            Some(n) => n,
            None => return Err("cannot set region because the number of regions is None"),
        };
        let area_constraint = match max_area {
            Some(v) => v,
            None => -1.0,
        };
        unsafe {
            let status = tri_set_region(self.ext_trigen, to_i32(index), to_i32(attribute), x, y, area_constraint);
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_REGION_LIST {
                    return Err("INTERNAL ERROR: found NULL region list");
                }
                if status == constants::TRITET_ERROR_INVALID_REGION_INDEX {
                    return Err("index of region is out of bounds");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        if index == nregion - 1 {
            self.all_regions_set = true;
        } else {
            self.all_regions_set = false;
        }
        Ok(self)
    }

    /// Marks a hole within the Planar Straight Line Graph (PSLG)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the hole and goes from 0 to `nhole` (passed down to `new`)
    /// * `x` -- is the x-coordinate of the hole
    /// * `y` -- is the y-coordinate of the hole
    pub fn set_hole(&mut self, index: usize, x: f64, y: f64) -> Result<&mut Self, StrError> {
        let nhole = match self.nhole {
            Some(n) => n,
            None => return Err("cannot set hole because the number of holes is None"),
        };
        unsafe {
            let status = tri_set_hole(self.ext_trigen, to_i32(index), x, y);
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_HOLE_LIST {
                    return Err("INTERNAL ERROR: found NULL hole list");
                }
                if status == constants::TRITET_ERROR_INVALID_HOLE_INDEX {
                    return Err("index of hole is out of bounds");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
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
    /// * `verbose` -- Prints Triangle's messages to the console
    pub fn generate_delaunay(&self, verbose: bool) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("cannot generate Delaunay triangulation because not all points are set");
        }
        unsafe {
            let status = tri_run_delaunay(self.ext_trigen, if verbose { 1 } else { 0 });
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: found NULL point list");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        Ok(())
    }

    /// Generates a Voronoi tessellation and Delaunay triangulation
    ///
    /// # Input
    ///
    /// * `verbose` -- Prints Triangle's messages to the console
    pub fn generate_voronoi(&self, verbose: bool) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("cannot generate Voronoi tessellation because not all points are set");
        }
        unsafe {
            let status = tri_run_voronoi(self.ext_trigen, if verbose { 1 } else { 0 });
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: found NULL point list");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        Ok(())
    }

    /// Generates a conforming constrained Delaunay triangulation with some quality constraints
    ///
    /// # Input
    ///
    /// * `verbose` -- Prints Triangle's messages to the console
    /// * `quadratic` -- Generates the middle nodes; e.g., nnode = 6
    /// * `allow_new_points_on_bry:bool` -- Allow the insertion of new (Steiner) points on the boundary
    /// * `global_max_area` -- The maximum area constraint for all generated triangles
    /// * `global_min_angle` -- The minimum angle constraint is given in degrees (the default minimum angle is twenty degrees)
    pub fn generate_mesh(
        &self,
        verbose: bool,
        quadratic: bool,
        allow_new_points_on_bry: bool,
        global_max_area: Option<f64>,
        global_min_angle: Option<f64>,
    ) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("cannot generate mesh of triangles because not all points are set");
        }
        if !self.all_segments_set {
            return Err("cannot generate mesh of triangles because not all segments are set");
        }
        let max_area = match global_max_area {
            Some(v) => v,
            None => 0.0,
        };
        let min_angle = match global_min_angle {
            Some(v) => v,
            None => 0.0,
        };
        unsafe {
            let status = tri_run_triangulate(
                self.ext_trigen,
                if verbose { 1 } else { 0 },
                if quadratic { 1 } else { 0 },
                if allow_new_points_on_bry { 1 } else { 0 },
                max_area,
                min_angle,
            );
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: found NULL point list");
                }
                if status == constants::TRITET_ERROR_NULL_SEGMENT_LIST {
                    return Err("INTERNAL ERROR: list of segments must be defined first");
                }
                if status == constants::TRITET_ERROR_STRING_CONCAT {
                    return Err("INTERNAL ERROR: cannot write string with commands for Triangle");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        Ok(())
    }

    /// Returns the number of (output) points of the Delaunay triangulation (constrained or not)
    pub fn out_npoint(&self) -> usize {
        unsafe { tri_out_npoint(self.ext_trigen) as usize }
    }

    /// Returns the number of (output) segments generated on the PSLG (not the interior)
    ///
    /// **Note:** This option is only available when calling [Trigen::generate_mesh]
    pub fn out_nsegment(&self) -> usize {
        unsafe { tri_out_nsegment(self.ext_trigen) as usize }
    }

    /// Returns the number of (output) triangles (aka cells) on the Delaunay triangulation (constrained or not)
    pub fn out_ncell(&self) -> usize {
        unsafe { tri_out_ncell(self.ext_trigen) as usize }
    }

    /// Returns the number of nodes on a triangle (e.g., 3 or 6)
    pub fn out_cell_npoint(&self) -> usize {
        unsafe { tri_out_cell_npoint(self.ext_trigen) as usize }
    }

    /// Returns the (output) generated point
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from `0` to `out_npoint`
    /// * `dim` -- is the space dimension index: 0 or 1
    ///
    /// # Output
    ///
    /// Returns `x` or `z`
    ///
    /// # Warning
    ///
    /// This function will return zero values if either `index` is out of range.
    pub fn out_point(&self, index: usize, dim: usize) -> f64 {
        unsafe { tri_out_point(self.ext_trigen, to_i32(index), to_i32(dim)) }
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
    ///
    /// # Note about boundary markers -- by J.R.Shewchuk
    ///
    /// The boundary marker associated with each vertex in the output is chosen as follows:
    ///
    /// * If a vertex is assigned a nonzero boundary marker in the input, then it is assigned the same marker in the output.
    /// * Otherwise, if the vertex lies on a PSLG segment (including the segment's endpoints) with a nonzero boundary marker, then the vertex is assigned the same marker. If the vertex lies on several such segments, one of the markers is chosen arbitrarily.
    /// * Otherwise, if the vertex occurs on a boundary of the triangulation, then the vertex is assigned the marker one (1).
    /// * Otherwise, the vertex is assigned the marker zero (0).
    pub fn out_point_marker(&self, index: usize) -> i32 {
        unsafe { tri_out_point_marker(self.ext_trigen, to_i32(index)) }
    }

    /// Returns the ID of a point of a segment generated on the PSLG
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the PSLG segment and goes from `0` to `out_nsegment`
    /// * `side` -- `0` or `1`; corresponds to the "side" of the segment
    ///
    /// # Warning
    ///
    /// This function will return zero values if the `index` or `side` is out of range.
    pub fn out_segment_point(&self, index: usize, side: usize) -> usize {
        unsafe { tri_out_segment_point(self.ext_trigen, to_i32(index), to_i32(side)) as usize }
    }

    /// Returns the marker attached to the output segment
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the PSLG segment and goes from `0` to `out_nsegment`
    ///
    /// # Warning
    ///
    /// This function will return zero values if the `index` is out of range.
    ///
    /// # Note about boundary markers -- by J.R.Shewchuk
    ///
    /// The boundary marker associated with each segment in the output is chosen as follows:
    ///
    /// * If an output edge is part or all of a PSLG segment with a nonzero boundary marker, then the edge is assigned the same marker as the segment.
    /// * Otherwise, if the edge occurs on a boundary of the triangulation (including boundaries of holes), then the edge is assigned the marker one (1).
    /// * Otherwise, the edge is assigned the marker zero (0).
    pub fn out_segment_marker(&self, index: usize) -> i32 {
        unsafe { tri_out_segment_marker(self.ext_trigen, to_i32(index)) }
    }

    /// Returns the ID of a point on the triangle (aka cell)
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
    /// # Input
    ///
    /// * `index` -- is the index of the triangle and goes from 0 to `out_ncell`
    /// * `m` -- is the local index of the node and goes from 0 to `out_cell_npoint`
    ///
    /// # Warning
    ///
    /// This function will return 0 if `index` or `m` is out of range.
    pub fn out_cell_point(&self, index: usize, m: usize) -> usize {
        unsafe {
            let corner = constants::TRITET_TO_TRIANGLE[m];
            tri_out_cell_point(self.ext_trigen, to_i32(index), to_i32(corner)) as usize
        }
    }

    /// Returns the attribute ID of a triangle (aka cell)
    ///
    /// # Warning
    ///
    /// This function will return 0 if the `index` is out of range.
    pub fn out_cell_attribute(&self, index: usize) -> usize {
        unsafe { tri_out_cell_attribute(self.ext_trigen, to_i32(index)) as usize }
    }

    /// Returns the number of points of the Voronoi tessellation
    pub fn out_voronoi_npoint(&self) -> usize {
        unsafe { tri_out_voronoi_npoint(self.ext_trigen) as usize }
    }

    /// Returns the x-y coordinates of a point on the Voronoi tessellation
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from 0 to `out_voronoi_npoint`
    /// * `dim` -- is the space dimension index: 0 or 1
    ///
    /// # Warning
    ///
    /// This function will return 0.0 if `index` or `dim` is out of range.
    pub fn out_voronoi_point(&self, index: usize, dim: usize) -> f64 {
        unsafe { tri_out_voronoi_point(self.ext_trigen, to_i32(index), to_i32(dim)) }
    }

    /// Returns the number of edges on the Voronoi tessellation
    pub fn out_voronoi_nedge(&self) -> usize {
        unsafe { tri_out_voronoi_nedge(self.ext_trigen) as usize }
    }

    /// Returns the index of the first endpoint on a Voronoi edge
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the edge and goes from 0 to `out_voronoi_nedge`
    ///
    /// # Warning
    ///
    /// This function will return 0 if `index` is out of range.
    pub fn out_voronoi_edge_point_a(&self, index: usize) -> usize {
        unsafe { tri_out_voronoi_edge_point(self.ext_trigen, to_i32(index), 0) as usize }
    }

    /// Returns the index of the second endpoint on a Voronoi edge or the direction of the Voronoi edge
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the edge and goes from 0 to `out_voronoi_nedge`
    ///
    /// # Warning
    ///
    /// This function will return Index(0) if `index` is out of range.
    pub fn out_voronoi_edge_point_b(&self, index: usize) -> VoronoiEdgePoint {
        unsafe {
            let index_i32 = to_i32(index);
            let id = tri_out_voronoi_edge_point(self.ext_trigen, index_i32, 1);
            if id == -1 {
                let x = tri_out_voronoi_edge_point_b_direction(self.ext_trigen, index_i32, 0);
                let y = tri_out_voronoi_edge_point_b_direction(self.ext_trigen, index_i32, 1);
                VoronoiEdgePoint::Direction(x, y)
            } else {
                VoronoiEdgePoint::Index(id as usize)
            }
        }
    }

    /// Draw triangles
    pub fn draw_triangles(
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
        let n_triangle = self.out_ncell();
        if n_triangle < 1 {
            return;
        }
        let mut canvas = Canvas::new();
        let mut point_ids = Text::new();
        let mut triangle_ids = Text::new();
        let mut attribute_ids = Text::new();
        if with_point_ids {
            point_ids
                .set_color("red")
                .set_align_horizontal("center")
                .set_align_vertical("center")
                .set_bbox(true)
                .set_bbox_facecolor("white")
                .set_bbox_alpha(0.8)
                .set_bbox_style("circle");
            if let Some(fsz) = fontsize_point_ids {
                point_ids.set_fontsize(fsz);
            }
        }
        if with_triangle_ids {
            triangle_ids
                .set_color("blue")
                .set_align_horizontal("center")
                .set_align_vertical("center");
            if let Some(fsz) = fontsize_triangle_ids {
                triangle_ids.set_fontsize(fsz);
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
        canvas.set_edge_color("black");
        let mut x = vec![0.0; 2];
        let mut xmid = vec![0.0; 2];
        let mut xatt = vec![0.0; 2];
        let mut min = vec![f64::MAX; 2];
        let mut max = vec![f64::MIN; 2];
        let mut colors: HashMap<usize, &'static str> = HashMap::new();
        let mut index_color = 0;
        let clr = constants::LIGHT_COLORS;
        for tri in 0..n_triangle {
            let attribute = self.out_cell_attribute(tri);
            let color = match colors.get(&attribute) {
                Some(c) => c,
                None => {
                    let c = clr[index_color % clr.len()];
                    colors.insert(attribute, c);
                    index_color += 1;
                    c
                }
            };
            canvas.set_face_color(color);
            canvas.polycurve_begin();
            for dim in 0..2 {
                xmid[dim] = 0.0;
            }
            for m in 0..3 {
                let p = self.out_cell_point(tri, m);
                for dim in 0..2 {
                    x[dim] = self.out_point(p, dim);
                    min[dim] = f64::min(min[dim], x[dim]);
                    max[dim] = f64::max(max[dim], x[dim]);
                    xmid[dim] += x[dim] / 3.0;
                }
                if m == 0 {
                    canvas.polycurve_add(x[0], x[1], PolyCode::MoveTo);
                } else {
                    canvas.polycurve_add(x[0], x[1], PolyCode::LineTo);
                }
            }
            canvas.polycurve_end(true);
            if with_triangle_ids {
                triangle_ids.draw(xmid[0], xmid[1], format!("{}", tri).as_str());
            }
            if with_attribute_ids {
                let p = self.out_cell_point(tri, 0);
                for dim in 0..2 {
                    x[dim] = self.out_point(p, dim);
                    xatt[dim] = (x[dim] + xmid[dim]) / 2.0;
                }
                attribute_ids.draw(xatt[0], xatt[1], format!("[{}]", attribute).as_str());
            }
        }
        if with_point_ids {
            for p in 0..self.out_npoint() {
                let x_val = self.out_point(p, 0);
                let y_val = self.out_point(p, 1);
                point_ids.draw(x_val, y_val, format!("{}", p).as_str());
            }
        }
        plot.add(&canvas);
        if with_triangle_ids {
            plot.add(&triangle_ids);
        }
        if with_point_ids {
            plot.add(&point_ids);
        }
        if with_attribute_ids {
            plot.add(&attribute_ids);
        }
        if set_range {
            plot.set_range(min[0], max[0], min[1], max[1]);
        }
    }

    /// Draws Voronoi diagram
    pub fn draw_voronoi(&self, plot: &mut Plot) {
        if self.out_voronoi_npoint() < 1 || self.out_voronoi_nedge() < 1 {
            return;
        }
        let mut x = vec![0.0; 2];
        let mut min = vec![f64::MAX; 2];
        let mut max = vec![f64::MIN; 2];
        let mut markers = Curve::new();
        markers
            .set_marker_color("gold")
            .set_marker_line_color("gold")
            .set_marker_style("o")
            .set_stop_clip(true);
        for p in 0..self.out_npoint() {
            for dim in 0..2 {
                x[dim] = self.out_point(p, dim);
                min[dim] = f64::min(min[dim], x[dim]);
                max[dim] = f64::max(max[dim], x[dim]);
            }
            markers.draw(&[x[0]], &[x[1]]);
        }
        for q in 0..self.out_voronoi_npoint() {
            for dim in 0..2 {
                x[dim] = self.out_voronoi_point(q, dim);
                min[dim] = f64::min(min[dim], x[dim]);
                max[dim] = f64::max(max[dim], x[dim]);
            }
        }
        let mut canvas = Canvas::new();
        canvas.polycurve_begin();
        for e in 0..self.out_voronoi_nedge() {
            let a = self.out_voronoi_edge_point_a(e);
            let xa = self.out_voronoi_point(a, 0);
            let ya = self.out_voronoi_point(a, 1);
            let b_or_direction = self.out_voronoi_edge_point_b(e);
            match b_or_direction {
                VoronoiEdgePoint::Index(b) => {
                    let xb = self.out_voronoi_point(b, 0);
                    let yb = self.out_voronoi_point(b, 1);
                    canvas.polycurve_add(xa, ya, PolyCode::MoveTo);
                    canvas.polycurve_add(xb, yb, PolyCode::LineTo);
                }
                VoronoiEdgePoint::Direction(dx, dy) => {
                    let mx = if dx > 0.0 {
                        (max[0] - xa) / dx
                    } else if dx < 0.0 {
                        (min[0] - xa) / dx
                    } else {
                        0.0
                    };
                    let my = if dy > 0.0 {
                        (max[1] - ya) / dy
                    } else if dy < 0.0 {
                        (min[1] - ya) / dy
                    } else {
                        0.0
                    };
                    let m = if mx < my { mx } else { my };
                    if m > 0.0 {
                        let xb = xa + m * dx;
                        let yb = ya + m * dy;
                        min[0] = f64::min(min[0], xb);
                        max[0] = f64::max(max[0], xb);
                        min[1] = f64::min(min[1], yb);
                        max[1] = f64::max(max[1], yb);
                        canvas.polycurve_add(xa, ya, PolyCode::MoveTo);
                        canvas.polycurve_add(xb, yb, PolyCode::LineTo);
                    }
                }
            }
        }
        canvas.polycurve_end(false);
        plot.set_range(min[0], max[0], min[1], max[1]);
        plot.add(&canvas).add(&markers);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Trigen;
    use crate::{StrError, VoronoiEdgePoint};
    use plotpy::Plot;

    const GENERATE_FIGURES: bool = false;

    #[test]
    fn derive_works() {
        let option = VoronoiEdgePoint::Index(0);
        let cloned = option.clone();
        assert_eq!(format!("{:?}", option), "Index(0)");
        assert_eq!(format!("{:?}", cloned), "Index(0)");
    }

    #[test]
    fn new_captures_some_errors() {
        assert_eq!(Trigen::new(2, None, None, None).err(), Some("npoint must be ≥ 3"));
        assert_eq!(Trigen::new(3, Some(2), None, None).err(), Some("nsegment must be ≥ 3"));
    }

    #[test]
    fn new_works() -> Result<(), StrError> {
        let trigen = Trigen::new(3, Some(3), None, None)?;
        assert_eq!(trigen.ext_trigen.is_null(), false);
        assert_eq!(trigen.npoint, 3);
        assert_eq!(trigen.nsegment, Some(3));
        assert_eq!(trigen.nregion, None);
        assert_eq!(trigen.nhole, None);
        assert_eq!(trigen.all_points_set, false);
        assert_eq!(trigen.all_segments_set, false);
        assert_eq!(trigen.all_regions_set, false);
        assert_eq!(trigen.all_holes_set, false);
        Ok(())
    }

    #[test]
    fn set_point_captures_some_errors() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, None, None, None)?;
        assert_eq!(
            trigen.set_point(4, 0, 0.0, 0.0).err(),
            Some("index of point is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn set_segment_captures_some_errors() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, None, None, None)?;
        assert_eq!(
            trigen.set_segment(0, -10, 0, 1).err(),
            Some("cannot set segment because the number of segments is None")
        );
        let mut trigen = Trigen::new(3, Some(3), None, None)?;
        assert_eq!(
            trigen.set_segment(4, -10, 0, 1).err(),
            Some("index of segment is out of bounds")
        );
        assert_eq!(
            trigen.set_segment(0, -10, 0, 4).err(),
            Some("id of segment point is out of bounds")
        );
        Ok(())
    }

    #[test]
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

    #[test]
    fn set_hole_captures_some_errors() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, None, None, None)?;
        assert_eq!(
            trigen.set_hole(0, 0.33, 0.33).err(),
            Some("cannot set hole because the number of holes is None")
        );
        let mut trigen = Trigen::new(3, Some(3), Some(1), Some(1))?;
        assert_eq!(
            trigen.set_hole(1, 0.33, 0.33).err(),
            Some("index of hole is out of bounds")
        );
        Ok(())
    }

    #[test]
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
            trigen.generate_mesh(false, false, false, None, None).err(),
            Some("cannot generate mesh of triangles because not all points are set")
        );
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0)?;
        assert_eq!(
            trigen.generate_mesh(false, false, false, None, None).err(),
            Some("cannot generate mesh of triangles because not all segments are set")
        );
        Ok(())
    }

    #[test]
    fn delaunay_1_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, None, None, None)?;
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0)?;
        trigen.generate_delaunay(false)?;
        assert_eq!(trigen.out_npoint(), 3);
        assert_eq!(trigen.out_ncell(), 1);
        assert_eq!(trigen.out_cell_npoint(), 3);
        assert_eq!(trigen.out_point(0, 0), 0.0);
        assert_eq!(trigen.out_point(0, 1), 0.0);
        assert_eq!(trigen.out_point(1, 0), 1.0);
        assert_eq!(trigen.out_point(1, 1), 0.0);
        assert_eq!(trigen.out_point(2, 0), 0.0);
        assert_eq!(trigen.out_point(2, 1), 1.0);
        assert_eq!(trigen.out_cell_point(0, 0), 0);
        assert_eq!(trigen.out_cell_point(0, 1), 1);
        assert_eq!(trigen.out_cell_point(0, 2), 2);
        assert_eq!(trigen.out_voronoi_npoint(), 0);
        assert_eq!(trigen.out_voronoi_nedge(), 0);
        Ok(())
    }

    #[test]
    fn voronoi_1_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, None, None, None)?;
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0)?;
        trigen.generate_voronoi(false)?;
        assert_eq!(trigen.out_npoint(), 3);
        assert_eq!(trigen.out_ncell(), 1);
        assert_eq!(trigen.out_cell_npoint(), 3);
        assert_eq!(trigen.out_point(0, 0), 0.0);
        assert_eq!(trigen.out_point(0, 1), 0.0);
        assert_eq!(trigen.out_point(1, 0), 1.0);
        assert_eq!(trigen.out_point(1, 1), 0.0);
        assert_eq!(trigen.out_point(2, 0), 0.0);
        assert_eq!(trigen.out_point(2, 1), 1.0);
        assert_eq!(trigen.out_voronoi_npoint(), 1);
        assert_eq!(trigen.out_voronoi_point(0, 0), 0.5);
        assert_eq!(trigen.out_voronoi_point(0, 1), 0.5);
        assert_eq!(trigen.out_voronoi_nedge(), 3);
        assert_eq!(trigen.out_voronoi_edge_point_a(0), 0);
        assert_eq!(
            format!("{:?}", trigen.out_voronoi_edge_point_b(0)),
            "Direction(0.0, -1.0)"
        );
        assert_eq!(trigen.out_voronoi_edge_point_a(1), 0);
        assert_eq!(
            format!("{:?}", trigen.out_voronoi_edge_point_b(1)),
            "Direction(1.0, 1.0)"
        );
        assert_eq!(trigen.out_voronoi_edge_point_a(2), 0);
        assert_eq!(
            format!("{:?}", trigen.out_voronoi_edge_point_b(2)),
            "Direction(-1.0, 0.0)"
        );
        Ok(())
    }

    #[test]
    fn mesh_1_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, Some(3), None, None)?;
        trigen
            .set_point(0, -100, 0.0, 0.0)?
            .set_point(1, -200, 1.0, 0.0)?
            .set_point(2, -300, 0.0, 1.0)?;
        trigen
            .set_segment(0, -10, 0, 1)?
            .set_segment(1, -20, 1, 2)?
            .set_segment(2, -30, 2, 0)?;
        trigen.generate_mesh(false, false, false, None, None)?;
        assert_eq!(trigen.out_npoint(), 3);
        assert_eq!(trigen.out_nsegment(), 3);
        assert_eq!(trigen.out_ncell(), 1);
        assert_eq!(trigen.out_cell_npoint(), 3);
        assert_eq!(trigen.out_point(0, 0), 0.0);
        assert_eq!(trigen.out_point(0, 1), 0.0);
        assert_eq!(trigen.out_point(1, 0), 1.0);
        assert_eq!(trigen.out_point(1, 1), 0.0);
        assert_eq!(trigen.out_point(2, 0), 0.0);
        assert_eq!(trigen.out_point(2, 1), 1.0);
        assert_eq!(trigen.out_point_marker(0), -100);
        assert_eq!(trigen.out_point_marker(1), -200);
        assert_eq!(trigen.out_point_marker(2), -300);
        assert_eq!(trigen.out_segment_marker(0), -10);
        assert_eq!(trigen.out_segment_marker(1), -20);
        assert_eq!(trigen.out_segment_marker(2), -30);
        assert_eq!(trigen.out_cell_point(0, 0), 0);
        assert_eq!(trigen.out_cell_point(0, 1), 1);
        assert_eq!(trigen.out_cell_point(0, 2), 2);
        assert_eq!(trigen.out_cell_attribute(0), 0);
        assert_eq!(trigen.out_cell_attribute(1), 0);
        assert_eq!(trigen.out_cell_attribute(2), 0);
        assert_eq!(trigen.out_voronoi_npoint(), 0);
        assert_eq!(trigen.out_voronoi_nedge(), 0);
        Ok(())
    }

    #[test]
    fn mesh_2_no_steiner_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(4, Some(4), None, None)?;
        trigen
            .set_point(0, -100, 0.0, 0.0)?
            .set_point(1, -200, 1.0, 0.0)?
            .set_point(2, -300, 1.0, 1.0)?
            .set_point(3, -400, 0.0, 1.0)?;
        trigen
            .set_segment(0, -10, 0, 1)?
            .set_segment(1, -20, 1, 2)?
            .set_segment(2, -30, 2, 3)?
            .set_segment(3, -40, 3, 0)?;
        trigen.generate_mesh(false, false, false, Some(0.1), None)?;

        if GENERATE_FIGURES {
            let mut plot = Plot::new();
            trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/test_mesh_2_no_steiner.svg")?;
        }

        assert_eq!(trigen.out_npoint(), 5);
        assert_eq!(trigen.out_nsegment(), 4);
        assert_eq!(trigen.out_ncell(), 4);
        assert_eq!(trigen.out_cell_npoint(), 3);

        println!("point markers");
        for i in 0..trigen.out_npoint() {
            println!("{} => {}", i, trigen.out_point_marker(i));
        }

        assert_eq!(trigen.out_point_marker(0), -100);
        assert_eq!(trigen.out_point_marker(1), -200);
        assert_eq!(trigen.out_point_marker(2), -300);
        assert_eq!(trigen.out_point_marker(3), -400);
        assert_eq!(trigen.out_point_marker(4), 0);

        println!("segments");
        for i in 0..trigen.out_nsegment() {
            let a = trigen.out_segment_point(i, 0);
            let b = trigen.out_segment_point(i, 1);
            let marker = trigen.out_segment_marker(i);
            println!("{:2} - {:2} => {}", a, b, marker);
        }

        let mut sides0 = vec![trigen.out_segment_point(0, 0), trigen.out_segment_point(0, 1)];
        let mut sides1 = vec![trigen.out_segment_point(1, 0), trigen.out_segment_point(1, 1)];
        let mut sides2 = vec![trigen.out_segment_point(2, 0), trigen.out_segment_point(2, 1)];
        let mut sides3 = vec![trigen.out_segment_point(3, 0), trigen.out_segment_point(3, 1)];
        sides0.sort();
        sides1.sort();
        sides2.sort();
        sides3.sort();
        assert_eq!(sides0, &[0, 1]);
        assert_eq!(sides1, &[1, 2]);
        assert_eq!(sides2, &[2, 3]);
        assert_eq!(sides3, &[0, 3]);
        assert_eq!(trigen.out_segment_marker(0), -10);
        assert_eq!(trigen.out_segment_marker(1), -20);
        assert_eq!(trigen.out_segment_marker(2), -30);
        assert_eq!(trigen.out_segment_marker(3), -40);
        Ok(())
    }

    #[test]
    fn mesh_2_ok_steiner_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(4, Some(4), None, None)?;
        trigen
            .set_point(0, -100, 0.0, 0.0)?
            .set_point(1, -200, 1.0, 0.0)?
            .set_point(2, -300, 1.0, 1.0)?
            .set_point(3, -400, 0.0, 1.0)?;
        trigen
            .set_segment(0, -10, 0, 1)?
            .set_segment(1, -20, 1, 2)?
            .set_segment(2, -30, 2, 3)?
            .set_segment(3, -40, 3, 0)?;
        trigen.generate_mesh(false, false, true, Some(0.1), None)?;

        if GENERATE_FIGURES {
            let mut plot = Plot::new();
            trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/test_mesh_2_ok_steiner.svg")?;
        }

        assert_eq!(trigen.out_npoint(), 13);
        assert_eq!(trigen.out_nsegment(), 8);
        assert_eq!(trigen.out_ncell(), 16);
        assert_eq!(trigen.out_cell_npoint(), 3);

        println!("point markers");
        for i in 0..trigen.out_npoint() {
            println!("{} => {}", i, trigen.out_point_marker(i));
        }

        assert_eq!(trigen.out_point_marker(0), -100);
        assert_eq!(trigen.out_point_marker(1), -200);
        assert_eq!(trigen.out_point_marker(2), -300);
        assert_eq!(trigen.out_point_marker(3), -400);
        assert_eq!(trigen.out_point_marker(4), 0);
        assert_eq!(trigen.out_point_marker(5), -40);
        assert_eq!(trigen.out_point_marker(6), -10);
        assert_eq!(trigen.out_point_marker(7), -30);
        assert_eq!(trigen.out_point_marker(8), 0);
        assert_eq!(trigen.out_point_marker(9), -20);
        assert_eq!(trigen.out_point_marker(10), 0);
        assert_eq!(trigen.out_point_marker(11), 0);
        assert_eq!(trigen.out_point_marker(12), 0);

        println!("segments");
        for i in 0..trigen.out_nsegment() {
            let a = trigen.out_segment_point(i, 0);
            let b = trigen.out_segment_point(i, 1);
            let marker = trigen.out_segment_marker(i);
            println!("{:2} - {:2} => {}", a, b, marker);
        }

        let mut sides0 = vec![trigen.out_segment_point(0, 0), trigen.out_segment_point(0, 1)];
        let mut sides1 = vec![trigen.out_segment_point(1, 0), trigen.out_segment_point(1, 1)];
        let mut sides2 = vec![trigen.out_segment_point(2, 0), trigen.out_segment_point(2, 1)];
        let mut sides3 = vec![trigen.out_segment_point(3, 0), trigen.out_segment_point(3, 1)];
        let mut sides4 = vec![trigen.out_segment_point(4, 0), trigen.out_segment_point(4, 1)];
        let mut sides5 = vec![trigen.out_segment_point(5, 0), trigen.out_segment_point(5, 1)];
        let mut sides6 = vec![trigen.out_segment_point(6, 0), trigen.out_segment_point(6, 1)];
        let mut sides7 = vec![trigen.out_segment_point(7, 0), trigen.out_segment_point(7, 1)];
        sides0.sort();
        sides1.sort();
        sides2.sort();
        sides3.sort();
        sides4.sort();
        sides5.sort();
        sides6.sort();
        sides7.sort();
        assert_eq!(sides0, &[1, 6]);
        assert_eq!(sides1, &[2, 9]);
        assert_eq!(sides2, &[3, 7]);
        assert_eq!(sides3, &[0, 5]);
        assert_eq!(sides4, &[3, 5]);
        assert_eq!(sides5, &[0, 6]);
        assert_eq!(sides6, &[2, 7]);
        assert_eq!(sides7, &[1, 9]);
        assert_eq!(trigen.out_segment_marker(0), -10);
        assert_eq!(trigen.out_segment_marker(1), -20);
        assert_eq!(trigen.out_segment_marker(2), -30);
        assert_eq!(trigen.out_segment_marker(3), -40);
        assert_eq!(trigen.out_segment_marker(4), -40);
        assert_eq!(trigen.out_segment_marker(5), -10);
        assert_eq!(trigen.out_segment_marker(6), -30);
        assert_eq!(trigen.out_segment_marker(7), -20);
        Ok(())
    }

    #[test]
    fn get_methods_work_with_wrong_indices() -> Result<(), StrError> {
        let trigen = Trigen::new(3, None, None, None)?;
        assert_eq!(trigen.out_point(100, 0), 0.0);
        assert_eq!(trigen.out_point(0, 100), 0.0);
        assert_eq!(trigen.out_cell_attribute(100), 0);
        assert_eq!(trigen.out_voronoi_point(100, 0), 0.0);
        assert_eq!(trigen.out_voronoi_point(0, 100), 0.0);
        assert_eq!(trigen.out_voronoi_edge_point_a(100), 0,);
        assert_eq!(format!("{:?}", trigen.out_voronoi_edge_point_b(100)), "Index(0)");
        Ok(())
    }

    #[test]
    fn draw_triangles_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, Some(3), None, None)?;
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0)?;
        trigen
            .set_segment(0, -10, 0, 1)?
            .set_segment(1, -20, 1, 2)?
            .set_segment(2, -30, 2, 0)?;
        trigen.generate_mesh(false, true, false, Some(0.25), None)?;
        let mut plot = Plot::new();
        trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
        if GENERATE_FIGURES {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/triangle_draw_triangles_works.svg")?;
        }
        Ok(())
    }

    #[test]
    fn draw_voronoi_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(5, None, None, None)?;
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 1.0, 1.0)?
            .set_point(3, 0, 0.0, 1.0)?
            .set_point(4, 0, 0.5, 0.5)?;
        trigen.generate_voronoi(false)?;
        assert_eq!(trigen.out_voronoi_npoint(), 4);
        let mut plot = Plot::new();
        trigen.draw_voronoi(&mut plot);
        if GENERATE_FIGURES {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/triangle_draw_voronoi_works.svg")?;
        }
        Ok(())
    }

    #[test]
    fn mesh_3_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(4, Some(3), Some(1), None)?;
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0)?
            .set_point(3, 0, 0.5, 0.5)?
            .set_region(0, 1, 0.5, 0.2, None)?;
        trigen
            .set_segment(0, -10, 0, 1)?
            .set_segment(1, -20, 1, 2)?
            .set_segment(2, -30, 2, 0)?;
        trigen.generate_mesh(false, true, false, Some(0.25), None)?;
        assert_eq!(trigen.out_ncell(), 2);
        assert_eq!(trigen.out_cell_attribute(0), 1);
        assert_eq!(trigen.out_cell_attribute(1), 1);
        let mut plot = Plot::new();
        trigen.draw_triangles(&mut plot, true, true, true, true, None, None, None);
        if GENERATE_FIGURES {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/triangle_mesh_3_works.svg")?;
        }
        Ok(())
    }

    #[test]
    fn mesh_4_works() -> Result<(), StrError> {
        let mut trigen = Trigen::new(12, Some(10), Some(2), Some(1))?;
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
            .set_point(11, 0, 1.0, 0.5)?
            .set_region(0, 111, 0.1, 0.1, None)?
            .set_region(1, 222, 0.1, 0.9, None)?
            .set_hole(0, 0.5, 0.5)?;
        trigen
            .set_segment(0, -10, 0, 1)?
            .set_segment(1, 0, 1, 2)?
            .set_segment(2, 0, 2, 3)?
            .set_segment(3, 0, 3, 0)?
            .set_segment(4, 0, 4, 5)?
            .set_segment(5, 0, 5, 6)?
            .set_segment(6, 0, 6, 7)?
            .set_segment(7, 0, 7, 4)?
            .set_segment(8, 0, 8, 9)?
            .set_segment(9, 0, 10, 11)?;
        trigen.generate_mesh(false, true, true, None, None)?;

        let mut plot = Plot::new();
        trigen.draw_triangles(&mut plot, true, true, true, true, Some(12.0), Some(20.0), None);
        if GENERATE_FIGURES {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/triangle_mesh_4_works.svg")?;
        }

        assert_eq!(trigen.out_ncell(), 14);
        assert_eq!(trigen.out_cell_attribute(0), 111);
        assert_eq!(trigen.out_cell_attribute(12), 222);
        Ok(())
    }
}
