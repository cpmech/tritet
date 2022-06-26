#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

#include <map>

#include "constants.h"
#include "tetgen.h"

extern "C" {
#include "interface_tetgen.h"
}

class ExtTetgenClass {
   public:
    tetgenio input;
    tetgenio output;
};

typedef std::map<int, ExtTetgenClass *> Database_t;

Database_t DATABASE;

void drop_tetgen(HANDLE handle) {
    try {
        if (DATABASE.count(handle) > 0) {
            ExtTetgenClass *tg = DATABASE[handle];
            delete tg;
            DATABASE.erase(handle);
        }
    } catch (...) {
    }
}

int new_tetgen(HANDLE handle, int npoint, int nfacet, int32_t const *facet_npoint, int nregion, int nhole) {
    ExtTetgenClass *tg;
    try {
        tg = new ExtTetgenClass;
        if (tg == NULL) {
            return TRITET_ERROR_NULL_DATA;
        }
        DATABASE[handle] = tg;
        tg->input.initialize();
        tg->output.initialize();
    } catch (...) {
        return TRITET_ERROR_INITIALIZE_FAILED;
    }

    // points
    try {
        tg->input.firstnumber = 0;
        tg->input.numberofpoints = npoint;
        tg->input.pointlist = new double[npoint * 3];
        if (tg->input.pointlist == NULL) {
            return TRITET_ERROR_ALLOC_POINT_LIST_FAILED;
        }
    } catch (...) {
        return TRITET_ERROR_ALLOC_POINT_LIST_FAILED;
    }

    // facets
    if (nfacet > 0) {
        try {
            tg->input.numberoffacets = nfacet;
            tg->input.facetlist = new tetgenio::facet[nfacet];
            if (tg->input.facetlist == NULL) {
                return TRITET_ERROR_ALLOC_FACET_LIST_FAILED;
            }
        } catch (...) {
            return TRITET_ERROR_ALLOC_FACET_LIST_FAILED;
        }
        try {
            const int NUM_POLY = 1;
            for (size_t index = 0; index < nfacet; index++) {
                // facet polygon
                tetgenio::facet *fac = &tg->input.facetlist[index];
                fac->polygonlist = new tetgenio::polygon[NUM_POLY];
                if (fac->polygonlist == NULL) {
                    return TRITET_ERROR_ALLOC_FACET_DATA_FAILED;
                }
                fac->numberofpolygons = NUM_POLY;
                fac->numberofholes = 0;
                fac->holelist = NULL;
                // face polygon vertices
                size_t nvertex = facet_npoint[index];
                tetgenio::polygon *gon = &fac->polygonlist[0];
                gon->vertexlist = new int[nvertex];
                if (gon->vertexlist == NULL) {
                    return TRITET_ERROR_ALLOC_FACET_DATA_FAILED;
                }
                gon->numberofvertices = nvertex;
            }
        } catch (...) {
            return TRITET_ERROR_ALLOC_FACET_DATA_FAILED;
        }
    }

    // regions
    if (nregion > 0) {
        try {
            tg->input.numberofregions = nregion;
            tg->input.regionlist = new double[nregion * 5];
            if (tg->input.regionlist == NULL) {
                return TRITET_ERROR_ALLOC_REGION_LIST_FAILED;
            }
        } catch (...) {
            return TRITET_ERROR_ALLOC_REGION_LIST_FAILED;
        }
    }

    // holes
    if (nhole > 0) {
        try {
            tg->input.numberofholes = nhole;
            tg->input.holelist = new double[nhole * 3];
            if (tg->input.holelist == NULL) {
                return TRITET_ERROR_ALLOC_HOLE_LIST_FAILED;
            }
        } catch (...) {
            return TRITET_ERROR_ALLOC_HOLE_LIST_FAILED;
        }
    }

    return TRITET_SUCCESS;
}

int tet_set_point(HANDLE handle, int index, double x, double y, double z) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tg->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (index >= tg->input.numberofpoints) {
        return TRITET_ERROR_INVALID_POINT_INDEX;
    }
    tg->input.pointlist[index * 3] = x;
    tg->input.pointlist[index * 3 + 1] = y;
    tg->input.pointlist[index * 3 + 2] = z;
    return TRITET_SUCCESS;
}

