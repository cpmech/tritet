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

struct ExtTriangle {
    struct triangulateio input;
    struct triangulateio output;
    struct triangulateio voronoi;
};

struct ExtTriangle *new_triangle(int32_t npoint, int32_t nsegment, int32_t nregion, int32_t nhole);

void drop_triangle(struct ExtTriangle *triangle);

int32_t set_point(struct ExtTriangle *triangle, int32_t index, double x, double y);

int32_t set_segment(struct ExtTriangle *triangle, int32_t index, int32_t a, int32_t b);

int32_t set_region(struct ExtTriangle *triangle, int32_t index, double x, double y, int32_t attribute, double max_area);

int32_t set_hole(struct ExtTriangle *triangle, int32_t index, double x, double y);

int32_t run_delaunay(struct ExtTriangle *triangle, int32_t verbose);

int32_t run_voronoi(struct ExtTriangle *triangle, int32_t verbose);

int32_t run_triangulate(struct ExtTriangle *triangle, int32_t verbose, int32_t quadratic, double global_max_area, double global_min_angle);

int32_t get_npoint(struct ExtTriangle *triangle);

int32_t get_ntriangle(struct ExtTriangle *triangle);

int32_t get_ncorner(struct ExtTriangle *triangle);

double get_point(struct ExtTriangle *triangle, int32_t index, int32_t dim);

int32_t get_triangle_corner(struct ExtTriangle *triangle, int32_t index, int32_t corner);

int32_t get_triangle_attribute(struct ExtTriangle *triangle, int32_t index);

int32_t get_voronoi_npoint(struct ExtTriangle *triangle);

int32_t get_voronoi_point(struct ExtTriangle *triangle, int32_t index, int32_t dim);

int32_t get_voronoi_nedge(struct ExtTriangle *triangle);

int32_t get_voronoi_edge_point(struct ExtTriangle *triangle, int32_t index, int32_t side);

double get_voronoi_edge_point_b_direction(struct ExtTriangle *triangle, int32_t index, int32_t dim);

#endif  // INTERFACE_TRIANGLE_H
