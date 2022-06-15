#ifndef RUST_INTERFACE_TRIANGLE_H
#define RUST_INTERFACE_TRIANGLE_H

#include <stdio.h>
#include <stdlib.h>

#define REAL double
#define ANSI_DECLARATORS
#define VOID int
#include "triangle.h"
#undef REAL
#undef ANSI_DECLARATORS
#undef VOID

struct ExtTriangle {
    struct triangulateio generator;
};

void set_all_null(struct triangulateio *generator) {
    // points
    generator->pointlist = NULL;
    generator->pointattributelist = NULL;
    generator->pointmarkerlist = NULL;
    generator->numberofpoints = 0;
    generator->numberofpointattributes = 0;

    // triangles
    generator->trianglelist = NULL;
    generator->triangleattributelist = NULL;
    generator->trianglearealist = NULL;
    generator->neighborlist = NULL;
    generator->numberoftriangles = 0;
    generator->numberofcorners = 0;
    generator->numberoftriangleattributes = 0;

    // segments
    generator->segmentlist = NULL;
    generator->segmentmarkerlist = NULL;
    generator->numberofsegments = 0;

    // holes
    generator->holelist = NULL;
    generator->numberofholes = 0;

    // regions
    generator->regionlist = NULL;
    generator->numberofregions = 0;

    // edges
    generator->edgelist = NULL;
    generator->edgemarkerlist = NULL;
    generator->normlist = NULL;
    generator->numberofedges = 0;
}

void free_generator(struct triangulateio *generator) {
    // Points
    if (generator->pointlist != NULL) {
        free(generator->pointlist);
    }
    if (generator->pointattributelist != NULL) {
        free(generator->pointattributelist);
    }
    if (generator->pointmarkerlist != NULL) {
        free(generator->pointmarkerlist);
    }

    // Triangles
    if (generator->trianglelist != NULL) {
        free(generator->trianglelist);
    }
    if (generator->triangleattributelist != NULL) {
        free(generator->triangleattributelist);
    }
    if (generator->trianglearealist != NULL) {
        free(generator->trianglearealist);
    }
    if (generator->neighborlist != NULL) {
        free(generator->neighborlist);
    }

    // Segments
    if (generator->segmentlist != NULL) {
        free(generator->segmentlist);
    }
    if (generator->segmentmarkerlist != NULL) {
        free(generator->segmentmarkerlist);
    }

    // Holes
    if (generator->holelist != NULL) {
        free(generator->holelist);
    }

    // Regions
    if (generator->regionlist != NULL) {
        free(generator->regionlist);
    }

    // Edges
    if (generator->edgelist != NULL) {
        free(generator->edgelist);
    }
    if (generator->edgemarkerlist != NULL) {
        free(generator->edgemarkerlist);
    }
    if (generator->normlist != NULL) {
        free(generator->normlist);
    }

    set_all_null(generator);
}

struct ExtTriangle *new_triangle(int npoint, int nsegment, int nregion, int nhole) {
    if (npoint < 3) {
        return NULL;
    }

    if (nsegment < 3) {
        return NULL;
    }

    struct ExtTriangle *triangle = (struct ExtTriangle *)malloc(sizeof(struct ExtTriangle));

    if (triangle == NULL) {
        return NULL;
    }
    set_all_null(&triangle->generator);

    // points
    triangle->generator.pointlist = (double *)malloc(npoint * 2 * sizeof(double));
    if (triangle->generator.pointlist == NULL) {
        free(triangle);
        return NULL;
    }
    triangle->generator.numberofpoints = npoint;

    // segments
    triangle->generator.segmentlist = (int *)malloc(nsegment * 2 * sizeof(int));
    if (triangle->generator.segmentlist == NULL) {
        free(triangle->generator.pointlist);
        free(triangle);
        return NULL;
    }
    triangle->generator.numberofsegments = nsegment;

    // regions
    if (nregion > 0) {
        triangle->generator.regionlist = (double *)malloc(nregion * 4 * sizeof(double));
        if (triangle->generator.regionlist == NULL) {
            free(triangle->generator.segmentlist);
            free(triangle->generator.pointlist);
            free(triangle);
            return NULL;
        }
        triangle->generator.numberofregions = nregion;
    }

    // holes
    if (nhole > 0) {
        triangle->generator.holelist = (double *)malloc(nhole * 2 * sizeof(double));
        if (triangle->generator.holelist == NULL) {
            if (triangle->generator.regionlist != NULL) {
                free(triangle->generator.regionlist);
            }
            free(triangle->generator.segmentlist);
            free(triangle->generator.pointlist);
            free(triangle);
            return NULL;
        }
        triangle->generator.numberofholes = nhole;
    }

    return triangle;
}

void drop_triangle(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return;
    }
    free_generator(&triangle->generator);
    free(triangle);
}

#endif  // RUST_INTERFACE_TRIANGLE_H
