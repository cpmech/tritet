use crate::StrError;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

/// Holds the input data for tetrahedral mesh generation using Tetgen.
///
/// The input of Tetgen is a Piecewise Linear Complex (PLC)
///
/// # Examples
///
/// ```
/// # use tritet::InputDataTetMesh;
/// let data = InputDataTetMesh {
///     points: vec![
///         (0, 0.0, 1.0, 0.0),
///         (0, 0.0, 0.0, 0.0),
///         (0, 1.0, 1.0, 0.0),
///         (0, 0.0, 1.0, 1.0),
///     ],
///     facets: vec![
///         (0, vec![0, 2, 1]),
///         (0, vec![0, 1, 3]),
///         (0, vec![0, 3, 2]),
///         (0, vec![1, 2, 3]),
///     ],
///     holes: vec![],
///     regions: vec![(1, 0.1, 0.9, 0.1, None)],
/// };
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputDataTetMesh {
    /// List of points
    ///
    /// Each point is defined by a boundary marker and its x, y, and z coordinates.
    pub points: Vec<(i32, f64, f64, f64)>,

    /// List of facets
    ///
    /// Each facet is defined by a boundary marker and a list of point indices that form the facet.
    pub facets: Vec<(i32, Vec<usize>)>,

    /// List of holes
    ///
    /// Each hole is defined by a coordinate located inside the hole.
    pub holes: Vec<(f64, f64, f64)>,

    /// List of regions
    ///
    /// Each region is defined by a region marker, the x, y, and z coordinates located inside the region,
    /// and an optional maximum volume constraint for the tetrahedra in that region.
    pub regions: Vec<(i32, f64, f64, f64, Option<f64>)>,
}

impl InputDataTetMesh {
    /// Reads a JSON file containing the data
    ///
    /// # Input
    ///
    /// * `full_path` -- may be a String, &str, or Path
    pub fn read_json<P>(full_path: &P) -> Result<Self, StrError>
    where
        P: AsRef<OsStr> + ?Sized,
    {
        let path = Path::new(full_path).to_path_buf();
        let input = File::open(path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(input);
        let mesh = serde_json::from_reader(buffered).map_err(|_| "cannot parse JSON file")?;
        Ok(mesh)
    }

    /// Writes a JSON file with the data
    ///
    /// # Input
    ///
    /// * `full_path` -- may be a String, &str, or Path
    pub fn write_json<P>(&self, full_path: &P) -> Result<(), StrError>
    where
        P: AsRef<OsStr> + ?Sized,
    {
        let path = Path::new(full_path).to_path_buf();
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).map_err(|_| "cannot create directory")?;
        }
        let mut file = File::create(&path).map_err(|_| "cannot create file")?;
        serde_json::to_writer(&mut file, &self).map_err(|_| "cannot write file")?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::InputDataTetMesh;

    #[test]
    fn read_and_write_tet_capture_errors() {
        assert_eq!(
            InputDataTetMesh::read_json("/tmp/not_found").err(),
            Some("cannot open file")
        );
        assert_eq!(
            InputDataTetMesh::read_json("./data/input/wrong_input_tetgen.json").err(),
            Some("cannot parse JSON file")
        );
        let data = InputDataTetMesh {
            points: vec![],
            facets: vec![],
            holes: vec![],
            regions: vec![],
        };
        assert_eq!(data.write_json("/tmp/").err(), Some("cannot create file"));
    }

    #[test]
    fn read_and_write_tet_json_work() {
        let data = InputDataTetMesh {
            points: vec![
                (0, 0.0, 1.0, 0.0),
                (0, 0.0, 0.0, 0.0),
                (0, 1.0, 1.0, 0.0),
                (0, 0.0, 1.0, 1.0),
            ],
            facets: vec![
                (0, vec![0, 2, 1]),
                (0, vec![0, 1, 3]),
                (0, vec![0, 3, 2]),
                (0, vec![1, 2, 3]),
            ],
            holes: vec![],
            regions: vec![(1, 0.1, 0.9, 0.1, None)],
        };
        data.write_json("/tmp/tritet/test_read_and_write_tet_work.json")
            .unwrap();
        let data_read = InputDataTetMesh::read_json("/tmp/tritet/test_read_and_write_tet_work.json").unwrap();
        assert_eq!(data.points, data_read.points);
        assert_eq!(data.facets, data_read.facets);
        assert_eq!(data.holes, data_read.holes);
        assert_eq!(data.regions, data_read.regions);
    }
}
