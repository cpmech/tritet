#ifndef INTERFACE_TRIANGLE_H
#define INTERFACE_TRIANGLE_H

#include <inttypes.h>

#define REAL double
#define ANSI_DECLARATORS
#define VOID int32_t
#include "triangle.h"
#undef REAL
#undef ANSI_DECLARATORS
#undef VOID

struct ExtTrigen {
    struct triangulateio input;
    struct triangulateio output;
    struct triangulateio voronoi;
};

struct ExtTrigen *new_trigen(int32_t npoint, int32_t nsegment, int32_t nregion, int32_t nhole);

void drop_trigen(struct ExtTrigen *trigen);

int32_t set_point(struct ExtTrigen *trigen, int32_t index, double x, double y);

int32_t set_segment(struct ExtTrigen *trigen, int32_t index, int32_t marker, int32_t a, int32_t b);

int32_t set_region(struct ExtTrigen *trigen, int32_t index, int32_t attribute, double x, double y, double max_area);

int32_t set_hole(struct ExtTrigen *trigen, int32_t index, double x, double y);

int32_t run_delaunay(struct ExtTrigen *trigen, int32_t verbose);

int32_t run_voronoi(struct ExtTrigen *trigen, int32_t verbose);

int32_t run_triangulate(struct ExtTrigen *trigen, int32_t verbose, int32_t quadratic, int32_t allow_new_points_on_bry, double global_max_area, double global_min_angle);

int32_t get_npoint(struct ExtTrigen *trigen);

int32_t get_n_out_segment(struct ExtTrigen *trigen);

int32_t get_ntriangle(struct ExtTrigen *trigen);

int32_t get_ncorner(struct ExtTrigen *trigen);

double get_point(struct ExtTrigen *trigen, int32_t index, int32_t dim);

void get_out_segment(struct ExtTrigen *trigen, int32_t index, int32_t *marker, int32_t *a, int32_t *b);

int32_t get_triangle_corner(struct ExtTrigen *trigen, int32_t index, int32_t corner);

int32_t get_triangle_attribute(struct ExtTrigen *trigen, int32_t index);

int32_t get_voronoi_npoint(struct ExtTrigen *trigen);

int32_t get_voronoi_point(struct ExtTrigen *trigen, int32_t index, int32_t dim);

int32_t get_voronoi_nedge(struct ExtTrigen *trigen);

int32_t get_voronoi_edge_point(struct ExtTrigen *trigen, int32_t index, int32_t side);

double get_voronoi_edge_point_b_direction(struct ExtTrigen *trigen, int32_t index, int32_t dim);

#endif // INTERFACE_TRIANGLE_H
