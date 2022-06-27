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

struct ExtTriangle *new_triangle(int32_t npoint, int32_t nsegment, int32_t nregion, int32_t nhole) {
    if (npoint < 3) {
        return NULL;
    }

    // triangle
    struct ExtTriangle *triangle = (struct ExtTriangle *)malloc(sizeof(struct ExtTriangle));
    if (triangle == NULL) {
        return NULL;
    }
    zero_triangle_data(&triangle->input);
    zero_triangle_data(&triangle->output);
    zero_triangle_data(&triangle->voronoi);

    // points
    triangle->input.pointlist = (double *)malloc(npoint * 2 * sizeof(double));
    if (triangle->input.pointlist == NULL) {
        free(triangle);
        return NULL;
    }
    triangle->input.numberofpoints = npoint;

    // segments
    if (nsegment > 0) {
        triangle->input.segmentlist = (int32_t *)malloc(nsegment * 2 * sizeof(int32_t));
        if (triangle->input.segmentlist == NULL) {
            free_triangle_data(&triangle->input);
            free(triangle);
            return NULL;
        }
        triangle->input.numberofsegments = nsegment;
    }

    // regions
    if (nregion > 0) {
        triangle->input.regionlist = (double *)malloc(nregion * 4 * sizeof(double));
        if (triangle->input.regionlist == NULL) {
            free_triangle_data(&triangle->input);
            free(triangle);
            return NULL;
        }
        triangle->input.numberofregions = nregion;
    }

    // holes
    if (nhole > 0) {
        triangle->input.holelist = (double *)malloc(nhole * 2 * sizeof(double));
        if (triangle->input.holelist == NULL) {
            free_triangle_data(&triangle->input);
            free(triangle);
            return NULL;
        }
        triangle->input.numberofholes = nhole;
    }

    return triangle;
}

void drop_triangle(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return;
    }
    free_triangle_data(&triangle->input);
    free_triangle_data(&triangle->output);
    free_triangle_data(&triangle->voronoi);
    free(triangle);
}

int32_t set_point(struct ExtTriangle *triangle, int32_t index, double x, double y) {
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (index >= triangle->input.numberofpoints) {
        return TRITET_ERROR_INVALID_POINT_INDEX;
    }
    triangle->input.pointlist[index * 2] = x;
    triangle->input.pointlist[index * 2 + 1] = y;
    return TRITET_SUCCESS;
}

int32_t set_segment(struct ExtTriangle *triangle, int32_t index, int32_t a, int32_t b) {
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.segmentlist == NULL) {
        return TRITET_ERROR_NULL_SEGMENT_LIST;
    }
    if (index >= triangle->input.numberofsegments) {
        return TRITET_ERROR_INVALID_SEGMENT_INDEX;
    }
    if (a >= triangle->input.numberofpoints || b >= triangle->input.numberofpoints) {
        return TRITET_ERROR_INVALID_SEGMENT_POINT_ID;
    }
    triangle->input.segmentlist[index * 2] = a;
    triangle->input.segmentlist[index * 2 + 1] = b;
    return TRITET_SUCCESS;
}

int32_t set_region(struct ExtTriangle *triangle, int32_t index, double x, double y, int32_t attribute, double max_area) {
    // Shewchuk: If you are using the -A and -a switches simultaneously and wish to assign an attribute
    // to some region without imposing an area constraint, use a negative maximum area.
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.regionlist == NULL) {
        return TRITET_ERROR_NULL_REGION_LIST;
    }
    if (index >= triangle->input.numberofregions) {
        return TRITET_ERROR_INVALID_REGION_INDEX;
    }
    triangle->input.regionlist[index * 4] = x;
    triangle->input.regionlist[index * 4 + 1] = y;
    triangle->input.regionlist[index * 4 + 2] = attribute;
    triangle->input.regionlist[index * 4 + 3] = max_area;
    return TRITET_SUCCESS;
}

int32_t set_hole(struct ExtTriangle *triangle, int32_t index, double x, double y) {
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.holelist == NULL) {
        return TRITET_ERROR_NULL_HOLE_LIST;
    }
    if (index >= triangle->input.numberofholes) {
        return TRITET_ERROR_INVALID_HOLE_INDEX;
    }
    triangle->input.holelist[index * 2] = x;
    triangle->input.holelist[index * 2 + 1] = y;
    return TRITET_SUCCESS;
}

int32_t run_delaunay(struct ExtTriangle *triangle, int32_t verbose) {
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.pointlist == NULL) {
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
    triangulate(command, &triangle->input, &triangle->output, NULL);

    // After triangulate (with -p switch), output.regionlist gets the content of input.regionlist and
    // output.holelist gets the content of input.holelist. Thus, these output variables must be set
    // to NULL in order to tell free_data to ignore them and avoid a double-free memory issue.
    triangle->output.regionlist = NULL;
    triangle->output.holelist = NULL;

    if (verbose == TRITET_TRUE) {
        report(&triangle->output, 1, 1, 0, 0, 0, 0);
    }
    return TRITET_SUCCESS;
}

