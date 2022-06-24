#![allow(unused)]

use crate::to_i32::to_i32;
use crate::StrError;

#[repr(C)]
pub(crate) struct ExtTetgen {
    data: [u8; 0],
    marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn new_tetgen(npoint: i32, nsegment: i32, nregion: i32, nhole: i32) -> *mut ExtTetgen;
    fn drop_tetgen(tetgen: *mut ExtTetgen);
}

pub struct Tetgen {
    ext_tetgen: *mut ExtTetgen, // data allocated by the c-code
    npoint: usize,              // number of points
    nsegment: Option<usize>,    // number of segments
    nregion: Option<usize>,     // number of regions
    nhole: Option<usize>,       // number of holes
    all_points_set: bool,       // indicates that all points have been set
    all_segments_set: bool,     // indicates that all segments have been set
    all_regions_set: bool,      // indicates that all regions have been set
    all_holes_set: bool,        // indicates that all holes have been set
}

impl Drop for Tetgen {
    /// Tells the c-code to release memory
    fn drop(&mut self) {
        unsafe {
            drop_tetgen(self.ext_tetgen);
        }
    }
}

impl Tetgen {
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
            let ext_tetgen = new_tetgen(npoint_i32, nsegment_i32, nregion_i32, nhole_i32);
            if ext_tetgen.is_null() {
                return Err("INTERNAL ERROR: cannot allocate ExtTetgen");
            }
            Ok(Tetgen {
                ext_tetgen,
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Tetgen;
    use crate::StrError;

    #[test]
    fn new_works() -> Result<(), StrError> {
        let tetgen = Tetgen::new(3, Some(3), None, None)?;
        assert_eq!(tetgen.ext_tetgen.is_null(), false);
        assert_eq!(tetgen.npoint, 3);
        assert_eq!(tetgen.nsegment, Some(3));
        assert_eq!(tetgen.nregion, None);
        assert_eq!(tetgen.nhole, None);
        assert_eq!(tetgen.all_points_set, false);
        assert_eq!(tetgen.all_segments_set, false);
        assert_eq!(tetgen.all_regions_set, false);
        assert_eq!(tetgen.all_holes_set, false);
        Ok(())
    }
}
