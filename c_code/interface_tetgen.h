#ifndef INTERFACE_TETGEN_H
#define INTERFACE_TETGEN_H

#include "tetgen.h"

struct ExtTetgen {
    struct tetgenio input;
    struct tetgenio output;
};

struct ExtTetgen *tet_new_tetgen(int npoint, int nfacet, int const *facet_npoint, int nregion, int nhole);

void tet_drop_tetgen(struct ExtTetgen *tetgen);

int tet_set_point(struct ExtTetgen *tetgen, int index, int marker, double x, double y, double z);

int tet_set_facet_point(struct ExtTetgen *tetgen, int index, int m, int p);

int tet_set_facet_marker(struct ExtTetgen *tetgen, int index, int marker);

int tet_set_region(struct ExtTetgen *tetgen, int index, int attribute, double x, double y, double z, double max_volume);

int tet_set_hole(struct ExtTetgen *tetgen, int index, double x, double y, double z);

int tet_run_delaunay(struct ExtTetgen *tetgen, int verbose);

int tet_run_tetrahedralize(struct ExtTetgen *tetgen, int verbose, int o2, double global_max_volume, double global_min_angle);

int tet_out_npoint(struct ExtTetgen *tetgen);

int tet_out_ncell(struct ExtTetgen *tetgen); // a "cell" here is a "tetrahedron"

int tet_out_cell_npoint(struct ExtTetgen *tetgen);

double tet_out_point(struct ExtTetgen *tetgen, int index, int dim);

int tet_out_point_marker(struct ExtTetgen *tetgen, int index);

int tet_out_cell_point(struct ExtTetgen *tetgen, int index, int corner);

int tet_out_cell_attribute(struct ExtTetgen *tetgen, int index);

int tet_out_n_marked_face(struct ExtTetgen *tetgen);

void tet_out_marked_face(struct ExtTetgen *tetgen, int index, int *a, int *b, int *c, int *marker, int *cell);

#endif // INTERFACE_TETGEN_H