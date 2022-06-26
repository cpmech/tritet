#![allow(unused)]

use crate::constants;
use crate::conversion::to_i32;
use crate::global;
use crate::StrError;

extern "C" {
    fn new_tetgen(
        handle: i32,
        npoint: i32,
        nfacet: i32,
        facet_npoint: *const i32,
        nregion: i32,
        nhole: i32,
    ) -> i32;
    fn drop_tetgen(handle: i32);
    fn tet_set_point(handle: i32, index: i32, x: f64, y: f64, z: f64) -> i32;
    fn tet_set_facet_point(handle: i32, index: i32, m: i32, p: i32) -> i32;
    fn tet_set_region(
        handle: i32,
        index: i32,
        x: f64,
        y: f64,
        z: f64,
        attribute: i32,
        max_volume: f64,
    ) -> i32;
    fn tet_set_hole(handle: i32, index: i32, x: f64, y: f64, z: f64) -> i32;
    fn tet_run_delaunay(handle: i32, verbose: i32) -> i32;
    fn tet_run_tetrahedralize(
        handle: i32,
        verbose: i32,
        o2: i32,
        global_max_volume: f64,
        global_min_angle: f64,
    ) -> i32;
    fn tet_get_npoint(handle: i32) -> i32;
    fn tet_get_ntetrahedron(handle: i32) -> i32;
    fn tet_get_ncorner(handle: i32) -> i32;
    fn tet_get_point(handle: i32, index: i32, dim: i32) -> f64;
    fn tet_get_tetrahedron_corner(handle: i32, index: i32, corner: i32) -> i32;
    fn tet_get_tetrahedron_attribute(handle: i32, index: i32) -> i32;
}

/// Implements high-level functions to call Si's Tetgen Cpp-Code
///
/// **Note:** All indices are are zero-based.
pub struct Tetgen {
    handle: i32,                      // handle to c-data
    npoint: usize,                    // number of points
    facet_npoint: Option<Vec<usize>>, // number of points on each facet
    nregion: Option<usize>,           // number of regions
    nhole: Option<usize>,             // number of holes
    all_points_set: bool,             // indicates that all points have been set
    all_facets_set: bool,             // indicates that all facets have been set
    all_regions_set: bool,            // indicates that all regions have been set
    all_holes_set: bool,              // indicates that all holes have been set
    total_facet_npoint: usize,        // total number of facet points
    facet_npoint_set: usize,          // number of facet points set already
}

impl Drop for Tetgen {
    /// Tells the c-code to release memory
    fn drop(&mut self) {
        unsafe {
            let _ = global::ACCESS_C_CODE.lock().unwrap();
            drop_tetgen(self.handle);
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
        let fnp = facet_npoint.clone();
        let npoint_i32: i32 = to_i32(npoint);
        let facet_npoint_i32: Vec<i32> = match facet_npoint {
            Some(v) => {
                // if v.len() < 4 {
                //     return Err("nfacet must be ≥ 4");
                // }
                v.into_iter().map(|n| to_i32(n)).collect()
            }

            None => Vec::new(),
        };
        let nregion_i32: i32 = match nregion {
            Some(v) => to_i32(v),
            None => 0,
        };
        let nhole_i32: i32 = match nhole {
            Some(v) => to_i32(v),
            None => 0,
        };
        let handle = to_i32(global::generate_handle());
        unsafe {
            let _ = global::ACCESS_C_CODE
                .lock()
                .map_err(|_| "INTERNAL ERROR: cannot lock access to c-code")?;
            let status = new_tetgen(
                handle,
                npoint_i32,
                to_i32(facet_npoint_i32.len()),
                facet_npoint_i32.as_ptr(),
                nregion_i32,
                nhole_i32,
            );
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_INITIALIZE_FAILED {
                    return Err("INTERNAL ERROR: cannot initialize c-data");
                }
                if status == constants::TRITET_ERROR_ALLOC_POINT_LIST_FAILED {
                    return Err("INTERNAL ERROR: cannot allocate point list");
                }
                if status == constants::TRITET_ERROR_ALLOC_FACET_LIST_FAILED {
                    return Err("INTERNAL ERROR: cannot allocate facet list");
                }
                if status == constants::TRITET_ERROR_ALLOC_FACET_DATA_FAILED {
                    return Err("INTERNAL ERROR: cannot allocate facet data");
                }
                if status == constants::TRITET_ERROR_ALLOC_REGION_LIST_FAILED {
                    return Err("INTERNAL ERROR: cannot allocate region list");
                }
                if status == constants::TRITET_ERROR_ALLOC_HOLE_LIST_FAILED {
                    return Err("INTERNAL ERROR: cannot allocate hole list");
                }
            }
        };
        Ok(Tetgen {
            handle,
            npoint,
            facet_npoint: fnp,
            nregion,
            nhole,
            all_points_set: false,
            all_facets_set: false,
            all_regions_set: false,
            all_holes_set: false,
            total_facet_npoint: 0,
            facet_npoint_set: 0,
        })
    }

