use crate::constants;
use crate::StrError;
use crate::Trigen;
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Write as IoWrite;
use std::path::Path;

impl Trigen {
    /// Writes a VTU file to visualize the mesh with Paraview
    ///
    /// # Input
    ///
    /// * `full_path` -- may be a String, &str, or Path
    pub fn write_vtu<P>(&self, full_path: &P) -> Result<(), StrError>
    where
        P: AsRef<OsStr> + ?Sized,
    {
        let ntriangle = self.out_ncell();
        if ntriangle < 1 {
            return Err("there are no triangles to write");
        }

        let npoint = self.out_npoint();
        let nnode = self.out_cell_npoint();
        let vtk_type = if nnode == 3 {
            constants::VTK_TRIANGLE
        } else {
            constants::VTK_QUADRATIC_TRIANGLE
        };

        let mut buffer = String::new();

        // header
        write!(
            &mut buffer,
            "<?xml version=\"1.0\"?>\n\
         <VTKFile type=\"UnstructuredGrid\" version=\"0.1\" byte_order=\"LittleEndian\">\n\
         <UnstructuredGrid>\n\
         <Piece NumberOfPoints=\"{}\" NumberOfCells=\"{}\">\n",
            npoint, ntriangle
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
            write!(
                &mut buffer,
                "{:?} {:?} 0.0 ",
                self.out_point(index, 0),
                self.out_point(index, 1)
            )
            .unwrap();
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
        for index in 0..ntriangle {
            for m in 0..nnode {
                write!(&mut buffer, "{} ", self.out_cell_point(index, m)).unwrap();
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
        for _ in 0..ntriangle {
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
        for _ in 0..ntriangle {
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
    use crate::Trigen;
    use std::fs;

    #[test]
    fn trigen_write_vtu() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, None, None, None)?;
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0)?;
        trigen.generate_delaunay(false)?;
        let file_path = "/tmp/tritet/test_trigen_write_vtu.vtu";
        trigen.write_vtu(file_path)?;
        let contents = fs::read_to_string(file_path).map_err(|_| "cannot open file")?;
        assert_eq!(
            contents,
            r#"<?xml version="1.0"?>
<VTKFile type="UnstructuredGrid" version="0.1" byte_order="LittleEndian">
<UnstructuredGrid>
<Piece NumberOfPoints="3" NumberOfCells="1">
<Points>
<DataArray type="Float64" NumberOfComponents="3" format="ascii">
0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 0.0 
</DataArray>
</Points>
<Cells>
<DataArray type="Int32" Name="connectivity" format="ascii">
0 1 2 
</DataArray>
<DataArray type="Int32" Name="offsets" format="ascii">
3 
</DataArray>
<DataArray type="UInt8" Name="types" format="ascii">
5 
</DataArray>
</Cells>
</Piece>
</UnstructuredGrid>
</VTKFile>
"#
        );
        Ok(())
    }

    #[test]
    fn trigen_write_vtu_o2() -> Result<(), StrError> {
        let mut trigen = Trigen::new(3, Some(3), None, None)?;
        trigen
            .set_point(0, 0, 0.0, 0.0)?
            .set_point(1, 0, 1.0, 0.0)?
            .set_point(2, 0, 0.0, 1.0)?;
        trigen
            .set_segment(0, -10, 0, 1)?
            .set_segment(1, -20, 1, 2)?
            .set_segment(2, -30, 2, 0)?;
        trigen.generate_mesh(false, true, false, None, None)?;
        let file_path = "/tmp/tritet/test_trigen_write_vtu_o2.vtu";
        trigen.write_vtu(file_path)?;
        let contents = fs::read_to_string(file_path).map_err(|_| "cannot open file")?;
        assert_eq!(
            contents,
            r#"<?xml version="1.0"?>
<VTKFile type="UnstructuredGrid" version="0.1" byte_order="LittleEndian">
<UnstructuredGrid>
<Piece NumberOfPoints="6" NumberOfCells="1">
<Points>
<DataArray type="Float64" NumberOfComponents="3" format="ascii">
0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 0.0 0.5 0.0 0.0 0.5 0.5 0.0 0.0 0.5 0.0 
</DataArray>
</Points>
<Cells>
<DataArray type="Int32" Name="connectivity" format="ascii">
0 1 2 3 4 5 
</DataArray>
<DataArray type="Int32" Name="offsets" format="ascii">
6 
</DataArray>
<DataArray type="UInt8" Name="types" format="ascii">
22 
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
