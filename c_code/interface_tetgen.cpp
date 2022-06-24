#include <stdio.h>
#include <stdlib.h>

#include "constants.h"
#include "tetgen.h"

extern "C" {
#include "interface_tetgen.h"
}

struct ExtTetgen *new_tetgen(int npoint, int nfacet, int nregion, int nhole) {
    struct ExtTetgen *tetgen = (struct ExtTetgen *)malloc(sizeof(struct ExtTetgen));
    if (tetgen == NULL) {
        return NULL;
    }

    tetgenio *input = new tetgenio;
    if (input == NULL) {
        return NULL;
    }
    input->initialize();

    tetgenio *output = new tetgenio;
    if (output == NULL) {
        return NULL;
    }
    output->initialize();

    tetgen->input = input;
    tetgen->output = output;
    return tetgen;
}

void drop_tetgen(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (input != NULL) {
        input->deinitialize();
    }
    if (output != NULL) {
        output->deinitialize();
    }
    delete input;
    delete output;
    free(tetgen);
}

int tet_set_point(struct ExtTetgen *tetgen, int index, double x, double y, double z) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    if (input == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (input->pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (index >= input->numberofpoints) {
        return TRITET_ERROR_INVALID_POINT_INDEX;
    }
    input->pointlist[index * 3] = x;
    input->pointlist[index * 3 + 1] = y;
    input->pointlist[index * 3 + 2] = z;
    return TRITET_SUCCESS;
}

int tet_set_facet_npoint(struct ExtTetgen *tetgen, int index, int npoint) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    if (input == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (input->facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
    }
    if (index >= input->numberoffacets) {
        return TRITET_ERROR_INVALID_FACET_INDEX;
    }
    tetgenio::facet *fac = &input->facetlist[index];
    if (fac->polygonlist == NULL) {
        const int NUM_POLY = 1;
        fac->polygonlist = new tetgenio::polygon[NUM_POLY];
        fac->numberofpolygons = NUM_POLY;
        fac->numberofholes = 0;
        fac->holelist = NULL;
        tetgenio::polygon *gon = &fac->polygonlist[0];
        gon->vertexlist = new int[npoint];
        gon->numberofvertices = npoint;
    }
    return TRITET_SUCCESS;
}

int tet_set_facet_point(struct ExtTetgen *tetgen, int index, int m, int p) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    if (input == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (input->facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
    }
    if (index >= input->numberoffacets) {
        return TRITET_ERROR_INVALID_FACET_INDEX;
    }
    tetgenio::facet *fac = &input->facetlist[index];
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
    if (p >= input->numberofpoints) {
        return TRITET_ERROR_INVALID_FACET_POINT_ID;
    }
    gon->vertexlist[m] = p;
    return TRITET_SUCCESS;
}

int tet_set_region(struct ExtTetgen *tetgen, int index, double x, double y, double z, int attribute, double max_volume) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    if (input == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (input->regionlist == NULL) {
        return TRITET_ERROR_NULL_REGION_LIST;
    }
    if (index >= input->numberofregions) {
        return TRITET_ERROR_INVALID_REGION_INDEX;
    }
    input->regionlist[index * 5] = x;
    input->regionlist[index * 5 + 1] = y;
    input->regionlist[index * 5 + 2] = z;
    input->regionlist[index * 5 + 3] = attribute;
    input->regionlist[index * 5 + 4] = max_volume;
    return TRITET_SUCCESS;
}

int tet_set_hole(struct ExtTetgen *tetgen, int index, double x, double y, double z) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    if (input == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (input->holelist == NULL) {
        return TRITET_ERROR_NULL_HOLE_LIST;
    }
    if (index >= input->numberofholes) {
        return TRITET_ERROR_INVALID_HOLE_INDEX;
    }
    input->holelist[index * 3] = x;
    input->holelist[index * 3 + 1] = y;
    input->holelist[index * 3 + 2] = z;
    return TRITET_SUCCESS;
}

int tet_run_delaunay(struct ExtTetgen *tetgen, int verbose) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    if (input == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (input->pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return TRITET_ERROR_NULL_DATA;
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
        tetrahedralize(command, input, output, NULL, NULL);
    } catch (int status) {
        printf("status = %d\n", status);  // TODO
    } catch (...) {
        return 1;  // TODO
    }

    return TRITET_SUCCESS;
}

int tet_run_tetrahedralize(struct ExtTetgen *tetgen, int verbose, int o2, double global_max_volume, double global_min_angle) {
    if (tetgen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    tetgenio *input = reinterpret_cast<tetgenio *>(tetgen->input);
    if (input == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (input->pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (input->facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }

    // Generate mesh
    // Switches:
    // * `p` -- tetrahedralize a piecewise linear complex (PLC)
    // * `z` -- number everything from zero (z)
    // * `A` -- assign a regional attribute to each element (A)
    char command[128];
    strcpy(command, "pzA");
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
        tetrahedralize(command, input, output, NULL, NULL);
    } catch (int status) {
        printf("status = %d\n", status);  // TODO
    } catch (...) {
        return 1;  // TODO
    }

    return TRITET_SUCCESS;
}

int tet_get_npoint(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return 0;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return 0;
    }
    return output->numberofpoints;
}

int tet_get_ntetrahedron(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return 0;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return 0;
    }
    return output->numberoftetrahedra;
}

int tet_get_ncorner(struct ExtTetgen *tetgen) {
    if (tetgen == NULL) {
        return 0;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return 0;
    }
    return output->numberofcorners;
}

double tet_get_point(struct ExtTetgen *tetgen, int index, int dim) {
    if (tetgen == NULL) {
        return 0.0;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return 0.0;
    }
    if (index < output->numberofpoints && (dim == 0 || dim == 1 || dim == 2)) {
        return output->pointlist[index * 3 + dim];
    } else {
        return 0.0;
    }
}

int tet_get_tetrahedron_corner(struct ExtTetgen *tetgen, int index, int corner) {
    if (tetgen == NULL) {
        return 0;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return 0;
    }
    if (index < output->numberoftetrahedra && corner < output->numberofcorners) {
        return output->tetrahedronlist[index * output->numberofcorners + corner];
    } else {
        return 0;
    }
}

int tet_get_tetgen_attribute(struct ExtTetgen *tetgen, int index) {
    if (tetgen == NULL) {
        return 0;
    }
    tetgenio *output = reinterpret_cast<tetgenio *>(tetgen->output);
    if (output == NULL) {
        return 0;
    }
    if (index < output->numberoftetrahedra && output->numberoftetrahedronattributes > 0) {
        return output->tetrahedronattributelist[index * output->numberoftetrahedronattributes];
    } else {
        return 0;
    }
}