    /// Sets the point coordinates
    pub fn set_point(
        &mut self,
        index: usize,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<&mut Self, StrError> {
        unsafe {
            let status = tet_set_point(self.handle, to_i32(index), x, y, z);
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

    /// Sets the facet's point IDs
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the facet and goes from 0 to `nfacet` (passed down to `new`)
    /// * `m` -- is the local index of the point on the facet and goes from 0 to `facet_npoint`
    /// * `p` -- is the ID (index) of the point on the facet
    pub fn set_facet_point(
        &mut self,
        index: usize,
        m: usize,
        p: usize,
    ) -> Result<&mut Self, StrError> {
        let facet_npoint = match &self.facet_npoint {
            Some(n) => n,
            None => return Err("cannot set facet point because facet_npoint is None"),
        };
        unsafe {
            let status = tet_set_facet_point(self.handle, to_i32(index), to_i32(m), to_i32(p));
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_FACET_LIST {
                    return Err("INTERNAL ERROR: found NULL facet list");
                }
                if status == constants::TRITET_ERROR_INVALID_FACET_INDEX {
                    return Err("index of facet is out of bounds");
                }
                if status == constants::TRITET_ERROR_NULL_FACET_POLYGON_LIST {
                    return Err("INTERNAL ERROR: found NULL facet polygon list");
                }
                if status == constants::TRITET_ERROR_INVALID_FACET_NUM_POLYGON {
                    return Err("INTERNAL ERROR: found invalid facet number of polygon");
                }
                if status == constants::TRITET_ERROR_INVALID_FACET_POINT_INDEX {
                    return Err("index of facet point is out of bounds");
                }
                if status == constants::TRITET_ERROR_INVALID_FACET_POINT_ID {
                    return Err("id of facet point is out of bounds");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        // if index == nfacet - 1 {
        //     self.all_facets_set = true;
        //     if self.facet_npoint_set != self.total_facet_npoint {
        //         self.facet_npoint_set += 1;
        //     }
        // } else {
        //     self.all_facets_set = false;
        //     self.facet_npoint_set += 1;
        // }
        Ok(self)
    }

    /// Marks a region within the Piecewise Linear Complexes (PLCs)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the region and goes from 0 to `nregion` (passed down to `new`)
    /// * `x` -- is the x-coordinate of the region
    /// * `y` -- is the y-coordinate of the region
    /// * `z` -- is the z-coordinate of the region
    /// * `attribute` -- is the attribute ID to group the tetrahedra belonging to this region
    /// * `max_volume` -- is the maximum volume constraint for the tetrahedra belonging to this region
    pub fn set_region(
        &mut self,
        index: usize,
        x: f64,
        y: f64,
        z: f64,
        attribute: usize,
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
                self.handle,
                to_i32(index),
                x,
                y,
                z,
                to_i32(attribute),
                volume_constraint,
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

    /// Marks a hole within the Piecewise Linear Complexes (PLCs)
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the hole and goes from 0 to `nhole` (passed down to `new`)
    /// * `x` -- is the x-coordinate of the hole
    /// * `y` -- is the y-coordinate of the hole
    /// * `z` -- is the z-coordinate of the hole
    pub fn set_hole(
        &mut self,
        index: usize,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<&mut Self, StrError> {
        let nhole = match self.nhole {
            Some(n) => n,
            None => return Err("cannot set hole because the number of holes is None"),
        };
        unsafe {
            let status = tet_set_hole(self.handle, to_i32(index), x, y, z);
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
    /// * `verbose` -- Prints Tetgen's messages to the console
    pub fn generate_delaunay(&self, verbose: bool) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err(
                "cannot generate Delaunay tetrahedralization because not all points are set",
            );
        }
        unsafe {
            let status = tet_run_delaunay(self.handle, if verbose { 1 } else { 0 });
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
    /// * `verbose` -- Prints Tetgen's messages to the console
    /// * `o2` -- Generates the middle nodes; e.g., nnode = 10
    /// * `global_max_volume` -- The maximum volume constraint for all generated tetrahedra
    /// * `global_min_angle` -- The minimum angle constraint is given in degrees (the default minimum angle is TODO degrees)
    pub fn generate_mesh(
        &self,
        verbose: bool,
        o2: bool,
        global_volume_area: Option<f64>,
        global_min_angle: Option<f64>,
    ) -> Result<(), StrError> {
        if !self.all_points_set {
            return Err("cannot generate mesh of tetrahedra because not all points are set");
        }
        if !self.all_facets_set {
            return Err("cannot generate mesh of tetrahedra because not all facets are set");
        }
        let max_volume = match global_volume_area {
            Some(v) => v,
            None => 0.0,
        };
        let min_angle = match global_min_angle {
            Some(v) => v,
            None => 0.0,
        };
        unsafe {
            let status = tet_run_tetrahedralize(
                self.handle,
                if verbose { 1 } else { 0 },
                if o2 { 1 } else { 0 },
                max_volume,
                min_angle,
            );
            if status != constants::TRITET_SUCCESS {
                if status == constants::TRITET_ERROR_NULL_DATA {
                    return Err("INTERNAL ERROR: found NULL data");
                }
                if status == constants::TRITET_ERROR_NULL_POINT_LIST {
                    return Err("INTERNAL ERROR: found NULL point list");
                }
                if status == constants::TRITET_ERROR_NULL_FACET_LIST {
                    return Err("INTERNAL ERROR: list of facets must be defined first");
                }
                if status == constants::TRITET_ERROR_STRING_CONCAT {
                    return Err("INTERNAL ERROR: cannot write string with commands for Tetgen");
                }
                return Err("INTERNAL ERROR: some error occurred");
            }
        }
        Ok(())
    }

    /// Returns the number of points of the Delaunay triangulation (constrained or not)
    pub fn npoint(&self) -> usize {
        unsafe { tet_get_npoint(self.handle) as usize }
    }

    /// Returns the number of tetrahedra on the Delaunay triangulation (constrained or not)
    pub fn ntetrahedron(&self) -> usize {
        unsafe { tet_get_ntetrahedron(self.handle) as usize }
    }

    /// Returns the number of nodes on a tetrahedron (e.g., 4 or 10)
    pub fn nnode(&self) -> usize {
        unsafe { tet_get_ncorner(self.handle) as usize }
    }

    /// Returns the x-y-z coordinates of a point
    ///
    /// # Input
    ///
    /// * `index` -- is the index of the point and goes from 0 to `npoint`
    /// * `dim` -- is the space dimension index: 0, 1, or 2
    ///
    /// # Warning
    ///
    /// This function will return 0.0 if either `index` or `dim` are out of range.
    pub fn point(&self, index: usize, dim: usize) -> f64 {
        unsafe { tet_get_point(self.handle, to_i32(index), to_i32(dim)) }
    }

    /// Returns the ID of a tetrahedron's node
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
    /// * `index` -- is the index of the tetrahedron and goes from 0 to `ntetrahedron`
    /// * `m` -- is the local index of the node and goes from 0 to `nnode`
    ///
    /// # Warning
    ///
    /// This function will return 0 if either `index` or `m` are out of range.
    pub fn tetgen_node(&self, index: usize, m: usize) -> usize {
        unsafe {
            let corner = constants::TRITET_TO_TETGEN[m];
            tet_get_tetrahedron_corner(self.handle, to_i32(index), to_i32(corner)) as usize
        }
    }

    /// Returns the attribute ID of a tetgen
    ///
    /// # Warning
    ///
    /// This function will return 0 if either `index` is out of range.
    pub fn tetgen_attribute(&self, index: usize) -> usize {
        unsafe { tet_get_tetrahedron_attribute(self.handle, to_i32(index)) as usize }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Tetgen;
    use crate::StrError;

    #[test]
    fn new_captures_some_errors() {
        assert_eq!(
            Tetgen::new(3, None, None, None).err(),
            Some("npoint must be ≥ 4")
        );
        // assert_eq!(
        //     Tetgen::new(4, Some(3), None, None).err(),
        //     Some("nfacet must be ≥ 4")
        // );
    }

    #[test]
    fn new_works() -> Result<(), StrError> {
        let tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), None, None)?;
        assert!(tetgen.handle > 0);
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
            tetgen.set_point(5, 0.0, 0.0, 0.0).err(),
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
            tetgen.set_region(0, 0.33, 0.33, 0.33, 1, Some(0.1)).err(),
            Some("cannot set region because the number of regions is None")
        );
        let mut tetgen = Tetgen::new(4, Some(vec![3, 3, 3, 3]), Some(1), None)?;
        assert_eq!(
            tetgen.set_region(1, 0.33, 0.33, 0.33, 1, Some(0.1)).err(),
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
            .set_point(0, 0.0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0, 0.0)?
            .set_point(2, 0.0, 1.0, 0.0)?
            .set_point(3, 0.0, 0.0, 1.0)?;
        assert_eq!(
            tetgen.generate_mesh(false, false, None, None).err(),
            Some("cannot generate mesh of tetrahedra because not all facets are set")
        );
        Ok(())
    }
}
