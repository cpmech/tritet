#ifndef INTERFACE_TETGEN_H
#define INTERFACE_TETGEN_H

#include <inttypes.h>

#include "tetgen.h"

struct ExtTetgen {
    struct tetgenio input;
    struct tetgenio output;
};

struct ExtTetgen *tet_new_tetgen(int32_t npoint, int32_t nfacet, int32_t const *facet_npoint, int32_t nregion, int32_t nhole);

void tet_drop_tetgen(struct ExtTetgen *tetgen);

int32_t tet_set_point(struct ExtTetgen *tetgen, int32_t index, int32_t marker, double x, double y, double z);

int32_t tet_set_facet_point(struct ExtTetgen *tetgen, int32_t index, int32_t m, int32_t p);

int32_t tet_set_facet_marker(struct ExtTetgen *tetgen, int32_t index, int32_t marker);

int32_t tet_set_region(struct ExtTetgen *tetgen, int32_t index, int32_t attribute, double x, double y, double z, double max_volume);

int32_t tet_set_hole(struct ExtTetgen *tetgen, int32_t index, double x, double y, double z);

int32_t tet_run_delaunay(struct ExtTetgen *tetgen, int32_t verbose);

int32_t tet_run_tetrahedralize(struct ExtTetgen *tetgen, int32_t verbose, int32_t o2, double global_max_volume, double global_min_angle);

int32_t tet_out_npoint(struct ExtTetgen *tetgen);

int32_t tet_out_ncell(struct ExtTetgen *tetgen); // a "cell" here is a "tetrahedron"

int32_t tet_out_cell_npoint(struct ExtTetgen *tetgen);

double tet_out_point(struct ExtTetgen *tetgen, int32_t index, int32_t dim);

int32_t tet_out_point_marker(struct ExtTetgen *tetgen, int32_t index);

int32_t tet_out_cell_point(struct ExtTetgen *tetgen, int32_t index, int32_t corner);

int32_t tet_out_cell_attribute(struct ExtTetgen *tetgen, int32_t index);

int32_t tet_out_n_marked_face(struct ExtTetgen *tetgen);

void tet_out_marked_face(struct ExtTetgen *tetgen, int32_t index, int32_t *a, int32_t *b, int32_t *c, int32_t *marker, int32_t *cell);

#endif // INTERFACE_TETGEN_H