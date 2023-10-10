#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

#include <new>

#include "constants.h"
#include "tetgen.h"

extern "C" {
#include "interface_tetgen.h"
}

void tet_drop_tetgen(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return;
    }
    delete tetgen;
}

struct ExtTetgen *tet_new_tetgen(int npoint, int nfacet, int const *facet_npoint, int nregion, int nhole) {
    if (npoint < 4) {
        return NULL;
    }

    // tetgen
    struct ExtTetgen *tetgen = new (std::nothrow) ExtTetgen;
    if (tetgen == NULL) {
        return NULL;
    }
    try {
        tetgen->input.initialize();
        tetgen->output.initialize();
    } catch (...) {
        tet_drop_tetgen(tetgen);
        return NULL;
    }

    // points
    tetgen->input.firstnumber = 0;
    tetgen->input.numberofpoints = npoint;
    tetgen->input.pointlist = new (std::nothrow) double[npoint * 3];
    if (tetgen->input.pointlist == NULL) {
        tet_drop_tetgen(tetgen);
        return NULL;
    }

    // point markers
    tetgen->input.pointmarkerlist = new (std::nothrow) int[npoint];
    if (tetgen->input.pointmarkerlist == NULL) {
        tet_drop_tetgen(tetgen);
        return NULL;
    }

    // facets
    if (nfacet > 0) {
        tetgen->input.numberoffacets = nfacet;
        tetgen->input.facetlist = new (std::nothrow) tetgenio::facet[nfacet];
        if (tetgen->input.facetlist == NULL) {
            tet_drop_tetgen(tetgen);
            return NULL;
        }
        tetgen->input.facetmarkerlist = new (std::nothrow) int[nfacet];
        const int NUM_POLY = 1;
        for (int index = 0; index < nfacet; index++) {
            // facet polygon
            tetgenio::facet *fac = &tetgen->input.facetlist[index];
            fac->polygonlist = new (std::nothrow) tetgenio::polygon[NUM_POLY];
            if (fac->polygonlist == NULL) {
                tet_drop_tetgen(tetgen);
                return NULL;
            }
            fac->numberofpolygons = NUM_POLY;
            fac->numberofholes = 0;
            fac->holelist = NULL;
            // facet polygon vertices
            size_t nvertex = facet_npoint[index];
            tetgenio::polygon *gon = &fac->polygonlist[0];
            gon->vertexlist = new (std::nothrow) int[nvertex];
            if (gon->vertexlist == NULL) {
                tet_drop_tetgen(tetgen);
                return NULL;
            }
            gon->numberofvertices = nvertex;
            // facet marker
            tetgen->input.facetmarkerlist[index] = 0;
        }
    }

    // regions
    if (nregion > 0) {
        tetgen->input.numberofregions = nregion;
        tetgen->input.regionlist = new (std::nothrow) double[nregion * 5];
        if (tetgen->input.regionlist == NULL) {
            tet_drop_tetgen(tetgen);
            return NULL;
        }
    }

    // holes
    if (nhole > 0) {
        tetgen->input.numberofholes = nhole;
        tetgen->input.holelist = new (std::nothrow) double[nhole * 3];
        if (tetgen->input.holelist == NULL) {
            tet_drop_tetgen(tetgen);
            return NULL;
        }
    }

    return tetgen;
}

int tet_set_point(struct ExtTetgen *tetgen, int index, int marker, double x, double y, double z) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tetgen->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (index >= tetgen->input.numberofpoints) {
        return TRITET_ERROR_INVALID_POINT_INDEX;
    }
    tetgen->input.pointlist[index * 3] = x;
    tetgen->input.pointlist[index * 3 + 1] = y;
    tetgen->input.pointlist[index * 3 + 2] = z;
    tetgen->input.pointmarkerlist[index] = marker;

    return TRITET_SUCCESS;
}

int tet_set_facet_point(struct ExtTetgen *tetgen, int index, int m, int p) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tetgen->input.facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
    }
    if (index >= tetgen->input.numberoffacets) {
        return TRITET_ERROR_INVALID_FACET_INDEX;
    }

    tetgenio::facet *fac = &tetgen->input.facetlist[index];
    if (fac->polygonlist == NULL) {
        return TRITET_ERROR_NULL_FACET_POLYGON_LIST;
    }
    if (fac->numberofpolygons != 1) {
        return TRITET_ERROR_INVALID_FACET_NUM_POLYGON;
    }

    tetgenio::polygon *gon = &fac->polygonlist[0];
    if (m >= gon->numberofvertices) {
        return TRITET_ERROR_INVALID_FACET_POINT_INDEX;
    }
    if (p >= tetgen->input.numberofpoints) {
        return TRITET_ERROR_INVALID_FACET_POINT_ID;
    }
    gon->vertexlist[m] = p;

    return TRITET_SUCCESS;
}

