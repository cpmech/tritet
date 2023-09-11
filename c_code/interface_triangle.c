#include "interface_triangle.h"

#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "constants.h"
#include "tricall_report.h"

void zero_triangle_data(struct triangulateio *data) {
    if (data == NULL) {
        return;
    }

    // points
    data->pointlist = NULL;
    data->pointattributelist = NULL;
    data->pointmarkerlist = NULL;
    data->numberofpoints = 0;
    data->numberofpointattributes = 0;

    // triangles
    data->trianglelist = NULL;
    data->triangleattributelist = NULL;
    data->trianglearealist = NULL;
    data->neighborlist = NULL;
    data->numberoftriangles = 0;
    data->numberofcorners = 0;
    data->numberoftriangleattributes = 0;

    // segments
    data->segmentlist = NULL;
    data->segmentmarkerlist = NULL;
    data->numberofsegments = 0;

    // holes
    data->holelist = NULL;
    data->numberofholes = 0;

    // regions
    data->regionlist = NULL;
    data->numberofregions = 0;

    // edges
    data->edgelist = NULL;
    data->edgemarkerlist = NULL;
    data->normlist = NULL;
    data->numberofedges = 0;
}

void free_triangle_data(struct triangulateio *data) {
    if (data == NULL) {
        return;
    }

    // Points
    if (data->pointlist != NULL) {
        free(data->pointlist);
    }
    if (data->pointattributelist != NULL) {
        free(data->pointattributelist);
    }
    if (data->pointmarkerlist != NULL) {
        free(data->pointmarkerlist);
    }

    // Triangles
    if (data->trianglelist != NULL) {
        free(data->trianglelist);
    }
    if (data->triangleattributelist != NULL) {
        free(data->triangleattributelist);
    }
    if (data->trianglearealist != NULL) {
        free(data->trianglearealist);
    }
    if (data->neighborlist != NULL) {
        free(data->neighborlist);
    }

    // Segments
    if (data->segmentlist != NULL) {
        free(data->segmentlist);
    }
    if (data->segmentmarkerlist != NULL) {
        free(data->segmentmarkerlist);
    }

    // Holes
    if (data->holelist != NULL) {
        free(data->holelist);
    }

    // Regions
    if (data->regionlist != NULL) {
        free(data->regionlist);
    }

    // Edges
    if (data->edgelist != NULL) {
        free(data->edgelist);
    }
    if (data->edgemarkerlist != NULL) {
        free(data->edgemarkerlist);
    }
    if (data->normlist != NULL) {
        free(data->normlist);
    }

    zero_triangle_data(data);
}

struct ExtTrigen *tri_new_trigen(int32_t npoint, int32_t nsegment, int32_t nregion, int32_t nhole) {
    if (npoint < 3) {
        return NULL;
    }

    // trigen
    struct ExtTrigen *trigen = (struct ExtTrigen *)malloc(sizeof(struct ExtTrigen));
    if (trigen == NULL) {
        return NULL;
    }
    zero_triangle_data(&trigen->input);
    zero_triangle_data(&trigen->output);
    zero_triangle_data(&trigen->voronoi);

    // points
    trigen->input.pointlist = (double *)malloc(npoint * 2 * sizeof(double));
    if (trigen->input.pointlist == NULL) {
        free(trigen);
        return NULL;
    }
    trigen->input.numberofpoints = npoint;

    // segments
    if (nsegment > 0) {
        trigen->input.segmentlist = (int32_t *)malloc(nsegment * 2 * sizeof(int32_t));
        if (trigen->input.segmentlist == NULL) {
            free_triangle_data(&trigen->input);
            free(trigen);
            return NULL;
        }
        trigen->input.segmentmarkerlist = (int32_t *)malloc(nsegment * sizeof(int32_t));
        if (trigen->input.segmentmarkerlist == NULL) {
            free_triangle_data(&trigen->input);
            free(trigen);
            return NULL;
        }
        trigen->input.numberofsegments = nsegment;
    }

