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

struct ExtTriangle *new_triangle() {
    struct ExtTriangle *triangle = (struct ExtTriangle *)malloc(sizeof(struct ExtTriangle));

    if (triangle == NULL) {
        return NULL;
    }

    return triangle;
}

void drop_triangle(struct ExtTriangle *triangle) {
    if (triangle == NULL) {
        return;
    }
    free(triangle);
}

#endif  // RUST_INTERFACE_TRIANGLE_H
