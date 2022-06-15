#ifndef INTERFACE_TRIANGLE_H
#define INTERFACE_TRIANGLE_H

#include <stdio.h>
#include <stdlib.h>

#define REAL double
#define ANSI_DECLARATORS
#define VOID int
#include "triangle.h"
#undef REAL
#undef ANSI_DECLARATORS
#undef VOID

const int TRUE = 1;
const int FALSE = 0;

struct ExtTriangle {
    struct triangulateio input;
    struct triangulateio output;
};

void set_all_null(struct triangulateio *data) {
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

void free_data(struct triangulateio *data) {
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

    set_all_null(data);
}

struct ExtTriangle *new_triangle(int npoint, int nsegment, int nregion, int nhole) {
    // triangle
    struct ExtTriangle *triangle = (struct ExtTriangle *)malloc(sizeof(struct ExtTriangle));
    if (triangle == NULL) {
        return NULL;
    }
    set_all_null(&triangle->input);
    set_all_null(&triangle->output);

    // points
    triangle->input.pointlist = (double *)malloc(npoint * 2 * sizeof(double));
    if (triangle->input.pointlist == NULL) {
        free(triangle);
        return NULL;
    }
    triangle->input.numberofpoints = npoint;

    // segments
    triangle->input.segmentlist = (int *)malloc(nsegment * 2 * sizeof(int));
    if (triangle->input.segmentlist == NULL) {
        free(triangle->input.pointlist);
        free(triangle);
        return NULL;
    }
    triangle->input.numberofsegments = nsegment;

    // regions
    if (nregion > 0) {
        triangle->input.regionlist = (double *)malloc(nregion * 4 * sizeof(double));
        if (triangle->input.regionlist == NULL) {
            free(triangle->input.segmentlist);
            free(triangle->input.pointlist);
            free(triangle);
            return NULL;
        }
        triangle->input.numberofregions = nregion;
    }

    // holes
    if (nhole > 0) {
        triangle->input.holelist = (double *)malloc(nhole * 2 * sizeof(double));
        if (triangle->input.holelist == NULL) {
            if (triangle->input.regionlist != NULL) {
                free(triangle->input.regionlist);
            }
            free(triangle->input.segmentlist);
            free(triangle->input.pointlist);
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
    free_data(&triangle->input);
    free_data(&triangle->output);
    free(triangle);
}

void set_point(struct ExtTriangle *triangle, int index, double x, double y) {
    triangle->input.pointlist[index * 2] = x;
    triangle->input.pointlist[index * 2 + 1] = y;
}

void set_segment(struct ExtTriangle *triangle, int index, int left, int right) {
    triangle->input.pointlist[index * 2] = left;
    triangle->input.pointlist[index * 2 + 1] = right;
}

void set_region(struct ExtTriangle *triangle, int index, double x, double y, int attribute, double max_area) {
    triangle->input.regionlist[index * 4] = x;
    triangle->input.regionlist[index * 4 + 1] = y;
    triangle->input.regionlist[index * 4 + 2] = attribute;
    triangle->input.regionlist[index * 4 + 3] = max_area;
}

void set_hole(struct ExtTriangle *triangle, int index, double x, double y) {
    triangle->input.holelist[index * 2] = x;
    triangle->input.holelist[index * 2 + 1] = y;
}

void generate(struct ExtTriangle *triangle, int quiet, int quadratic, double global_max_area, double global_min_angle) {
    // Triangulate the points
    // Switches:
    // * `p` -- write a PSLG (p)
    // * `c` -- preserve the convex hull (c)
    // * `z` -- number everything from zero (z)
    // * `A` -- assign a regional attribute to each element (A)
    if (quiet == TRUE) {
        // todo
    }
    triangulate("pczA", &triangle->input, &triangle->output, NULL);
    // After triangulate (with -p switch), output.regionlist gets the content of input.regionlist and
    // output.holelist gets the content of input.holelist. Thus, these output variables must be set
    // to NULL in order to tell free_data to ignore them and avoid a double-free memory issue.
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
    return triangle->output.pointlist[index * 2];
}

double get_point_y(struct ExtTriangle *triangle, int index) {
    return triangle->output.pointlist[index * 2 + 1];
}

int get_triangle_corner(struct ExtTriangle *triangle, int index, int corner) {
    return triangle->output.trianglelist[index * triangle->output.numberofcorners + corner];
}

#endif  // INTERFACE_TRIANGLE_H