int tet_set_facet_point(HANDLE handle, int index, int m, int p) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tg->input.facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
    }
    if (index >= tg->input.numberoffacets) {
        return TRITET_ERROR_INVALID_FACET_INDEX;
    }
    tetgenio::facet *fac = &tg->input.facetlist[index];
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
    if (p >= tg->input.numberofpoints) {
        return TRITET_ERROR_INVALID_FACET_POINT_ID;
    }
    gon->vertexlist[m] = p;
    return TRITET_SUCCESS;
}

int tet_set_region(HANDLE handle, int index, double x, double y, double z, int attribute, double max_volume) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tg->input.regionlist == NULL) {
        return TRITET_ERROR_NULL_REGION_LIST;
    }
    if (index >= tg->input.numberofregions) {
        return TRITET_ERROR_INVALID_REGION_INDEX;
    }
    tg->input.regionlist[index * 5] = x;
    tg->input.regionlist[index * 5 + 1] = y;
    tg->input.regionlist[index * 5 + 2] = z;
    tg->input.regionlist[index * 5 + 3] = attribute;
    tg->input.regionlist[index * 5 + 4] = max_volume;
    return TRITET_SUCCESS;
}

int tet_set_hole(HANDLE handle, int index, double x, double y, double z) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tg->input.holelist == NULL) {
        return TRITET_ERROR_NULL_HOLE_LIST;
    }
    if (index >= tg->input.numberofholes) {
        return TRITET_ERROR_INVALID_HOLE_INDEX;
    }
    tg->input.holelist[index * 3] = x;
    tg->input.holelist[index * 3 + 1] = y;
    tg->input.holelist[index * 3 + 2] = z;
    return TRITET_SUCCESS;
}

int tet_run_delaunay(HANDLE handle, int verbose) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tg->input.pointlist == NULL) {
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
        tetrahedralize(command, &tg->input, &tg->output, NULL, NULL);
    } catch (int status) {
        printf("status = %d\n", status);  // TODO
    } catch (...) {
        return 1;  // TODO
    }

    return TRITET_SUCCESS;
}

int tet_run_tetrahedralize(HANDLE handle, int verbose, int o2, double global_max_volume, double global_min_angle) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (tg->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (tg->input.facetlist == NULL) {
        return TRITET_ERROR_NULL_FACET_LIST;
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
        tetrahedralize(command, &tg->input, &tg->output, NULL, NULL);
    } catch (int status) {
        printf("status = %d\n", status);  // TODO
    } catch (...) {
        return 1;  // TODO
    }

    return TRITET_SUCCESS;
}

int tet_get_npoint(HANDLE handle) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return 0;
    }
    return tg->output.numberofpoints;
}

int tet_get_ntetrahedron(HANDLE handle) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return 0;
    }
    return tg->output.numberoftetrahedra;
}

int tet_get_ncorner(HANDLE handle) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return 0;
    }
    return tg->output.numberofcorners;
}

double tet_get_point(HANDLE handle, int index, int dim) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return 0.0;
    }
    if (index < tg->output.numberofpoints && (dim == 0 || dim == 1 || dim == 2)) {
        return tg->output.pointlist[index * 3 + dim];
    } else {
        return 0.0;
    }
}

int tet_get_tetrahedron_corner(HANDLE handle, int index, int corner) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return 0;
    }
    if (index < tg->output.numberoftetrahedra && corner < tg->output.numberofcorners) {
        return tg->output.tetrahedronlist[index * tg->output.numberofcorners + corner];
    } else {
        return 0;
    }
}

int tet_get_tetgen_attribute(HANDLE handle, int index) {
    ExtTetgenClass *tg = NULL;
    try {
        if (DATABASE.count(handle) > 0) {
            tg = DATABASE.at(handle);
        }
    } catch (...) {
    }
    if (tg == NULL) {
        return 0;
    }
    if (index < tg->output.numberoftetrahedra && tg->output.numberoftetrahedronattributes > 0) {
        return tg->output.tetrahedronattributelist[index * tg->output.numberoftetrahedronattributes];
    } else {
        return 0;
    }
}
