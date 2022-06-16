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
    fn set_segment(triangle: *mut ExtTriangle, index: i32, left: i32, right: i32) -> i32;
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

pub struct Triangle {
    ext_triangle: *mut ExtTriangle, // data allocated by the c-code
}

impl Triangle {
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
            Ok(Triangle { ext_triangle })
        }
    }

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
        Ok(self)
    }

    pub fn set_segment(
        &mut self,
        index: usize,
        left: usize,
        right: usize,
    ) -> Result<&mut Self, StrError> {
        unsafe {
            let status = set_segment(
                self.ext_triangle,
                to_i32(index),
                to_i32(left),
                to_i32(right),
            );
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
        Ok(self)
    }

    pub fn set_region(
        &mut self,
        index: usize,
        x: f64,
        y: f64,
        attribute: usize,
        max_area: f64,
    ) -> Result<&mut Self, StrError> {
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
        Ok(self)
    }

    pub fn set_hole(&mut self, index: usize, x: f64, y: f64) -> Result<&mut Self, StrError> {
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
        Ok(self)
    }

    pub fn generate_delaunay(&self, verbose: bool) -> Result<(), StrError> {
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

    pub fn generate_voronoi(&self, verbose: bool) -> Result<(), StrError> {
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

    // The minimum angle constraint is given in degrees (the default minimum angle is twenty degrees)
    pub fn generate_mesh(
        &mut self,
        verbose: bool,
        quadratic: bool,
        global_max_area: Option<f64>,
        global_min_angle: Option<f64>,
    ) -> Result<(), StrError> {
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

    pub fn get_npoint(&self) -> usize {
        unsafe { get_npoint(self.ext_triangle) as usize }
    }

    pub fn get_ntriangle(&self) -> usize {
        unsafe { get_ntriangle(self.ext_triangle) as usize }
    }

    pub fn get_ncorner(&self) -> usize {
        unsafe { get_ncorner(self.ext_triangle) as usize }
    }

    pub fn get_point_x(&self, index: usize) -> f64 {
        unsafe { get_point_x(self.ext_triangle, to_i32(index)) }
    }

    pub fn get_point_y(&self, index: usize) -> f64 {
        unsafe { get_point_y(self.ext_triangle, to_i32(index)) }
    }

    pub fn get_triangle_corner(&self, index: usize, corner: usize) -> usize {
        unsafe { get_triangle_corner(self.ext_triangle, to_i32(index), to_i32(corner)) as usize }
    }

    pub fn get_voronoi_npoint(&self) -> usize {
        unsafe { get_voronoi_npoint(self.ext_triangle) as usize }
    }

    pub fn get_voronoi_point(&self, index: usize) -> (f64, f64) {
        unsafe {
            let index_i32 = to_i32(index);
            let x = get_voronoi_point_x(self.ext_triangle, index_i32);
            let y = get_voronoi_point_y(self.ext_triangle, index_i32);
            (x, y)
        }
    }

    pub fn get_voronoi_nedge(&self) -> usize {
        unsafe { get_voronoi_nedge(self.ext_triangle) as usize }
    }

    pub fn get_voronoi_edge(&self, index: usize) -> (usize, Option<usize>, Option<(f64, f64)>) {
        unsafe {
            let index_i32 = to_i32(index);
            let a = get_voronoi_edge_point_a(self.ext_triangle, index_i32);
            let b = get_voronoi_edge_point_b(self.ext_triangle, index_i32);
            if b == -1 {
                let x = get_voronoi_edge_point_b_direction_x(self.ext_triangle, index_i32);
                let y = get_voronoi_edge_point_b_direction_y(self.ext_triangle, index_i32);
                (a as usize, None, Some((x, y)))
            } else {
                (a as usize, Some(b as usize), None)
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
    fn new_works() -> Result<(), StrError> {
        let triangle = Triangle::new(3, Some(3), None, None)?;
        assert_eq!(triangle.ext_triangle.is_null(), false);
        Ok(())
    }

    #[test]
    fn delaunay_1_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(15, None, None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, -0.416, 0.909)?
            .set_point(2, -1.35, 0.436)?
            .set_point(3, -1.64, -0.549)?
            .set_point(4, -1.31, -1.51)?
            .set_point(5, -0.532, -2.17)?
            .set_point(6, 0.454, -2.41)?
            .set_point(7, 1.45, -2.21)?
            .set_point(8, 2.29, -1.66)?
            .set_point(9, 2.88, -0.838)?
            .set_point(10, 3.16, 0.131)?
            .set_point(11, 3.12, 1.14)?
            .set_point(12, 2.77, 2.08)?
            .set_point(13, 2.16, 2.89)?
            .set_point(14, 1.36, 3.49)?;
        triangle.generate_delaunay(true)?;
        assert_eq!(triangle.get_npoint(), 15);
        assert_eq!(triangle.get_ntriangle(), 15);
        Ok(())
    }

    #[test]
    fn voronoi_1_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(15, None, None, None)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, -0.416, 0.909)?
            .set_point(2, -1.35, 0.436)?
            .set_point(3, -1.64, -0.549)?
            .set_point(4, -1.31, -1.51)?
            .set_point(5, -0.532, -2.17)?
            .set_point(6, 0.454, -2.41)?
            .set_point(7, 1.45, -2.21)?
            .set_point(8, 2.29, -1.66)?
            .set_point(9, 2.88, -0.838)?
            .set_point(10, 3.16, 0.131)?
            .set_point(11, 3.12, 1.14)?
            .set_point(12, 2.77, 2.08)?
            .set_point(13, 2.16, 2.89)?
            .set_point(14, 1.36, 3.49)?;
        triangle.generate_voronoi(true)?;
        assert_eq!(triangle.get_npoint(), 15);
        assert_eq!(triangle.get_ntriangle(), 15);
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
        assert_eq!(triangle.get_ncorner(), 3);
        assert_eq!(triangle.get_point_x(0), 0.0);
        assert_eq!(triangle.get_point_y(0), 0.0);
        assert_eq!(triangle.get_point_x(1), 1.0);
        assert_eq!(triangle.get_point_y(1), 0.0);
        assert_eq!(triangle.get_point_x(2), 0.0);
        assert_eq!(triangle.get_point_y(2), 1.0);
        assert_eq!(triangle.get_triangle_corner(0, 0), 0);
        assert_eq!(triangle.get_triangle_corner(0, 1), 1);
        assert_eq!(triangle.get_triangle_corner(0, 2), 2);
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
        assert_eq!(triangle.get_ncorner(), 6);
        Ok(())
    }
}
