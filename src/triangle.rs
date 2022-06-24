use crate::constants;
use crate::to_i32::to_i32;
use crate::StrError;
use plotpy::{Canvas, Curve, Plot, PolyCode, Text};
use std::collections::HashMap;

#[repr(C)]
pub(crate) struct ExtTriangle {
    data: [u8; 0],
    marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    // Triangle
    fn new_triangle(npoint: i32, nsegment: i32, nregion: i32, nhole: i32) -> *mut ExtTriangle;
    fn drop_triangle(triangle: *mut ExtTriangle);
    fn set_point(triangle: *mut ExtTriangle, index: i32, x: f64, y: f64) -> i32;
    fn set_segment(triangle: *mut ExtTriangle, index: i32, a: i32, b: i32) -> i32;
    fn set_region(
        triangle: *mut ExtTriangle,
        index: i32,
        x: f64,
        y: f64,
        attribute: i32,
        max_area: f64,
    ) -> i32;
    fn set_hole(triangle: *mut ExtTriangle, index: i32, x: f64, y: f64) -> i32;
    fn run_delaunay(triangle: *mut ExtTriangle, verbose: i32) -> i32;
    fn run_voronoi(triangle: *mut ExtTriangle, verbose: i32) -> i32;
    fn run_triangulate(
        triangle: *mut ExtTriangle,
        verbose: i32,
        quadratic: i32,
        global_max_area: f64,
        global_min_angle: f64,
    ) -> i32;
    fn get_npoint(triangle: *mut ExtTriangle) -> i32;
    fn get_ntriangle(triangle: *mut ExtTriangle) -> i32;
    fn get_ncorner(triangle: *mut ExtTriangle) -> i32;
    fn get_point(triangle: *mut ExtTriangle, index: i32, dim: i32) -> f64;
    fn get_triangle_corner(triangle: *mut ExtTriangle, index: i32, corner: i32) -> i32;
    fn get_triangle_attribute(triangle: *mut ExtTriangle, index: i32) -> i32;
    fn get_voronoi_npoint(triangle: *mut ExtTriangle) -> i32;
    fn get_voronoi_point(triangle: *mut ExtTriangle, index: i32, dim: i32) -> f64;
    fn get_voronoi_nedge(triangle: *mut ExtTriangle) -> i32;
    fn get_voronoi_edge_point(triangle: *mut ExtTriangle, index: i32, side: i32) -> i32;
    fn get_voronoi_edge_point_b_direction(triangle: *mut ExtTriangle, index: i32, dim: i32) -> f64;
}

/// Holds the index of an endpoint on a Voronoi edge or the direction of the Voronoi edge
#[derive(Clone, Debug)]
pub enum VoronoiEdgePoint {
    /// The index of the endpoint
    Index(usize),

    /// The direction of the infinite ray
    Direction(f64, f64),
}

/// Maps indices used in this library (tritet) to indices used in Triangle
///
/// ```text
/// This library (tritet)      Triangle
///         NODES               CORNERS
///           2                    2
///          / \                  / \
///         /   \                /   \
///        5     4              4     3
///       /       \            /       \
///      /         \          /         \
///     0-----3-----1        0-----5-----1
/// ```
const TRITET_TO_TRIANGLE: [usize; 6] = [0, 1, 2, 5, 3, 4];

/// Defines a set of "light" colors
const LIGHT_COLORS: [&'static str; 17] = [
    "#cbe4f9", "#cdf5f6", "#eff9da", "#f9ebdf", "#f9d8d6", "#d6cdea", "#acddde", "#caf1de",
    "#e1f8dc", "#fef8dd", "#ffe7c7", "#f7d8ba", "#d0fffe", "#fffddb", "#e4ffde", "#ffd3fd",
    "#ffe7d3",
];

/// Implements high-level functions to call Shewchuk's Triangle C-Code
pub struct Triangle {
    ext_triangle: *mut ExtTriangle, // data allocated by the c-code
    npoint: usize,                  // number of points
    nsegment: Option<usize>,        // number of segments
    nregion: Option<usize>,         // number of regions
    nhole: Option<usize>,           // number of holes
    all_points_set: bool,           // indicates that all points have been set
    all_segments_set: bool,         // indicates that all segments have been set
    all_regions_set: bool,          // indicates that all regions have been set
    all_holes_set: bool,            // indicates that all holes have been set
}

