use crate::constants;
use crate::StrError;
use crate::Tetgen;
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Write as IoWrite;
use std::path::Path;

impl Tetgen {
    /// Writes a VTU file to visualize the mesh with Paraview
    ///
    /// # Input
    ///
    /// * `full_path` -- may be a String, &str, or Path
    pub fn write_vtu<P>(&self, full_path: &P) -> Result<(), StrError>
    where
        P: AsRef<OsStr> + ?Sized,
    {
        let ntet = self.ntet();
        if ntet < 1 {
            return Err("there are no tetrahedra to write");
        }

        let npoint = self.npoint();
        let nnode = self.nnode();
        let vtk_type = if nnode == 4 {
            constants::VTK_TETRA
        } else {
            constants::VTK_QUADRATIC_TETRA
        };

        let mut buffer = String::new();

        // header
        write!(
            &mut buffer,
            "<?xml version=\"1.0\"?>\n\
         <VTKFile type=\"UnstructuredGrid\" version=\"0.1\" byte_order=\"LittleEndian\">\n\
         <UnstructuredGrid>\n\
         <Piece NumberOfPoints=\"{}\" NumberOfCells=\"{}\">\n",
            npoint, ntet
        )
        .unwrap();

        // nodes: coordinates
        write!(
            &mut buffer,
            "<Points>\n\
         <DataArray type=\"Float64\" NumberOfComponents=\"3\" format=\"ascii\">\n"
        )
        .unwrap();
        for index in 0..npoint {
            for dim in 0..3 {
                write!(&mut buffer, "{} ", self.point(index, dim)).unwrap();
            }
        }
        write!(
            &mut buffer,
            "\n</DataArray>\n\
         </Points>\n"
        )
        .unwrap();

        // elements: connectivity
        write!(
            &mut buffer,
            "<Cells>\n\
         <DataArray type=\"Int32\" Name=\"connectivity\" format=\"ascii\">\n"
        )
        .unwrap();
        for index in 0..ntet {
            for m in 0..nnode {
                write!(&mut buffer, "{} ", self.tet_node(index, m)).unwrap();
            }
        }

        // elements: offsets
        write!(
            &mut buffer,
            "\n</DataArray>\n\
         <DataArray type=\"Int32\" Name=\"offsets\" format=\"ascii\">\n"
        )
        .unwrap();
        let mut offset = 0;
        for _ in 0..ntet {
            offset += nnode;
            write!(&mut buffer, "{} ", offset).unwrap();
        }

        // elements: types
        write!(
            &mut buffer,
            "\n</DataArray>\n\
         <DataArray type=\"UInt8\" Name=\"types\" format=\"ascii\">\n"
        )
        .unwrap();
        for _ in 0..ntet {
            write!(&mut buffer, "{} ", vtk_type).unwrap();
        }
        write!(
            &mut buffer,
            "\n</DataArray>\n\
         </Cells>\n"
        )
        .unwrap();

        write!(
            &mut buffer,
            "</Piece>\n\
         </UnstructuredGrid>\n\
         </VTKFile>\n"
        )
        .unwrap();

        // create directory
        let path = Path::new(full_path);
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).map_err(|_| "cannot create directory")?;
        }

        // write file
        let mut file = File::create(path).map_err(|_| "cannot create file")?;
        file.write_all(buffer.as_bytes()).map_err(|_| "cannot write file")?;

        // force sync
        file.sync_all().map_err(|_| "cannot sync file")?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::StrError;
    use crate::Tetgen;
    use std::fs;

    #[test]
    fn tetgen_write_vtu() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(4, None, None, None)?;
        tetgen
            .set_point(0, 0.0, 0.0, 0.0)?
            .set_point(1, 1.0, 0.0, 0.0)?
            .set_point(2, 0.0, 1.0, 0.0)?
            .set_point(3, 0.0, 0.0, 1.0)?;
        tetgen.generate_delaunay(false)?;
        let file_path = "/tmp/tritet/test_tetgen_write_vtu.vtu";
        tetgen.write_vtu(file_path)?;
        let contents = fs::read_to_string(file_path).map_err(|_| "cannot open file")?;
        assert_eq!(
            contents,
            r#"<?xml version="1.0"?>
<VTKFile type="UnstructuredGrid" version="0.1" byte_order="LittleEndian">
<UnstructuredGrid>
<Piece NumberOfPoints="4" NumberOfCells="1">
<Points>
<DataArray type="Float64" NumberOfComponents="3" format="ascii">
0 0 0 1 0 0 0 1 0 0 0 1 
</DataArray>
</Points>
<Cells>
<DataArray type="Int32" Name="connectivity" format="ascii">
1 0 3 2 
</DataArray>
<DataArray type="Int32" Name="offsets" format="ascii">
4 
</DataArray>
<DataArray type="UInt8" Name="types" format="ascii">
10 
</DataArray>
</Cells>
</Piece>
</UnstructuredGrid>
</VTKFile>
"#
        );
        Ok(())
    }
}
