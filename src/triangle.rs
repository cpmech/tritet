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
}

pub struct Triangle {
    ext_triangle: *mut ExtTriangle, // data allocated by the c-code
}

impl Triangle {
    pub fn new(
        npoint: usize,
        nsegment: usize,
        nregion: usize,
        nhole: usize,
    ) -> Result<Self, StrError> {
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
            Ok(Triangle { ext_triangle })
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
        let triangle = Triangle::new(3, 3, 0, 0)?;
        assert_eq!(triangle.ext_triangle.is_null(), false);
        Ok(())
    }
}
