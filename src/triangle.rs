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
    fn set_point(triangle: *mut ExtTriangle, index: i32, x: f64, y: f64);
    fn set_segment(triangle: *mut ExtTriangle, index: i32, left: i32, right: i32);
    fn set_region(
        triangle: *mut ExtTriangle,
        index: i32,
        x: f64,
        y: f64,
        attribute: i32,
        max_area: f64,
    );
    fn set_hole(triangle: *mut ExtTriangle, index: i32, x: f64, y: f64);
    fn generate(
        triangle: *mut ExtTriangle,
        quiet: i32,
        quadratic: i32,
        global_max_area: f64,
        global_min_angle: f64,
    );
    fn get_npoint(triangle: *mut ExtTriangle) -> i32;
    fn get_ntriangle(triangle: *mut ExtTriangle) -> i32;
    fn get_ncorner(triangle: *mut ExtTriangle) -> i32;
    fn get_point_x(triangle: *mut ExtTriangle, index: i32) -> f64;
    fn get_point_y(triangle: *mut ExtTriangle, index: i32) -> f64;
    fn get_triangle_corner(triangle: *mut ExtTriangle, index: i32, corner: i32) -> i32;
}

pub struct Triangle {
    ext_triangle: *mut ExtTriangle, // data allocated by the c-code
    npoint: usize,
    nsegment: usize,
    nregion: usize,
    nhole: usize,
    // all_points_set: bool,
    // all_segments_set: bool,
    // all_regions_set: bool,
    // all_holes_set: bool,
}

impl Triangle {
    pub fn new(
        npoint: usize,
        nsegment: usize,
        nregion: usize,
        nhole: usize,
    ) -> Result<Self, StrError> {
        if npoint < 3 {
            return Err("npoint must be ≥ 3");
        }
        if nsegment < 3 {
            return Err("nsegment must be ≥ 3");
        }
        unsafe {
            let ext_triangle = new_triangle(
                to_i32(npoint),
                to_i32(nsegment),
                to_i32(nregion),
                to_i32(nhole),
            );
            if ext_triangle.is_null() {
                return Err("c-code failed to allocate Triangle");
            }
            Ok(Triangle {
                ext_triangle,
                npoint,
                nsegment,
                nregion,
                nhole,
                // all_points_set: false,
                // all_segments_set: false,
                // all_regions_set: false,
                // all_holes_set: false,
            })
        }
    }

    pub fn set_point(&mut self, index: usize, x: f64, y: f64) -> Result<&mut Self, StrError> {
        if index >= self.npoint {
            return Err("index of point is out of bounds");
        }
        unsafe {
            set_point(self.ext_triangle, to_i32(index), x, y);
        }
        Ok(self)
    }

    pub fn set_segment(
        &mut self,
        index: usize,
        left: usize,
        right: usize,
    ) -> Result<&mut Self, StrError> {
        if index >= self.nsegment {
            return Err("index of segment is out of bounds");
        }
        unsafe {
            set_segment(
                self.ext_triangle,
                to_i32(index),
                to_i32(left),
                to_i32(right),
            );
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
        if index >= self.nregion {
            return Err("index of region is out of bounds");
        }
        unsafe {
            set_region(
                self.ext_triangle,
                to_i32(index),
                x,
                y,
                to_i32(attribute),
                max_area,
            );
        }
        Ok(self)
    }

    pub fn set_hole(&mut self, index: usize, x: f64, y: f64) -> Result<&mut Self, StrError> {
        if index >= self.nhole {
            return Err("index of hole is out of bounds");
        }
        unsafe {
            set_hole(self.ext_triangle, to_i32(index), x, y);
        }
        Ok(self)
    }

    pub fn generate(
        &mut self,
        quiet: bool,
        quadratic: bool,
        _global_max_area: Option<f64>,
        _global_min_angle: Option<f64>,
    ) {
        unsafe {
            generate(
                self.ext_triangle,
                if quiet { 1 } else { 0 },
                if quadratic { 1 } else { 0 },
                0.0, // global_max_area,
                0.0, // global_min_angle,
            )
        }
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
        let triangle = Triangle::new(3, 3, 0, 0)?;
        assert_eq!(triangle.ext_triangle.is_null(), false);
        Ok(())
    }

    #[test]
    fn generate_works() -> Result<(), StrError> {
        let mut triangle = Triangle::new(3, 3, 0, 0)?;
        triangle
            .set_point(0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0)?
            .set_point(2, 0.0, 1.0)?;
        triangle
            .set_segment(0, 0, 1)?
            .set_segment(1, 1, 2)?
            .set_segment(2, 2, 0)?;
        triangle.generate(false, false, None, None);
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
}
