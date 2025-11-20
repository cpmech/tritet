use crate::StrError;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

/// Holds the input data for triangle mesh generation using Trigen.
///
/// The input of Trigen (Triangle) is a Planar Straight Line Graph (PSLG).
/// See the definitions #[derive(Clone, Debug, Deserialize, Serialize)]in the README file.
///
/// # Examples
///
/// ```
/// # use tritet::InputDataTriMesh;
/// let data = InputDataTriMesh {
///     points: vec![
///         (0, 0.0, 0.0),
///         (0, 1.0, 0.0),
///         (0, 1.0, 1.0),
///         (0, 0.0, 1.0),
///         (0, 0.2, 0.2),
///         (0, 0.8, 0.2),
///         (0, 0.8, 0.8),
///         (0, 0.2, 0.8),
///         (0, 0.0, 0.5),
///         (0, 0.2, 0.5),
///         (0, 0.8, 0.5),
///         (0, 1.0, 0.5),
///     ],
///     segments: vec![
///         (-1, 0, 1),
///         (-1, 1, 2),
///         (-1, 2, 3),
///         (-1, 3, 0),
///         (-1, 4, 5),
///         (-1, 5, 6),
///         (-1, 6, 7),
///         (-1, 7, 4),
///         (-1, 8, 9),
///         (-1, 10, 11),
///     ],
///     holes: vec![(0.5, 0.5)],
///     regions: vec![(1, 0.1, 0.1, None), (2, 0.1, 0.9, None)],
/// };
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputDataTriMesh {
    /// List of points
    ///
    /// Each point is defined by a boundary marker and its x and y coordinates.
    pub points: Vec<(i32, f64, f64)>,

    /// List of segments
    ///
    /// Each segment is defined by a boundary marker and the IDs of its two endpoints.
    pub segments: Vec<(i32, usize, usize)>,

    /// List of holes
    ///
    /// Each hole is defined by a coordinate located inside the hole.
    pub holes: Vec<(f64, f64)>,

    /// List of regions
    ///
    /// Each region is defined by a region marker, the x and y coordinates located inside the region,
    /// and an optional maximum area constraint for the triangles in that region.
    pub regions: Vec<(i32, f64, f64, Option<f64>)>,
}

impl InputDataTriMesh {
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
    use super::InputDataTriMesh;

    #[test]
    fn read_and_write_tri_capture_errors() {
        assert_eq!(
            InputDataTriMesh::read_json("/tmp/not_found").err(),
            Some("cannot open file")
        );
        assert_eq!(
            InputDataTriMesh::read_json("./data/input/wrong_input_trigen.json").err(),
            Some("cannot parse JSON file")
        );
        let data = InputDataTriMesh {
            points: vec![],
            segments: vec![],
            holes: vec![],
            regions: vec![],
        };
        assert_eq!(data.write_json("/tmp/").err(), Some("cannot create file"));
    }

    #[test]
    fn read_and_write_tri_json_work() {
        let data = InputDataTriMesh {
            points: vec![
                (0, 0.0, 0.0),
                (0, 1.0, 0.0),
                (0, 1.0, 1.0),
                (0, 0.0, 1.0),
                (0, 0.2, 0.2),
                (0, 0.8, 0.2),
                (0, 0.8, 0.8),
                (0, 0.2, 0.8),
                (0, 0.0, 0.5),
                (0, 0.2, 0.5),
                (0, 0.8, 0.5),
                (0, 1.0, 0.5),
            ],
            segments: vec![
                (-1, 0, 1),
                (-1, 1, 2),
                (-1, 2, 3),
                (-1, 3, 0),
                (-1, 4, 5),
                (-1, 5, 6),
                (-1, 6, 7),
                (-1, 7, 4),
                (-1, 8, 9),
                (-1, 10, 11),
            ],
            holes: vec![(0.5, 0.5)],
            regions: vec![(1, 0.1, 0.1, None), (2, 0.1, 0.9, None)],
        };
        data.write_json("/tmp/tritet/test_read_and_write_work.json").unwrap();
        let data_read = InputDataTriMesh::read_json("/tmp/tritet/test_read_and_write_work.json").unwrap();
        assert_eq!(data.points, data_read.points);
        assert_eq!(data.segments, data_read.segments);
        assert_eq!(data.holes, data_read.holes);
        assert_eq!(data.regions, data_read.regions);
    }
}
