#[repr(C)]
pub(crate) struct ExtTriangle {
    data: [u8; 0],
    marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    // Triangle
    fn new_triangle() -> *mut ExtTriangle;
    fn drop_triangle(triangle: *mut ExtTriangle);
}

/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

pub struct Triangle {
    ext_triangle: *mut ExtTriangle, // data allocated by the c-code
}

impl Triangle {
    pub fn new() -> Result<Self, StrError> {
        unsafe {
            let ext_triangle = new_triangle();
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
        let triangle = Triangle::new()?;
        assert_eq!(triangle.ext_triangle.is_null(), false);
        Ok(())
    }
}
