#ifndef INTERFACE_TRIANGLE_H
#define INTERFACE_TRIANGLE_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define REAL double
#define ANSI_DECLARATORS
#define VOID int
#include "triangle.h"
#undef REAL
#undef ANSI_DECLARATORS
#undef VOID

#include "constants.h"
#include "tricall_report.h"

struct ExtTriangle {
    struct triangulateio input;
    struct triangulateio output;
    struct triangulateio voronoi;
};

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

struct ExtTriangle *new_triangle(int npoint, int nsegment, int nregion, int nhole) {
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
        triangle->input.segmentlist = (int *)malloc(nsegment * 2 * sizeof(int));
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

int set_point(struct ExtTriangle *triangle, int index, double x, double y) {
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

int set_segment(struct ExtTriangle *triangle, int index, int a, int b) {
    if (triangle == NULL) {
        return TRITET_ERROR_NULL_DATA;
    }
    if (triangle->input.segmentlist == NULL) {
        return TRITET_ERROR_NULL_SEGMENT_LIST;
    }
    if (index >= triangle->input.numberofsegments) {
        return TRITET_ERROR_INVALID_SEGMENT_INDEX;
    }
    triangle->input.segmentlist[index * 2] = a;
    triangle->input.segmentlist[index * 2 + 1] = b;
    return TRITET_SUCCESS;
}

int set_region(struct ExtTriangle *triangle, int index, double x, double y, int attribute, double max_area) {
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

int set_hole(struct ExtTriangle *triangle, int index, double x, double y) {
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

int run_delaunay(struct ExtTriangle *triangle, int verbose) {
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

int run_voronoi(struct ExtTriangle *triangle, int verbose) {
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

int run_triangulate(struct ExtTriangle *triangle, int verbose, int quadratic, double global_max_area, double global_min_angle) {
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
        int n = snprintf(buf, 32, "a%.15f", global_max_area);
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

int get_npoint(struct ExtTriangle *triangle) {
    return triangle->output.numberofpoints;
}

int get_ntriangle(struct ExtTriangle *triangle) {
    return triangle->output.numberoftriangles;
}

int get_ncorner(struct ExtTriangle *triangle) {
    return triangle->output.numberofcorners;
}

double get_point_x(struct ExtTriangle *triangle, int index) {
    if (index < triangle->output.numberofpoints) {
        return triangle->output.pointlist[index * 2];
    } else {
        return 0.0;
    }
}

double get_point_y(struct ExtTriangle *triangle, int index) {
    if (index < triangle->output.numberofpoints) {
        return triangle->output.pointlist[index * 2 + 1];
    } else {
        return 0.0;
    }
}

int get_triangle_corner(struct ExtTriangle *triangle, int index, int corner) {
    if (index < triangle->output.numberoftriangles && corner < triangle->output.numberofcorners) {
        return triangle->output.trianglelist[index * triangle->output.numberofcorners + corner];
    } else {
        return 0;
    }
}

int get_voronoi_npoint(struct ExtTriangle *triangle) {
    return triangle->voronoi.numberofpoints;
}

int get_voronoi_point_x(struct ExtTriangle *triangle, int index) {
    if (index < triangle->voronoi.numberofpoints) {
        return triangle->voronoi.pointlist[index * 2];
    } else {
        return 0.0;
    }
}

int get_voronoi_point_y(struct ExtTriangle *triangle, int index) {
    if (index < triangle->voronoi.numberofpoints) {
        return triangle->voronoi.pointlist[index * 2 + 1];
    } else {
        return 0.0;
    }
}

int get_voronoi_nedge(struct ExtTriangle *triangle) {
    return triangle->voronoi.numberofedges;
}

int get_voronoi_edge_point_a(struct ExtTriangle *triangle, int index) {
    if (index < triangle->voronoi.numberofedges) {
        return triangle->voronoi.edgelist[index * 2];
    } else {
        return 0;
    }
}

int get_voronoi_edge_point_b(struct ExtTriangle *triangle, int index) {
    if (index < triangle->voronoi.numberofedges) {
        return triangle->voronoi.edgelist[index * 2 + 1];
    } else {
        return 0;
    }
}

double get_voronoi_edge_point_b_direction_x(struct ExtTriangle *triangle, int index) {
    if (index < triangle->voronoi.numberofedges) {
        if (triangle->voronoi.edgelist[index * 2 + 1] == -1) {
            return triangle->voronoi.normlist[index * 2];
        } else {
            return 0.0;
        }
    } else {
        return 0.0;
    }
}

double get_voronoi_edge_point_b_direction_y(struct ExtTriangle *triangle, int index) {
    if (index < triangle->voronoi.numberofedges) {
        if (triangle->voronoi.edgelist[index * 2 + 1] == -1) {
            return triangle->voronoi.normlist[index * 2 + 1];
        } else {
            return 0.0;
        }
    } else {
        return 0.0;
    }
}

#endif  // INTERFACE_TRIANGLE_H
