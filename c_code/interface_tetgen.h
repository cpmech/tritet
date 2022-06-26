#ifndef INTERFACE_TETGEN_H
#define INTERFACE_TETGEN_H

#include <inttypes.h>

#include "constants.h"

int32_t new_tetgen(HANDLE handle, int32_t npoint, int32_t nfacet, int32_t const *facet_npoint, int32_t nregion, int32_t nhole);

void drop_tetgen(HANDLE handle);

int32_t tet_set_point(HANDLE handle, int32_t index, double x, double y, double z);

int32_t tet_set_facet_point(HANDLE handle, int32_t index, int32_t m, int32_t p);

int32_t tet_set_region(HANDLE handle, int32_t index, double x, double y, double z, int32_t attribute, double max_volume);

int32_t tet_set_hole(HANDLE handle, int32_t index, double x, double y, double z);

int32_t tet_run_delaunay(HANDLE handle, int32_t verbose);

int32_t tet_run_tetrahedralize(HANDLE handle, int32_t verbose, int32_t o2, double global_max_volume, double global_min_angle);

int32_t tet_get_npoint(HANDLE handle);

int32_t tet_get_ntetrahedron(HANDLE handle);

int32_t tet_get_ncorner(HANDLE handle);

double tet_get_point(HANDLE handle, int32_t index, int32_t dim);

int32_t tet_get_tetrahedron_corner(HANDLE handle, int32_t index, int32_t corner);

int32_t tet_get_tetrahedron_attribute(HANDLE handle, int32_t index);

#endif  // INTERFACE_TETGEN_H