int tet_set_facet_marker(struct ExtTetgen *tetgen, int index, int marker) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tetgen->input.facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
    }
    if (index >= tetgen->input.numberoffacets) {
        return TRITET_ERROR_INVALID_FACET_INDEX;
    }

    tetgen->input.facetmarkerlist[index] = marker;

    return TRITET_SUCCESS;
}

int tet_set_region(struct ExtTetgen *tetgen, int index, int attribute, double x, double y, double z, double max_volume) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tetgen->input.regionlist == NULL) {
        return TRITET_ERROR_NULL_REGION_LIST;
    }
    if (index >= tetgen->input.numberofregions) {
        return TRITET_ERROR_INVALID_REGION_INDEX;
    }
    tetgen->input.regionlist[index * 5] = x;
    tetgen->input.regionlist[index * 5 + 1] = y;
    tetgen->input.regionlist[index * 5 + 2] = z;
    tetgen->input.regionlist[index * 5 + 3] = attribute;
    tetgen->input.regionlist[index * 5 + 4] = max_volume;

    return TRITET_SUCCESS;
}

int tet_set_hole(struct ExtTetgen *tetgen, int index, double x, double y, double z) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tetgen->input.holelist == NULL) {
        return TRITET_ERROR_NULL_HOLE_LIST;
    }
    if (index >= tetgen->input.numberofholes) {
        return TRITET_ERROR_INVALID_HOLE_INDEX;
    }
    tetgen->input.holelist[index * 3] = x;
    tetgen->input.holelist[index * 3 + 1] = y;
    tetgen->input.holelist[index * 3 + 2] = z;

    return TRITET_SUCCESS;
}

int tet_run_delaunay(struct ExtTetgen *tetgen, int verbose) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tetgen->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }

    // Tetrahedralize the points
    // Switches:
    // * `z` -- number everything from zero (z)
    char command[10];
    strcpy(command, "z");
    if (verbose == TRITET_FALSE) {
        strcat(command, "Q");
    }
    try {
        tetrahedralize(command, &tetgen->input, &tetgen->output, NULL, NULL);
    } catch (int status) {
        printf("status = %d\n", status); // TODO
    } catch (...) {
        return 1; // TODO
    }

    return TRITET_SUCCESS;
}

