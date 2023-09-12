use crate::constants;
use crate::constants::VTK_TRIANGLE;
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
        let ntet = self.out_ncell();
        if ntet < 1 {
            return Err("there are no tetrahedra to write");
        }

        let n_marked_faces = self.out_n_marked_face();
        let ncell = ntet + n_marked_faces;

        let npoint = self.out_npoint();
        let nnode = self.out_cell_npoint();
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
            npoint, ncell
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
                "{:?} {:?} {:?} ",
                self.out_point(index, 0),
                self.out_point(index, 1),
                self.out_point(index, 2)
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
        for index in 0..ntet {
            for m in 0..nnode {
                write!(&mut buffer, "{} ", self.out_cell_point(index, m)).unwrap();
            }
        }
        for index in 0..n_marked_faces {
            let (a, b, c, _, _) = self.out_marked_face(index);
            write!(&mut buffer, "{} {} {} ", a, b, c).unwrap();
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
        for _ in 0..n_marked_faces {
            offset += 3;
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
        for _ in 0..n_marked_faces {
            write!(&mut buffer, "{} ", VTK_TRIANGLE).unwrap();
        }

        // close Cells
        write!(
            &mut buffer,
            "\n</DataArray>\n\
         </Cells>\n"
        )
        .unwrap();

        // data: marked faces

        // data -- points
        write!(&mut buffer, "<PointData Scalars=\"TheScalars\">\n").unwrap();
        write!(
            &mut buffer,
            "<DataArray type=\"Int32\" Name=\"marker\" NumberOfComponents=\"1\" format=\"ascii\">\n"
        )
        .unwrap();
        for index in 0..npoint {
            let marker = self.out_point_marker(index);
            write!(&mut buffer, "{} ", marker).unwrap();
        }
        write!(&mut buffer, "\n</DataArray>\n").unwrap();
        write!(&mut buffer, "</PointData>\n").unwrap();

        // data -- cells
        write!(&mut buffer, "<CellData Scalars=\"TheScalars\">\n").unwrap();
        write!(
            &mut buffer,
            "<DataArray type=\"Int32\" Name=\"attribute\" NumberOfComponents=\"1\" format=\"ascii\">\n"
        )
        .unwrap();
        for index in 0..ntet {
            let attribute = self.out_cell_attribute(index);
            write!(&mut buffer, "{} ", attribute).unwrap();
        }
        for index in 0..n_marked_faces {
            let (_, _, _, marker, _) = self.out_marked_face(index);
            write!(&mut buffer, "{} ", marker).unwrap();
        }
        write!(&mut buffer, "\n</DataArray>\n").unwrap();
        write!(&mut buffer, "</CellData>\n").unwrap();

        // close UnstructuredGrid
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
    fn tetgen_write_vtu_1() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(4, None, None, None)?;
        tetgen
            .set_point(0, -1, 0.0, 0.0, 0.0)?
            .set_point(1, -2, 1.0, 0.0, 0.0)?
            .set_point(2, -3, 0.0, 1.0, 0.0)?
            .set_point(3, -4, 0.0, 0.0, 1.0)?;
        tetgen.generate_delaunay(false)?;
        let file_path = "/tmp/tritet/test_tetgen_write_vtu_1.vtu";
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
0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 
</DataArray>
</Points>
<Cells>
<DataArray type="Int32" Name="connectivity" format="ascii">
0 2 3 1 
</DataArray>
<DataArray type="Int32" Name="offsets" format="ascii">
4 
</DataArray>
<DataArray type="UInt8" Name="types" format="ascii">
10 
</DataArray>
</Cells>
<PointData Scalars="TheScalars">
<DataArray type="Int32" Name="marker" NumberOfComponents="1" format="ascii">
-1 -2 -3 -4 
</DataArray>
</PointData>
<CellData Scalars="TheScalars">
<DataArray type="Int32" Name="attribute" NumberOfComponents="1" format="ascii">
0 
</DataArray>
</CellData>
</Piece>
</UnstructuredGrid>
</VTKFile>
"#
        );
        Ok(())
    }

    #[test]
    fn tetgen_write_vtu_2() -> Result<(), StrError> {
        let mut tetgen = Tetgen::new(8, Some(vec![4, 4, 4, 4, 4, 4]), Some(1), None)?;
        tetgen
            .set_point(0, -100, 0.0, 0.0, 0.0)?
            .set_point(1, -200, 1.0, 0.0, 0.0)?
            .set_point(2, -300, 1.0, 1.0, 0.0)?
            .set_point(3, -400, 0.0, 1.0, 0.0)?
            .set_point(4, -500, 0.0, 0.0, 1.0)?
            .set_point(5, -600, 1.0, 0.0, 1.0)?
            .set_point(6, -700, 1.0, 1.0, 1.0)?
            .set_point(7, -800, 0.0, 1.0, 1.0)?;
        tetgen
            .set_facet_point(0, 0, 0)?
            .set_facet_point(0, 1, 4)?
            .set_facet_point(0, 2, 7)?
            .set_facet_point(0, 3, 3)?; // -x
        tetgen
            .set_facet_point(1, 0, 1)?
            .set_facet_point(1, 1, 2)?
            .set_facet_point(1, 2, 6)?
            .set_facet_point(1, 3, 5)?; // +x
        tetgen
            .set_facet_point(2, 0, 0)?
            .set_facet_point(2, 1, 1)?
            .set_facet_point(2, 2, 5)?
            .set_facet_point(2, 3, 4)?; // -y
        tetgen
            .set_facet_point(3, 0, 2)?
            .set_facet_point(3, 1, 3)?
            .set_facet_point(3, 2, 7)?
            .set_facet_point(3, 3, 6)?; // +y
        tetgen
            .set_facet_point(4, 0, 0)?
            .set_facet_point(4, 1, 3)?
            .set_facet_point(4, 2, 2)?
            .set_facet_point(4, 3, 1)?; // -z
        tetgen
            .set_facet_point(5, 0, 4)?
            .set_facet_point(5, 1, 5)?
            .set_facet_point(5, 2, 6)?
            .set_facet_point(5, 3, 7)?; // +z
        tetgen
            .set_facet_marker(0, -10)? // -x
            .set_facet_marker(1, -20)? // +x
            .set_facet_marker(2, -30)? // -y
            .set_facet_marker(3, -40)? // +y
            .set_facet_marker(4, -50)? // -z
            .set_facet_marker(5, -60)?; // +z

        tetgen.set_region(0, 1, 0.5, 0.5, 0.5, None)?;
        tetgen.generate_mesh(false, false, None, None)?;

        let file_path = "/tmp/tritet/test_tetgen_write_vtu_2.vtu";
        tetgen.write_vtu(file_path)?;
        let contents = fs::read_to_string(file_path).map_err(|_| "cannot open file")?;
        assert_eq!(
            contents,
            r#"<?xml version="1.0"?>
<VTKFile type="UnstructuredGrid" version="0.1" byte_order="LittleEndian">
<UnstructuredGrid>
<Piece NumberOfPoints="8" NumberOfCells="18">
<Points>
<DataArray type="Float64" NumberOfComponents="3" format="ascii">
0.0 0.0 0.0 1.0 0.0 0.0 1.0 1.0 0.0 0.0 1.0 0.0 0.0 0.0 1.0 1.0 0.0 1.0 1.0 1.0 1.0 0.0 1.0 1.0 
</DataArray>
</Points>
<Cells>
<DataArray type="Int32" Name="connectivity" format="ascii">
0 3 7 2 0 7 4 6 5 0 4 6 0 7 6 2 5 0 6 1 6 0 2 1 2 3 7 0 2 3 0 3 7 4 6 7 0 4 7 4 5 6 0 4 5 2 6 7 1 5 6 0 1 5 0 1 2 1 2 6 
</DataArray>
<DataArray type="Int32" Name="offsets" format="ascii">
4 8 12 16 20 24 27 30 33 36 39 42 45 48 51 54 57 60 
</DataArray>
<DataArray type="UInt8" Name="types" format="ascii">
10 10 10 10 10 10 5 5 5 5 5 5 5 5 5 5 5 5 
</DataArray>
</Cells>
<PointData Scalars="TheScalars">
<DataArray type="Int32" Name="marker" NumberOfComponents="1" format="ascii">
-100 -200 -300 -400 -500 -600 -700 -800 
</DataArray>
</PointData>
<CellData Scalars="TheScalars">
<DataArray type="Int32" Name="attribute" NumberOfComponents="1" format="ascii">
1 1 1 1 1 1 -40 -50 -10 -60 -10 -60 -30 -40 -20 -30 -50 -20 
</DataArray>
</CellData>
</Piece>
</UnstructuredGrid>
</VTKFile>
"#
        );
        Ok(())
    }
}
