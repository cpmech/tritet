#ifndef INTERFACE_TETGEN_H
#define INTERFACE_TETGEN_H

struct ExtTetgen {
    void* input;   // pointer to tetgenio
    void* output;  // pointer to tetgenio
};

struct ExtTetgen* new_tetgen(int npoint, int nfacet, int nregion, int nhole);

void drop_tetgen(struct ExtTetgen* tetgen);

int tet_set_point(struct ExtTetgen* tetgen, int index, double x, double y, double z);

int tet_set_facet_npoint(struct ExtTetgen* tetgen, int index, int npoint);

int tet_set_facet_point(struct ExtTetgen* tetgen, int index, int m, int p);

int tet_set_region(struct ExtTetgen* tetgen, int index, double x, double y, double z, int attribute, double max_volume);

int tet_set_hole(struct ExtTetgen* tetgen, int index, double x, double y, double z);

int tet_run_delaunay(struct ExtTetgen* tetgen, int verbose);

int tet_run_tetrahedralize(struct ExtTetgen* tetgen, int verbose, int o2, double global_max_volume, double global_min_angle);

int tet_get_npoint(struct ExtTetgen* tetgen);

int tet_get_ntetrahedron(struct ExtTetgen* tetgen);

int tet_get_ncorner(struct ExtTetgen* tetgen);

double tet_get_point(struct ExtTetgen* tetgen, int index, int dim);

int tet_get_tetrahedron_corner(struct ExtTetgen* tetgen, int index, int corner);

int tet_get_tetrahedron_attribute(struct ExtTetgen* tetgen, int index);

#endif  // INTERFACE_TETGEN_H