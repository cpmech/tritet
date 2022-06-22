use crate::constants;
use crate::to_i32::to_i32;
use crate::StrError;

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
    fn get_point_x(triangle: *mut ExtTriangle, index: i32) -> f64;
    fn get_point_y(triangle: *mut ExtTriangle, index: i32) -> f64;
    fn get_triangle_corner(triangle: *mut ExtTriangle, index: i32, corner: i32) -> i32;
    fn get_voronoi_npoint(triangle: *mut ExtTriangle) -> i32;
    fn get_voronoi_point_x(triangle: *mut ExtTriangle, index: i32) -> f64;
    fn get_voronoi_point_y(triangle: *mut ExtTriangle, index: i32) -> f64;
    fn get_voronoi_nedge(triangle: *mut ExtTriangle) -> i32;
    fn get_voronoi_edge_point_a(triangle: *mut ExtTriangle, index: i32) -> i32;
    fn get_voronoi_edge_point_b(triangle: *mut ExtTriangle, index: i32) -> i32;
    fn get_voronoi_edge_point_b_direction_x(triangle: *mut ExtTriangle, index: i32) -> f64;
    fn get_voronoi_edge_point_b_direction_y(triangle: *mut ExtTriangle, index: i32) -> f64;
}

/// Defines the index or the direction related to the second point (B) of an edge on a Voronoi tesselation
#[derive(Clone, Debug)]
pub enum VoronoiEdgePointB {
    /// The index of point B
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
                return Err("INTERNAL ERROR: Cannot allocate ExtTriangle");
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
                    return Err("INTERNAL ERROR: Found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: Found NULL point list");
                }
                if status == constants::TRITET_ERROR_INVALID_POINT_INDEX {
                    return Err("Index of point is out of bounds");
                }
                return Err("INTERNAL ERROR: Some error occurred");
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
    pub fn set_segment(&mut self, index: usize, a: usize, b: usize) -> Result<&mut Self, StrError> {
        let nsegment = match self.nsegment {
            Some(n) => n,
            None => {
                return Err(
                    "The number of segments (given to 'new') must not be None to set segment",
                )
            }
        };
        unsafe {
            let status = set_segment(self.ext_triangle, to_i32(index), to_i32(a), to_i32(b));
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: Found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_SEGMENT_LIST {
                    return Err("INTERNAL ERROR: Found NULL segment list");
                }
                if status == constants::TRITET_ERROR_INVALID_SEGMENT_INDEX {
                    return Err("Index of segment is out of bounds");
                }
                return Err("INTERNAL ERROR: Some error occurred");
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
    pub fn set_region(
        &mut self,
        index: usize,
        x: f64,
        y: f64,
        attribute: usize,
        max_area: f64,
    ) -> Result<&mut Self, StrError> {
        let nregion = match self.nregion {
            Some(n) => n,
            None => {
                return Err("The number of regions (given to 'new') must not be None to set region")
            }
        };
        unsafe {
            let status = set_region(
                self.ext_triangle,
                to_i32(index),
                x,
                y,
                to_i32(attribute),
                max_area,
            );
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: Found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_REGION_LIST {
                    return Err("INTERNAL ERROR: Found NULL region list");
                }
                if status == constants::TRITET_ERROR_INVALID_REGION_INDEX {
                    return Err("Index of region is out of bounds");
                }
                return Err("INTERNAL ERROR: Some error occurred");
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
    pub fn set_hole(&mut self, index: usize, x: f64, y: f64) -> Result<&mut Self, StrError> {
        let nhole = match self.nhole {
            Some(n) => n,
            None => {
                return Err("The number of holes (given to 'new') must not be None to set hole")
            }
        };
        unsafe {
            let status = set_hole(self.ext_triangle, to_i32(index), x, y);
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: Found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_HOLE_LIST {
                    return Err("INTERNAL ERROR: Found NULL hole list");
                }
                if status == constants::TRITET_ERROR_INVALID_HOLE_INDEX {
                    return Err("Index of hole is out of bounds");
                }
                return Err("INTERNAL ERROR: Some error occurred");
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
    pub fn generate_delaunay(&self, verbose: bool) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("All points must be set to generate Delaunay triangulation");
        }
        unsafe {
            let status = run_delaunay(self.ext_triangle, if verbose { 1 } else { 0 });
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: Found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: Found NULL point list");
                }
                return Err("INTERNAL ERROR: Some error occurred");
            }
        }
        Ok(())
    }

    /// Generates a Voronoi tesselation and Delaunay triangulation
    pub fn generate_voronoi(&self, verbose: bool) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("All points must be set to generate Voronoi tessellation");
        }
        unsafe {
            let status = run_voronoi(self.ext_triangle, if verbose { 1 } else { 0 });
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: Found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: Found NULL point list");
                }
                return Err("INTERNAL ERROR: Some error occurred");
            }
        }
        Ok(())
    }

    /// Generates a conforming constrained Delaunay triangulation with some quality constraints
    ///
    /// * `global_min_angle` -- The minimum angle constraint is given in degrees (the default minimum angle is twenty degrees)
    pub fn generate_mesh(
        &mut self,
        verbose: bool,
        quadratic: bool,
        global_max_area: Option<f64>,
        global_min_angle: Option<f64>,
    ) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("All points must be set to generate mesh");
        }
        if !self.all_segments_set {
            return Err("All segments must be set to generate mesh");
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
                    return Err("INTERNAL ERROR: Found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: Found NULL point list");
                }
                if status == constants::TRITET_ERROR_NULL_SEGMENT_LIST {
                    return Err("List of segments must be defined first");
                }
                if status == constants::TRITET_ERROR_STRING_CONCAT {
                    return Err("Cannot write string with commands for Triangle");
                }
                return Err("INTERNAL ERROR: Some error occurred");
            }
        }
        Ok(())
    }

    /// Returns the number of points of the Delaunay triangulation (constrained or not)
    pub fn get_npoint(&self) -> usize {
        unsafe { get_npoint(self.ext_triangle) as usize }
    }

    /// Returns the number of triangles on the Delaunay triangulation (constrained or not)
    pub fn get_ntriangle(&self) -> usize {
        unsafe { get_ntriangle(self.ext_triangle) as usize }
    }

    /// Returns the number of nodes on a triangle (e.g., 3 or 6)
    pub fn get_nnode(&self) -> usize {
        unsafe { get_ncorner(self.ext_triangle) as usize }
    }

    /// Returns the x-y coordinates of a point
    pub fn get_point(&self, index: usize) -> (f64, f64) {
        unsafe {
            let index_i32 = to_i32(index);
            let x = get_point_x(self.ext_triangle, index_i32);
            let y = get_point_y(self.ext_triangle, index_i32);
            (x, y)
        }
    }

    /// Returns the ID of a Triangle's node
    ///
    /// * `m` -- goes from 0 to `nnode`
    pub fn get_triangle_node(&self, index: usize, m: usize) -> usize {
        unsafe {
            let corner = TRITET_TO_TRIANGLE[m];
            get_triangle_corner(self.ext_triangle, to_i32(index), to_i32(corner)) as usize
        }
    }

    /// Returns the number of points of the Voronoi tesselation
    pub fn get_voronoi_npoint(&self) -> usize {
        unsafe { get_voronoi_npoint(self.ext_triangle) as usize }
    }

    /// Returns the x-y coordinates of a point on the Voronoi tesselation
    pub fn get_voronoi_point(&self, index: usize) -> (f64, f64) {
        unsafe {
            let index_i32 = to_i32(index);
            let x = get_voronoi_point_x(self.ext_triangle, index_i32);
            let y = get_voronoi_point_y(self.ext_triangle, index_i32);
            (x, y)
        }
    }

    /// Returns the number of edges on the Voronoi tesselation
    pub fn get_voronoi_nedge(&self) -> usize {
        unsafe { get_voronoi_nedge(self.ext_triangle) as usize }
    }

    /// Returns the first point on an edge of the Voronoi tesselation
    pub fn get_voronoi_edge_point_a(&self, index: usize) -> usize {
        unsafe { get_voronoi_edge_point_a(self.ext_triangle, to_i32(index)) as usize }
    }

    /// Returns the second point (or the direction) on an edge of the Voronoi tesselation
    pub fn get_voronoi_edge_point_b(&self, index: usize) -> VoronoiEdgePointB {
        unsafe {
            let index_i32 = to_i32(index);
            let b = get_voronoi_edge_point_b(self.ext_triangle, index_i32);
            if b == -1 {
                let x = get_voronoi_edge_point_b_direction_x(self.ext_triangle, index_i32);
                let y = get_voronoi_edge_point_b_direction_y(self.ext_triangle, index_i32);
                VoronoiEdgePointB::Direction(x, y)
            } else {
                VoronoiEdgePointB::Index(b as usize)
            }
        }
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
    use crate::StrError;

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
            Some("Index of point is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn set_segment_captures_some_errors() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(
            triangle.set_segment(0, 0, 1).err(),
            Some("The number of segments (given to 'new') must not be None to set segment")
        );
        let mut triangle = Triangle::new(3, Some(3), None, None)?;
        assert_eq!(
            triangle.set_segment(4, 0, 1).err(),
            Some("Index of segment is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn set_region_captures_some_errors() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(
            triangle.set_region(0, 0.33, 0.33, 1, 0.1).err(),
            Some("The number of regions (given to 'new') must not be None to set region")
        );
        let mut triangle = Triangle::new(3, Some(3), Some(1), None)?;
        assert_eq!(
            triangle.set_region(1, 0.33, 0.33, 1, 0.1).err(),
            Some("Index of region is out of bounds")
        );
        Ok(())
    }

    #[test]
    fn set_hole_captures_some_errors() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, None, None, None)?;
        assert_eq!(
            triangle.set_hole(0, 0.33, 0.33).err(),
            Some("The number of holes (given to 'new') must not be None to set hole")
        );
        let mut triangle = Triangle::new(3, Some(3), Some(1), Some(1))?;
        assert_eq!(
            triangle.set_hole(1, 0.33, 0.33).err(),
            Some("Index of hole is out of bounds")
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
        assert_eq!(triangle.get_npoint(), 3);
        assert_eq!(triangle.get_ntriangle(), 1);
        assert_eq!(triangle.get_nnode(), 3);
        assert_eq!(triangle.get_point(0), (0.0, 0.0));
        assert_eq!(triangle.get_point(1), (1.0, 0.0));
        assert_eq!(triangle.get_point(2), (0.0, 1.0));
        assert_eq!(triangle.get_triangle_node(0, 0), 0);
        assert_eq!(triangle.get_triangle_node(0, 1), 1);
        assert_eq!(triangle.get_triangle_node(0, 2), 2);
        assert_eq!(triangle.get_voronoi_npoint(), 0);
        assert_eq!(triangle.get_voronoi_nedge(), 0);
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
        assert_eq!(triangle.get_npoint(), 3);
        assert_eq!(triangle.get_ntriangle(), 1);
        assert_eq!(triangle.get_nnode(), 3);
        assert_eq!(triangle.get_point(0), (0.0, 0.0));
        assert_eq!(triangle.get_point(1), (1.0, 0.0));
        assert_eq!(triangle.get_point(2), (0.0, 1.0));
        assert_eq!(triangle.get_triangle_node(0, 0), 0);
        assert_eq!(triangle.get_triangle_node(0, 1), 1);
        assert_eq!(triangle.get_triangle_node(0, 2), 2);
        assert_eq!(triangle.get_voronoi_npoint(), 1);
        assert_eq!(triangle.get_voronoi_point(0), (0.5, 0.5));
        assert_eq!(triangle.get_voronoi_nedge(), 3);
        assert_eq!(triangle.get_voronoi_edge_point_a(0), 0);
        assert_eq!(
            format!("{:?}", triangle.get_voronoi_edge_point_b(0)),
            "Direction(0.0, -1.0)"
        );
        assert_eq!(triangle.get_voronoi_edge_point_a(1), 0);
        assert_eq!(
            format!("{:?}", triangle.get_voronoi_edge_point_b(1)),
            "Direction(1.0, 1.0)"
        );
        assert_eq!(triangle.get_voronoi_edge_point_a(2), 0);
        assert_eq!(
            format!("{:?}", triangle.get_voronoi_edge_point_b(2)),
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
        assert_eq!(triangle.get_npoint(), 3);
        assert_eq!(triangle.get_ntriangle(), 1);
        assert_eq!(triangle.get_nnode(), 3);
        assert_eq!(triangle.get_point(0), (0.0, 0.0));
        assert_eq!(triangle.get_point(1), (1.0, 0.0));
        assert_eq!(triangle.get_point(2), (0.0, 1.0));
        assert_eq!(triangle.get_triangle_node(0, 0), 0);
        assert_eq!(triangle.get_triangle_node(0, 1), 1);
        assert_eq!(triangle.get_triangle_node(0, 2), 2);
        assert_eq!(triangle.get_voronoi_npoint(), 0);
        assert_eq!(triangle.get_voronoi_nedge(), 0);
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
        assert_eq!(triangle.get_npoint(), 22);
        assert_eq!(triangle.get_ntriangle(), 7);
        assert_eq!(triangle.get_nnode(), 6);
        Ok(())
    }
}