impl Triangle {
    /// Allocates a new instance
    pub fn new(
        npoint: usize,
        nsegment: Option<usize>,
        nregion: Option<usize>,
        nhole: Option<usize>,
    ) -> Result<Self, StrError> {
        if npoint < 3 {
            return Err("npoint must be ≥ 3");
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
            let ext_triangle = new_triangle(npoint_i32, nsegment_i32, nregion_i32, nhole_i32);
            if ext_triangle.is_null() {
                return Err("INTERNAL ERROR: cannot allocate ExtTriangle");
            }
            Ok(Triangle {
                ext_triangle,
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
    pub fn set_point(&mut self, index: usize, x: f64, y: f64) -> Result<&mut Self, StrError> {
        unsafe {
            let status = set_point(self.ext_triangle, to_i32(index), x, y);
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
    /// * `a` -- is the ID (index) of the first point on the segment
    /// * `b` -- is the ID (index) of the second point on the segment
    pub fn set_segment(&mut self, index: usize, a: usize, b: usize) -> Result<&mut Self, StrError> {
        let nsegment = match self.nsegment {
            Some(n) => n,
            None => return Err("cannot set segment because the number of segments is None"),
        };
        unsafe {
            let status = set_segment(self.ext_triangle, to_i32(index), to_i32(a), to_i32(b));
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
    /// * `x` -- is the x-coordinate of the hole
    /// * `y` -- is the x-coordinate of the hole
    /// * `attribute` -- is the attribute ID to group the triangles belonging to this region
    /// * `max_area` -- is the maximum area constraint for the triangles belonging to this region
    pub fn set_region(
        &mut self,
        index: usize,
        x: f64,
        y: f64,
        attribute: usize,
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
            let status = set_region(
                self.ext_triangle,
                to_i32(index),
                x,
                y,
                to_i32(attribute),
                area_constraint,
            );
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
    /// * `y` -- is the x-coordinate of the hole
    pub fn set_hole(&mut self, index: usize, x: f64, y: f64) -> Result<&mut Self, StrError> {
        let nhole = match self.nhole {
            Some(n) => n,
            None => return Err("cannot set hole because the number of holes is None"),
        };
        unsafe {
            let status = set_hole(self.ext_triangle, to_i32(index), x, y);
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
            let status = run_delaunay(self.ext_triangle, if verbose { 1 } else { 0 });
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
            let status = run_voronoi(self.ext_triangle, if verbose { 1 } else { 0 });
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
    /// * `global_max_area` -- The maximum area constraint for all generated triangles
    /// * `global_min_angle` -- The minimum angle constraint is given in degrees (the default minimum angle is twenty degrees)
    pub fn generate_mesh(
        &mut self,
        verbose: bool,
        quadratic: bool,
        global_max_area: Option<f64>,
        global_min_angle: Option<f64>,
    ) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("cannot generate mesh because not all points are set");
        }
        if !self.all_segments_set {
            return Err("cannot generate mesh because not all segments are set");
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
            let status = run_triangulate(
                self.ext_triangle,
                if verbose { 1 } else { 0 },
                if quadratic { 1 } else { 0 },
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
                    return Err("list of segments must be defined first");
                }
                if status == constants::TRITET_ERROR_STRING_CONCAT {
                    return Err("INTERNAL ERROR: cannot write string with commands for Triangle");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        Ok(())
    }

    /// Returns the number of points of the Delaunay triangulation (constrained or not)
    pub fn npoint(&self) -> usize {
        unsafe { get_npoint(self.ext_triangle) as usize }
    }

    /// Returns the number of triangles on the Delaunay triangulation (constrained or not)
    pub fn ntriangle(&self) -> usize {
        unsafe { get_ntriangle(self.ext_triangle) as usize }
    }

    /// Returns the number of nodes on a triangle (e.g., 3 or 6)
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
    pub fn nnode(&self) -> usize {
        unsafe { get_ncorner(self.ext_triangle) as usize }
    }

    /// Returns the x-y coordinates of a point
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from 0 to `npoint`
    /// * `dim` -- is the space dimension index: 0 or 1
    ///
    /// # Warning
    ///
    /// This function will return 0.0 if either `index` or `dim` are out of range.
    pub fn point(&self, index: usize, dim: usize) -> f64 {
        unsafe { get_point(self.ext_triangle, to_i32(index), to_i32(dim)) }
    }

    /// Returns the ID of a Triangle's node
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
    /// * `index` -- is the index of the triangle and goes from 0 to `ntriangle`
    /// * `m` -- is the local index of the node and goes from 0 to `nnode`
    ///
    /// # Warning
    ///
    /// This function will return 0 if either `index` or `m` are out of range.
    pub fn triangle_node(&self, index: usize, m: usize) -> usize {
        unsafe {
            let corner = TRITET_TO_TRIANGLE[m];
            get_triangle_corner(self.ext_triangle, to_i32(index), to_i32(corner)) as usize
        }
    }

    /// Returns the attribute ID of a triangle
    ///
    /// # Warning
    ///
    /// This function will return 0 if either `index` is out of range.
    pub fn triangle_attribute(&self, index: usize) -> usize {
        unsafe { get_triangle_attribute(self.ext_triangle, to_i32(index)) as usize }
    }

    /// Returns the number of points of the Voronoi tessellation
    pub fn voronoi_npoint(&self) -> usize {
        unsafe { get_voronoi_npoint(self.ext_triangle) as usize }
    }

    /// Returns the x-y coordinates of a point on the Voronoi tessellation
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from 0 to `voronoi_npoint`
    /// * `dim` -- is the space dimension index: 0 or 1
    ///
    /// # Warning
    ///
    /// This function will return 0.0 if either `index` or `dim` are out of range.
    pub fn voronoi_point(&self, index: usize, dim: usize) -> f64 {
        unsafe { get_voronoi_point(self.ext_triangle, to_i32(index), to_i32(dim)) }
    }

    /// Returns the number of edges on the Voronoi tessellation
    pub fn voronoi_nedge(&self) -> usize {
        unsafe { get_voronoi_nedge(self.ext_triangle) as usize }
    }

    /// Returns the index of the first endpoint on a Voronoi edge
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the edge and goes from 0 to `voronoi_nedge`
    ///
    /// # Warning
    ///
    /// This function will return 0 if either `index` is out of range.
    pub fn voronoi_edge_point_a(&self, index: usize) -> usize {
        unsafe { get_voronoi_edge_point(self.ext_triangle, to_i32(index), 0) as usize }
    }

    /// Returns the index of the second endpoint on a Voronoi edge or the direction of the Voronoi edge
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the edge and goes from 0 to `voronoi_nedge`
    ///
    /// # Warning
    ///
    /// This function will return Index(0) if either `index` is out of range.
    pub fn voronoi_edge_point_b(&self, index: usize) -> VoronoiEdgePoint {
        unsafe {
            let index_i32 = to_i32(index);
            let id = get_voronoi_edge_point(self.ext_triangle, index_i32, 1);
            if id == -1 {
                let x = get_voronoi_edge_point_b_direction(self.ext_triangle, index_i32, 0);
                let y = get_voronoi_edge_point_b_direction(self.ext_triangle, index_i32, 1);
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
        let n_triangle = self.ntriangle();
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
        for tri in 0..n_triangle {
            let attribute = self.triangle_attribute(tri);
            let color = match colors.get(&attribute) {
                Some(c) => c,
                None => {
                    let c = LIGHT_COLORS[index_color % LIGHT_COLORS.len()];
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
                let p = self.triangle_node(tri, m);
                for dim in 0..2 {
                    x[dim] = self.point(p, dim);
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
                for dim in 0..2 {
                    x[dim] = self.point(self.triangle_node(tri, 0), dim);
                    xatt[dim] = (x[dim] + xmid[dim]) / 2.0;
                }
                attribute_ids.draw(xatt[0], xatt[1], format!("[{}]", attribute).as_str());
            }
        }
        if with_point_ids {
            for p in 0..self.npoint() {
                let x = self.point(p, 0);
                let y = self.point(p, 1);
                point_ids.draw(x, y, format!("{}", p).as_str());
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
        if self.voronoi_npoint() < 1 || self.voronoi_nedge() < 1 {
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
        for p in 0..self.npoint() {
            for dim in 0..2 {
                x[dim] = self.point(p, dim);
                min[dim] = f64::min(min[dim], x[dim]);
                max[dim] = f64::max(max[dim], x[dim]);
            }
            markers.draw(&[x[0]], &[x[1]]);
        }
        for q in 0..self.voronoi_npoint() {
            for dim in 0..2 {
                x[dim] = self.voronoi_point(q, dim);
                min[dim] = f64::min(min[dim], x[dim]);
                max[dim] = f64::max(max[dim], x[dim]);
            }
        }
        let mut canvas = Canvas::new();
        canvas.polycurve_begin();
        for e in 0..self.voronoi_nedge() {
            let a = self.voronoi_edge_point_a(e);
            let xa = self.voronoi_point(a, 0);
            let ya = self.voronoi_point(a, 1);
            let b_or_direction = self.voronoi_edge_point_b(e);
            match b_or_direction {
                VoronoiEdgePoint::Index(b) => {
                    let xb = self.voronoi_point(b, 0);
                    let yb = self.voronoi_point(b, 1);
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

impl Drop for Triangle {
    /// Tells the c-code to release memory
    fn drop(&mut self) {
        unsafe {
            drop_triangle(self.ext_triangle);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Triangle;
    use crate::{StrError, VoronoiEdgePoint};
    use plotpy::Plot;

    #[test]
    fn derive_works() {
        let option = VoronoiEdgePoint::Index(0);
        let cloned = option.clone();
        assert_eq!(format!("{:?}", option), "Index(0)");
        assert_eq!(format!("{:?}", cloned), "Index(0)");
    }

    #[test]
    fn new_captures_some_errors() {
        assert_eq!(
            Triangle::new(2, None, None, None).err(),
            Some("npoint must be ≥ 3")
        );
    }

    #[test]
    fn new_works() -> Result<(), StrError> {
        let triangle = Triangle::new(3, Some(3), None, None)?;
        assert_eq!(triangle.ext_triangle.is_null(), false);
        assert_eq!(triangle.npoint, 3);
        assert_eq!(triangle.nsegment, Some(3));
        assert_eq!(triangle.nregion, None);
        assert_eq!(triangle.nhole, None);
        assert_eq!(triangle.all_points_set, false);
        assert_eq!(triangle.all_segments_set, false);
        assert_eq!(triangle.all_regions_set, false);
        assert_eq!(triangle.all_holes_set, false);
        Ok(())
    }

    #[test]
    fn set_point_captures_some_errors() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(
            triangle.set_point(4, 0.0, 0.0).err(),
            Some("index of point is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn set_segment_captures_some_errors() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(
            triangle.set_segment(0, 0, 1).err(),
            Some("cannot set segment because the number of segments is None")
        );
        let mut triangle = Triangle::new(3, Some(3), None, None)?;
        assert_eq!(
            triangle.set_segment(4, 0, 1).err(),
            Some("index of segment is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn set_region_captures_some_errors() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(
            triangle.set_region(0, 0.33, 0.33, 1, Some(0.1)).err(),
            Some("cannot set region because the number of regions is None")
        );
        let mut triangle = Triangle::new(3, Some(3), Some(1), None)?;
        assert_eq!(
            triangle.set_region(1, 0.33, 0.33, 1, Some(0.1)).err(),
            Some("index of region is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn set_hole_captures_some_errors() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(
            triangle.set_hole(0, 0.33, 0.33).err(),
            Some("cannot set hole because the number of holes is None")
        );
        let mut triangle = Triangle::new(3, Some(3), Some(1), Some(1))?;
        assert_eq!(
            triangle.set_hole(1, 0.33, 0.33).err(),
            Some("index of hole is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn delaunay_1_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 0.0, 1.0)?;
        triangle.generate_delaunay(false)?;
        assert_eq!(triangle.npoint(), 3);
        assert_eq!(triangle.ntriangle(), 1);
        assert_eq!(triangle.nnode(), 3);
        assert_eq!(triangle.point(0, 0), 0.0);
        assert_eq!(triangle.point(0, 1), 0.0);
        assert_eq!(triangle.point(1, 0), 1.0);
        assert_eq!(triangle.point(1, 1), 0.0);
        assert_eq!(triangle.point(2, 0), 0.0);
        assert_eq!(triangle.point(2, 1), 1.0);
        assert_eq!(triangle.triangle_node(0, 0), 0);
        assert_eq!(triangle.triangle_node(0, 1), 1);
        assert_eq!(triangle.triangle_node(0, 2), 2);
        assert_eq!(triangle.voronoi_npoint(), 0);
        assert_eq!(triangle.voronoi_nedge(), 0);
        Ok(())
    }

    #[test]
    fn voronoi_1_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 0.0, 1.0)?;
        triangle.generate_voronoi(false)?;
        assert_eq!(triangle.npoint(), 3);
        assert_eq!(triangle.ntriangle(), 1);
        assert_eq!(triangle.nnode(), 3);
        assert_eq!(triangle.point(0, 0), 0.0);
        assert_eq!(triangle.point(0, 1), 0.0);
        assert_eq!(triangle.point(1, 0), 1.0);
        assert_eq!(triangle.point(1, 1), 0.0);
        assert_eq!(triangle.point(2, 0), 0.0);
        assert_eq!(triangle.point(2, 1), 1.0);
        assert_eq!(triangle.triangle_node(0, 0), 0);
        assert_eq!(triangle.triangle_node(0, 1), 1);
        assert_eq!(triangle.triangle_node(0, 2), 2);
        assert_eq!(triangle.voronoi_npoint(), 1);
        assert_eq!(triangle.voronoi_point(0, 0), 0.5);
        assert_eq!(triangle.voronoi_point(0, 1), 0.5);
        assert_eq!(triangle.voronoi_nedge(), 3);
        assert_eq!(triangle.voronoi_edge_point_a(0), 0);
        assert_eq!(
            format!("{:?}", triangle.voronoi_edge_point_b(0)),
            "Direction(0.0, -1.0)"
        );
        assert_eq!(triangle.voronoi_edge_point_a(1), 0);
        assert_eq!(
            format!("{:?}", triangle.voronoi_edge_point_b(1)),
            "Direction(1.0, 1.0)"
        );
        assert_eq!(triangle.voronoi_edge_point_a(2), 0);
        assert_eq!(
            format!("{:?}", triangle.voronoi_edge_point_b(2)),
            "Direction(-1.0, 0.0)"
        );
        Ok(())
    }

    #[test]
    fn mesh_1_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, Some(3), None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 0.0, 1.0)?;
        triangle
            .set_segment(0, 0, 1)?
            .set_segment(1, 1, 2)?
            .set_segment(2, 2, 0)?;
        triangle.generate_mesh(false, false, None, None)?;
        assert_eq!(triangle.npoint(), 3);
        assert_eq!(triangle.ntriangle(), 1);
        assert_eq!(triangle.nnode(), 3);
        assert_eq!(triangle.point(0, 0), 0.0);
        assert_eq!(triangle.point(0, 1), 0.0);
        assert_eq!(triangle.point(1, 0), 1.0);
        assert_eq!(triangle.point(1, 1), 0.0);
        assert_eq!(triangle.point(2, 0), 0.0);
        assert_eq!(triangle.point(2, 1), 1.0);
        assert_eq!(triangle.triangle_node(0, 0), 0);
        assert_eq!(triangle.triangle_node(0, 1), 1);
        assert_eq!(triangle.triangle_node(0, 2), 2);
        assert_eq!(triangle.triangle_attribute(0), 0);
        assert_eq!(triangle.triangle_attribute(1), 0);
        assert_eq!(triangle.triangle_attribute(2), 0);
        assert_eq!(triangle.voronoi_npoint(), 0);
        assert_eq!(triangle.voronoi_nedge(), 0);
        Ok(())
    }

    #[test]
    fn mesh_2_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, Some(3), None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 0.0, 1.0)?;
        triangle
            .set_segment(0, 0, 1)?
            .set_segment(1, 1, 2)?
            .set_segment(2, 2, 0)?;
        triangle.generate_mesh(false, true, Some(0.1), Some(20.0))?;
        assert_eq!(triangle.npoint(), 22);
        assert_eq!(triangle.ntriangle(), 7);
        assert_eq!(triangle.nnode(), 6);
        Ok(())
    }

    #[test]
    fn get_methods_work_with_wrong_indices() -> Result<(), StrError> {
        let triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(triangle.point(100, 0), 0.0);
        assert_eq!(triangle.point(0, 100), 0.0);
        assert_eq!(triangle.triangle_attribute(100), 0);
        assert_eq!(triangle.voronoi_point(100, 0), 0.0);
        assert_eq!(triangle.voronoi_point(0, 100), 0.0);
        assert_eq!(triangle.voronoi_edge_point_a(100), 0,);
        assert_eq!(
            format!("{:?}", triangle.voronoi_edge_point_b(100)),
            "Index(0)"
        );
        Ok(())
    }

    #[test]
    fn draw_triangles_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, Some(3), None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 0.0, 1.0)?;
        triangle
            .set_segment(0, 0, 1)?
            .set_segment(1, 1, 2)?
            .set_segment(2, 2, 0)?;
        triangle.generate_mesh(false, true, Some(0.25), None)?;
        let mut plot = Plot::new();
        triangle.draw_triangles(&mut plot, true, true, true, true, None, None, None);
        if false {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/draw_triangles_works.svg")?;
        }
        Ok(())
    }

    #[test]
    fn draw_voronoi_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(5, None, None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 1.0, 1.0)?
            .set_point(3, 0.0, 1.0)?
            .set_point(4, 0.5, 0.5)?;
        // let mut triangle = Triangle::new(10, None, None, None)?;
        // triangle
        //     .set_point(0, 0.478554, 0.00869692)?
        //     .set_point(1, 0.13928, 0.180603)?
        //     .set_point(2, 0.578587, 0.760349)?
        //     .set_point(3, 0.903726, 0.975904)?
        //     .set_point(4, 0.0980015, 0.981755)?
        //     .set_point(5, 0.133721, 0.348832)?
        //     .set_point(6, 0.648071, 0.369534)?
        //     .set_point(7, 0.230951, 0.558482)?
        //     .set_point(8, 0.0307942, 0.459123)?
        //     .set_point(9, 0.540745, 0.331184)?;
        triangle.generate_voronoi(false)?;
        let mut plot = Plot::new();
        triangle.draw_voronoi(&mut plot);
        if false {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/draw_voronoi_works.svg")?;
        }
        Ok(())
    }

    #[test]
    fn mesh_3_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(4, Some(3), Some(1), None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 0.0, 1.0)?
            .set_point(3, 0.5, 0.5)?
            .set_region(0, 0.5, 0.2, 1, None)?;
        triangle
            .set_segment(0, 0, 1)?
            .set_segment(1, 1, 2)?
            .set_segment(2, 2, 0)?;
        triangle.generate_mesh(false, true, Some(0.25), None)?;
        assert_eq!(triangle.ntriangle(), 2);
        assert_eq!(triangle.triangle_attribute(0), 1);
        assert_eq!(triangle.triangle_attribute(1), 1);
        let mut plot = Plot::new();
        triangle.draw_triangles(&mut plot, true, true, true, true, None, None, None);
        if false {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/mesh_3_works.svg")?;
        }
        Ok(())
    }

    #[test]
    fn mesh_4_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(12, Some(10), Some(2), Some(1))?;
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
            .set_point(11, 1.0, 0.5)?
            .set_region(0, 0.1, 0.1, 1, None)?
            .set_region(1, 0.1, 0.9, 2, None)?
            .set_hole(0, 0.5, 0.5)?;
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
        triangle.generate_mesh(false, true, None, None)?;
        assert_eq!(triangle.ntriangle(), 14);
        assert_eq!(triangle.triangle_attribute(0), 1);
        assert_eq!(triangle.triangle_attribute(12), 2);
        let mut plot = Plot::new();
        triangle.draw_triangles(
            &mut plot,
            true,
            true,
            true,
            true,
            Some(12.0),
            Some(20.0),
            None,
        );
        if false {
            plot.set_equal_axes(true)
                .set_figure_size_points(600.0, 600.0)
                .save("/tmp/tritet/mesh_4_works.svg")?;
        }
        Ok(())
    }
}