int32_t run_voronoi(struct ExtTriangle *triangle, int32_t verbose) {
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.pointlist == NULL) {
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
    triangulate(command, &triangle->input, &triangle->output, &triangle->voronoi);

    // After triangulate (with -p switch), output.regionlist gets the content of input.regionlist and
    // output.holelist gets the content of input.holelist. Thus, these output variables must be set
    // to NULL in order to tell free_data to ignore them and avoid a double-free memory issue.
    triangle->output.regionlist = NULL;
    triangle->output.holelist = NULL;

    if (verbose == TRITET_TRUE) {
        report(&triangle->voronoi, 0, 0, 0, 0, 1, 1);
    }
    return TRITET_SUCCESS;
}

int32_t run_triangulate(struct ExtTriangle *triangle, int32_t verbose, int32_t quadratic, double global_max_area, double global_min_angle) {
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.pointlist == NULL) {
        return TRITET_ERROR_NULL_POINT_LIST;
    }
    if (triangle->input.segmentlist == NULL) {
        return TRITET_ERROR_NULL_SEGMENT_LIST;
    }

    // Generate mesh
    // Switches:
    // * `p` -- write a PSLG (p)
    // * `z` -- number everything from zero (z)
    // * `A` -- assign a regional attribute to each element (A)
    char command[128];
    strcpy(command, "pzA");
    if (verbose == TRITET_FALSE) {
        strcat(command, "Q");
    }
    if (quadratic == TRITET_TRUE) {
        strcat(command, "o2");
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
    triangulate(command, &triangle->input, &triangle->output, NULL);

    // After triangulate (with -p switch), output.regionlist gets the content of input.regionlist and
    // output.holelist gets the content of input.holelist. Thus, these output variables must be set
    // to NULL in order to tell free_data to ignore them and avoid a double-free memory issue.
    triangle->output.regionlist = NULL;
    triangle->output.holelist = NULL;

    if (verbose == TRITET_TRUE) {
        report(&triangle->output, 1, 1, 0, 0, 0, 0);
    }
    return TRITET_SUCCESS;
}

int32_t get_npoint(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return 0;
    }
    return triangle->output.numberofpoints;
}

int32_t get_ntriangle(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return 0;
    }
    return triangle->output.numberoftriangles;
}

int32_t get_ncorner(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return 0;
    }
    return triangle->output.numberofcorners;
}

double get_point(struct ExtTriangle *triangle, int32_t index, int32_t dim) {
    if (triangle == NULL) {
        return 0.0;
    }
    if (index < triangle->output.numberofpoints && (dim == 0 || dim == 1)) {
        return triangle->output.pointlist[index * 2 + dim];
    } else {
        return 0.0;
    }
}

int32_t get_triangle_corner(struct ExtTriangle *triangle, int32_t index, int32_t corner) {
    if (triangle == NULL) {
        return 0;
    }
    if (index < triangle->output.numberoftriangles && corner < triangle->output.numberofcorners) {
        return triangle->output.trianglelist[index * triangle->output.numberofcorners + corner];
    } else {
        return 0;
    }
}

int32_t get_triangle_attribute(struct ExtTriangle *triangle, int32_t index) {
    if (triangle == NULL) {
        return 0;
    }
    if (index < triangle->output.numberoftriangles && triangle->output.numberoftriangleattributes > 0) {
        return triangle->output.triangleattributelist[index * triangle->output.numberoftriangleattributes];
    } else {
        return 0;
    }
}

int32_t get_voronoi_npoint(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return 0;
    }
    return triangle->voronoi.numberofpoints;
}

int32_t get_voronoi_point(struct ExtTriangle *triangle, int32_t index, int32_t dim) {
    if (triangle == NULL) {
        return 0.0;
    }
    if (index < triangle->voronoi.numberofpoints && (dim == 0 || dim == 1)) {
        return triangle->voronoi.pointlist[index * 2 + dim];
    } else {
        return 0.0;
    }
}

int32_t get_voronoi_nedge(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return 0;
    }
    return triangle->voronoi.numberofedges;
}

int32_t get_voronoi_edge_point(struct ExtTriangle *triangle, int32_t index, int32_t side) {
    if (triangle == NULL) {
        return 0;
    }
    if (index < triangle->voronoi.numberofedges && (side == 0 || side == 1)) {
        return triangle->voronoi.edgelist[index * 2 + side];
    } else {
        return 0;
    }
}

double get_voronoi_edge_point_b_direction(struct ExtTriangle *triangle, int32_t index, int32_t dim) {
    if (triangle == NULL) {
        return 0.0;
    }
    if (index < triangle->voronoi.numberofedges && (dim == 0 || dim == 1)) {
        if (triangle->voronoi.edgelist[index * 2 + 1] == -1) {
            return triangle->voronoi.normlist[index * 2 + dim];
        } else {
            return 0.0;
        }
    } else {
        return 0.0;
    }
}