int tet_run_tetrahedralize(struct ExtTetgen *tetgen, int verbose, int o2, double global_max_volume, double global_min_angle) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tetgen->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (tetgen->input.facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
    }

    // Generate mesh
    // Selected:
    // * `p` -- tetrahedralize a piecewise linear complex (PLC)
    // * `z` -- number everything from zero (z)
    // * `A` -- assign a regional attribute to each element (A)
    // * `f` -- Outputs all faces to .face file
    // All:
    // * `b` -- NOT AVAILABLE / DISABLED
    // * `p` -- Tetrahedralize a piecewise linear complex (PLC)
    // * `Y` -- Preserves the input surface mesh (does not modify it)
    // * `r` -- Reconstructs a previously generated mesh
    // * `q` -- Refines mesh (to improve mesh quality)
    // * `R` -- Mesh coarsening (to reduce the mesh elements)
    // * `A` -- Assigns attributes to tetrahedra in different regions
    // * `a` -- Applies a maximum tetrahedron volume constraint
    // * `m` -- Applies a mesh sizing function
    // * `i` -- Inserts a list of additional points
    // * `O` -- Specifies the level of mesh optimization
    // * `S` -- Specifies maximum number of added points
    // * `T` -- Sets a tolerance for coplanar test (default 1e-8)
    // * `X` -- Suppresses use of exact arithmetic
    // * `M` -- No merge of coplanar facets or very close vertices
    // * `w` -- Generates weighted Delaunay (regular) triangulation
    // * `c` -- Retains the convex hull of the PLC
    // * `d` -- Detects self-intersections of facets of the PLC
    // * `z` -- Numbers all output items starting from zero
    // * `f` -- Outputs all faces to .face file
    // * `e` -- Outputs all edges to .edge file
    // * `n` -- Outputs tetrahedra neighbors to .neigh file
    // * `v` -- Outputs Voronoi diagram to files
    // * `g` -- Outputs mesh to .mesh file for viewing by Medit
    // * `k` -- Outputs mesh to .vtk file for viewing by Paraview
    // * `J` -- No jettison of unused vertices from output .node file
    // * `B` -- Suppresses output of boundary information
    // * `N` -- Suppresses output of .node file
    // * `E` -- Suppresses output of .ele file
    // * `F` -- Suppresses output of .face and .edge file
    // * `I` -- Suppresses mesh iteration numbers
    // * `C` -- Checks the consistency of the final mesh
    // * `Q` -- Quiet: No terminal output except errors
    // * `V` -- Verbose: Detailed information, more terminal output
    // * `h` -- Help: A brief instruction for using TetGen
    char command[128];
    strcpy(command, "pzAf");
    if (verbose == TRITET_FALSE) {
        strcat(command, "Q");
    }
    if (o2 == TRITET_TRUE) {
        strcat(command, "o2");
    }
    if (global_max_volume > 0.0) {
        char buf[32];
        int n = snprintf(buf, 32, "a%.15f", global_max_volume);
        if (n >= 32) {
            return TRITET_ERROR_STRING_CONCAT;
        }
        strcat(command, buf);
    }
    if (global_min_angle > 0.0) {
        char buf[32];
        int n = snprintf(buf, 32, "q%.15f", global_min_angle);
        if (n >= 32) {
            return TRITET_ERROR_STRING_CONCAT;
        }
        strcat(command, buf);
    } else {
        strcat(command, "q");
    }
    try {
        tetrahedralize(command, &tetgen->input, &tetgen->output, NULL, NULL);
    } catch (int status) {
        printf("status = %d\n", status); // TODO
    } catch (...) {
        return 1; // TODO
    }

    return TRITET_SUCCESS;
}

int tet_out_npoint(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return 0;
    }
    return tetgen->output.numberofpoints;
}

int tet_out_ncell(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return 0;
    }
    return tetgen->output.numberoftetrahedra;
}

int tet_out_cell_npoint(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return 0;
    }
    return tetgen->output.numberofcorners;
}

double tet_out_point(struct ExtTetgen *tetgen, int index, int dim) {
    if (tetgen == NULL) {
        return 0.0;
    }
    if (index >= 0 && index < tetgen->output.numberofpoints && (dim == 0 || dim == 1 || dim == 2)) {
        return tetgen->output.pointlist[index * 3 + dim];
    } else {
        return 0.0;
    }
}

int tet_out_point_marker(struct ExtTetgen *tetgen, int index) {
    if (tetgen == NULL) {
        return 0;
    }
    if (index >= 0 && index < tetgen->output.numberofpoints) {
        return tetgen->output.pointmarkerlist[index];
    } else {
        return 0;
    }
}

int tet_out_cell_point(struct ExtTetgen *tetgen, int index, int corner) {
    if (tetgen == NULL) {
        return 0;
    }
    if (index >= 0 && index < tetgen->output.numberoftetrahedra && corner < tetgen->output.numberofcorners) {
        return tetgen->output.tetrahedronlist[index * tetgen->output.numberofcorners + corner];
    } else {
        return 0;
    }
}

int tet_out_cell_attribute(struct ExtTetgen *tetgen, int index) {
    if (tetgen == NULL) {
        return 0;
    }
    if (index >= 0 && index < tetgen->output.numberoftetrahedra && tetgen->output.numberoftetrahedronattributes > 0) {
        return tetgen->output.tetrahedronattributelist[index * tetgen->output.numberoftetrahedronattributes];
    } else {
        return 0;
    }
}

int tet_out_n_marked_face(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return 0;
    }
    return static_cast<int>(tetgen->output.marked_faces.size());
}

void tet_out_marked_face(struct ExtTetgen *tetgen, int index, int *a, int *b, int *c, int *marker, int *cell) {
    *a = 0;
    *b = 0;
    *c = 0;
    *marker = 0;
    *cell = 0;
    if (tetgen == NULL) {
        return;
    }
    if (index >= 0 && index < static_cast<int>(tetgen->output.marked_faces.size())) {
        auto marked_face = tetgen->output.marked_faces[index];
        *a = marked_face.key[0];
        *b = marked_face.key[1];
        *c = marked_face.key[2];
        *marker = marked_face.marker;
        *cell = marked_face.cell;
    } else {
        return;
    }
}