    // regions
    if (nregion > 0) {
        trigen->input.regionlist = (double *)malloc(nregion * 4 * sizeof(double));
        if (trigen->input.regionlist == NULL) {
            free_triangle_data(&trigen->input);
            free(trigen);
            return NULL;
        }
        trigen->input.numberofregions = nregion;
    }

    // holes
    if (nhole > 0) {
        trigen->input.holelist = (double *)malloc(nhole * 2 * sizeof(double));
        if (trigen->input.holelist == NULL) {
            free_triangle_data(&trigen->input);
            free(trigen);
            return NULL;
        }
        trigen->input.numberofholes = nhole;
    }

    return trigen;
}

void tri_drop_trigen(struct ExtTrigen *trigen) {
    if (trigen == NULL) {
        return;
    }
    free_triangle_data(&trigen->input);
    free_triangle_data(&trigen->output);
    free_triangle_data(&trigen->voronoi);
    free(trigen);
}

int32_t tri_set_point(struct ExtTrigen *trigen, int32_t index, int32_t marker, double x, double y) {
    if (trigen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (trigen->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (index >= trigen->input.numberofpoints) {
        return TRITET_ERROR_INVALID_POINT_INDEX;
    }
    trigen->input.pointlist[index * 2] = x;
    trigen->input.pointlist[index * 2 + 1] = y;
    return TRITET_SUCCESS;
}

int32_t tri_set_segment(struct ExtTrigen *trigen, int32_t index, int32_t marker, int32_t a, int32_t b) {
    if (trigen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (trigen->input.segmentlist == NULL || trigen->input.segmentmarkerlist == NULL) {
        return TRITET_ERROR_NULL_SEGMENT_LIST;
    }
    if (index >= trigen->input.numberofsegments) {
        return TRITET_ERROR_INVALID_SEGMENT_INDEX;
    }
    if (a >= trigen->input.numberofpoints || b >= trigen->input.numberofpoints) {
        return TRITET_ERROR_INVALID_SEGMENT_POINT_ID;
    }
    trigen->input.segmentlist[index * 2] = a;
    trigen->input.segmentlist[index * 2 + 1] = b;
    trigen->input.segmentmarkerlist[index] = marker;
    return TRITET_SUCCESS;
}

int32_t tri_set_region(struct ExtTrigen *trigen, int32_t index, int32_t attribute, double x, double y, double max_area) {
    // Shewchuk: If you are using the -A and -a switches simultaneously and wish to assign an attribute
    // to some region without imposing an area constraint, use a negative maximum area.
    if (trigen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (trigen->input.regionlist == NULL) {
        return TRITET_ERROR_NULL_REGION_LIST;
    }
    if (index >= trigen->input.numberofregions) {
        return TRITET_ERROR_INVALID_REGION_INDEX;
    }
    trigen->input.regionlist[index * 4] = x;
    trigen->input.regionlist[index * 4 + 1] = y;
    trigen->input.regionlist[index * 4 + 2] = attribute;
    trigen->input.regionlist[index * 4 + 3] = max_area;
    return TRITET_SUCCESS;
}

int32_t tri_set_hole(struct ExtTrigen *trigen, int32_t index, double x, double y) {
    if (trigen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (trigen->input.holelist == NULL) {
        return TRITET_ERROR_NULL_HOLE_LIST;
    }
    if (index >= trigen->input.numberofholes) {
        return TRITET_ERROR_INVALID_HOLE_INDEX;
    }
    trigen->input.holelist[index * 2] = x;
    trigen->input.holelist[index * 2 + 1] = y;
    return TRITET_SUCCESS;
}

int32_t tri_run_delaunay(struct ExtTrigen *trigen, int32_t verbose) {
    if (trigen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (trigen->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }

    // Triangulate the points
    // Switches:
    // * `z` -- number everything from zero (z)
    char command[10];
    strcpy(command, "z");
    if (verbose == TRITET_FALSE) {
        strcat(command, "Q");
    }
    triangulate(command, &trigen->input, &trigen->output, NULL);

    // After triangulate (with -p switch), output.regionlist gets the content of input.regionlist and
    // output.holelist gets the content of input.holelist. Thus, these output variables must be set
    // to NULL in order to tell free_data to ignore them and avoid a double-free memory issue.
    trigen->output.regionlist = NULL;
    trigen->output.holelist = NULL;

    if (verbose == TRITET_TRUE) {
        report(&trigen->output, 1, 1, 0, 0, 0, 0);
    }
    return TRITET_SUCCESS;
}

int32_t tri_run_voronoi(struct ExtTrigen *trigen, int32_t verbose) {
    if (trigen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (trigen->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }

    // Triangulate the points
    // Switches:
    // * `z` -- number everything from zero (z)
    // * `v` -- Voronoi diagram
    char command[10];
    strcpy(command, "zv");
    if (verbose == TRITET_FALSE) {
        strcat(command, "Q");
    }
    triangulate(command, &trigen->input, &trigen->output, &trigen->voronoi);

    // After triangulate (with -p switch), output.regionlist gets the content of input.regionlist and
    // output.holelist gets the content of input.holelist. Thus, these output variables must be set
    // to NULL in order to tell free_data to ignore them and avoid a double-free memory issue.
    trigen->output.regionlist = NULL;
    trigen->output.holelist = NULL;

    if (verbose == TRITET_TRUE) {
        report(&trigen->voronoi, 0, 0, 0, 0, 1, 1);
    }
    return TRITET_SUCCESS;
}

int32_t tri_run_triangulate(struct ExtTrigen *trigen, int32_t verbose, int32_t quadratic, int32_t allow_new_points_on_bry, double global_max_area, double global_min_angle) {
    if (trigen == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (trigen->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (trigen->input.segmentlist == NULL) {
        return TRITET_ERROR_NULL_SEGMENT_LIST;
    }

    // Generate mesh
    // Switches:
    // * `p` -- write a PSLG (p)
    // * `z` -- number everything from zero (z)
    // * `A` -- assign a regional attribute to each element (A)
    // * `Q` -- quiet mode
    // * `o2` -- generates second-order elements with six nodes each
    // * `Y` -- prohibits the insertion of Steiner points on the mesh boundary
    char command[128];
    // strcpy(command, "pzAY");
    strcpy(command, "pzA");
    if (verbose == TRITET_FALSE) {
        strcat(command, "Q");
    }
    if (quadratic == TRITET_TRUE) {
        strcat(command, "o2");
    }
    if (allow_new_points_on_bry == TRITET_FALSE) {
        strcat(command, "Y");
    }
    if (global_max_area > 0.0) {
        char buf[32];
        int32_t n = snprintf(buf, 32, "a%.15f", global_max_area);
        if (n >= 32) {
            return TRITET_ERROR_STRING_CONCAT;
        }
        strcat(command, buf);
    }
    if (global_min_angle > 0.0) {
        char buf[32];
        int32_t n = snprintf(buf, 32, "q%.15f", global_min_angle);
        if (n >= 32) {
            return TRITET_ERROR_STRING_CONCAT;
        }
        strcat(command, buf);
    } else {
        strcat(command, "q");
    }
    triangulate(command, &trigen->input, &trigen->output, NULL);

    // After triangulate (with -p switch), output.regionlist gets the content of input.regionlist and
    // output.holelist gets the content of input.holelist. Thus, these output variables must be set
    // to NULL in order to tell free_data to ignore them and avoid a double-free memory issue.
    trigen->output.regionlist = NULL;
    trigen->output.holelist = NULL;

    if (verbose == TRITET_TRUE) {
        report(&trigen->output, 1, 1, 0, 0, 0, 0);
    }
    return TRITET_SUCCESS;
}

int32_t tri_out_npoint(struct ExtTrigen *trigen) {
    if (trigen == NULL) {
        return 0;
    }
    return trigen->output.numberofpoints;
}

int32_t tri_out_nsegment(struct ExtTrigen *trigen) {
    if (trigen == NULL) {
        return 0;
    }
    return trigen->output.numberofsegments;
}

int32_t tri_out_ncell(struct ExtTrigen *trigen) {
    if (trigen == NULL) {
        return 0;
    }
    return trigen->output.numberoftriangles;
}

int32_t tri_out_cell_npoint(struct ExtTrigen *trigen) {
    if (trigen == NULL) {
        return 0;
    }
    return trigen->output.numberofcorners;
}

double tri_out_point(struct ExtTrigen *trigen, int32_t index, int32_t dim) {
    if (trigen == NULL) {
        return 0.0;
    }
    if (index < trigen->output.numberofpoints && (dim == 0 || dim == 1)) {
        return trigen->output.pointlist[index * 2 + dim];
    } else {
        return 0.0;
    }
}

int32_t tri_out_segment_point(struct ExtTrigen *trigen, int32_t index, int32_t side) {
    if (trigen == NULL) {
        return 0;
    }
    if (index < trigen->output.numberofsegments && (side == 0 || side == 1)) {
        return trigen->output.segmentlist[index * 2 + side];
    } else {
        return 0;
    }
}

int32_t tri_out_segment_marker(struct ExtTrigen *trigen, int32_t index) {
    if (trigen == NULL) {
        return 0;
    }
    if (index < trigen->output.numberofsegments) {
        return trigen->output.segmentmarkerlist[index];
    } else {
        return 0;
    }
}

int32_t tri_out_cell_point(struct ExtTrigen *trigen, int32_t index, int32_t corner) {
    if (trigen == NULL) {
        return 0;
    }
    if (index < trigen->output.numberoftriangles && corner < trigen->output.numberofcorners) {
        return trigen->output.trianglelist[index * trigen->output.numberofcorners + corner];
    } else {
        return 0;
    }
}

int32_t tri_out_cell_attribute(struct ExtTrigen *trigen, int32_t index) {
    if (trigen == NULL) {
        return 0;
    }
    if (index < trigen->output.numberoftriangles && trigen->output.numberoftriangleattributes > 0) {
        return trigen->output.triangleattributelist[index * trigen->output.numberoftriangleattributes];
    } else {
        return 0;
    }
}

int32_t tri_out_voronoi_npoint(struct ExtTrigen *trigen) {
    if (trigen == NULL) {
        return 0;
    }
    return trigen->voronoi.numberofpoints;
}

int32_t tri_out_voronoi_point(struct ExtTrigen *trigen, int32_t index, int32_t dim) {
    if (trigen == NULL) {
        return 0.0;
    }
    if (index < trigen->voronoi.numberofpoints && (dim == 0 || dim == 1)) {
        return trigen->voronoi.pointlist[index * 2 + dim];
    } else {
        return 0.0;
    }
}

int32_t tri_out_voronoi_nedge(struct ExtTrigen *trigen) {
    if (trigen == NULL) {
        return 0;
    }
    return trigen->voronoi.numberofedges;
}

int32_t tri_out_voronoi_edge_point(struct ExtTrigen *trigen, int32_t index, int32_t side) {
    if (trigen == NULL) {
        return 0;
    }
    if (index < trigen->voronoi.numberofedges && (side == 0 || side == 1)) {
        return trigen->voronoi.edgelist[index * 2 + side];
    } else {
        return 0;
    }
}

double tri_out_voronoi_edge_point_b_direction(struct ExtTrigen *trigen, int32_t index, int32_t dim) {
    if (trigen == NULL) {
        return 0.0;
    }
    if (index < trigen->voronoi.numberofedges && (dim == 0 || dim == 1)) {
        if (trigen->voronoi.edgelist[index * 2 + 1] == -1) {
            return trigen->voronoi.normlist[index * 2 + dim];
        } else {
            return 0.0;
        }
    } else {
        return 0.0;
    }
}
