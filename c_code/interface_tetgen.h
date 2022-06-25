#ifndef INTERFACE_TETGEN_H
#define INTERFACE_TETGEN_H

#include <inttypes.h>

#include "constants.h"

int new_tetgen(HANDLE handle, int npoint, int nfacet, int32_t const *facet_npoint, int nregion, int nhole);

void drop_tetgen(HANDLE handle);

int tet_set_point(HANDLE handle, int index, double x, double y, double z);

int tet_set_facet_npoint(HANDLE handle, int index, int npoint);

int tet_set_facet_point(HANDLE handle, int index, int m, int p);

int tet_set_region(HANDLE handle, int index, double x, double y, double z, int attribute, double max_volume);

int tet_set_hole(HANDLE handle, int index, double x, double y, double z);

int tet_run_delaunay(HANDLE handle, int verbose);

int tet_run_tetrahedralize(HANDLE handle, int verbose, int o2, double global_max_volume, double global_min_angle);

int tet_get_npoint(HANDLE handle);

int tet_get_ntetrahedron(HANDLE handle);

int tet_get_ncorner(HANDLE handle);

double tet_get_point(HANDLE handle, int index, int dim);

int tet_get_tetrahedron_corner(HANDLE handle, int index, int corner);

int tet_get_tetrahedron_attribute(HANDLE handle, int index);

#endif  // INTERFACE_TETGEN_H