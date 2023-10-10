#ifndef INTERFACE_TRIANGLE_H
#define INTERFACE_TRIANGLE_H

#include <inttypes.h>

#define REAL double
#define ANSI_DECLARATORS
#define VOID int
#include "triangle.h"
#undef REAL
#undef ANSI_DECLARATORS
#undef VOID

struct ExtTrigen {
    struct triangulateio input;
    struct triangulateio output;
    struct triangulateio voronoi;
};

struct ExtTrigen *tri_new_trigen(int npoint, int nsegment, int nregion, int nhole);

void tri_drop_trigen(struct ExtTrigen *trigen);

int tri_set_point(struct ExtTrigen *trigen, int index, int marker, double x, double y);

int tri_set_segment(struct ExtTrigen *trigen, int index, int marker, int a, int b);

int tri_set_region(struct ExtTrigen *trigen, int index, int attribute, double x, double y, double max_area);

int tri_set_hole(struct ExtTrigen *trigen, int index, double x, double y);

int tri_run_delaunay(struct ExtTrigen *trigen, int verbose);

int tri_run_voronoi(struct ExtTrigen *trigen, int verbose);

int tri_run_triangulate(struct ExtTrigen *trigen, int verbose, int quadratic, int allow_new_points_on_bry, double global_max_area, double global_min_angle);

int tri_out_npoint(struct ExtTrigen *trigen);

int tri_out_nsegment(struct ExtTrigen *trigen);

int tri_out_ncell(struct ExtTrigen *trigen); // a "cell" here is a "triangle"

int tri_out_cell_npoint(struct ExtTrigen *trigen);

double tri_out_point(struct ExtTrigen *trigen, int index, int dim);

int tri_out_point_marker(struct ExtTrigen *trigen, int index);

int tri_out_segment_point(struct ExtTrigen *trigen, int index, int side);

int tri_out_segment_marker(struct ExtTrigen *trigen, int index);

int tri_out_cell_point(struct ExtTrigen *trigen, int index, int corner);

int tri_out_cell_attribute(struct ExtTrigen *trigen, int index);

int tri_out_voronoi_npoint(struct ExtTrigen *trigen);

int tri_out_voronoi_point(struct ExtTrigen *trigen, int index, int dim);

int tri_out_voronoi_nedge(struct ExtTrigen *trigen);

int tri_out_voronoi_edge_point(struct ExtTrigen *trigen, int index, int side);

double tri_out_voronoi_edge_point_b_direction(struct ExtTrigen *trigen, int index, int dim);

#endif // INTERFACE_TRIANGLE_H
