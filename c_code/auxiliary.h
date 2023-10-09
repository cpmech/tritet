#pragma once

#include <inttypes.h>

inline void sort_3(int *a, int *b, int *c) {
    int temp;
    if (*b < *a) {
        // a, b = b, a
        temp = *a;
        *a = *b;
        *b = temp;
    }
    if (*c < *b) {
        // b, c = c, b
        temp = *b;
        *b = *c;
        *c = temp;
    }
    if (*b < *a) {
        // a, b = b, a
        temp = *a;
        *a = *b;
        *b = temp;
    }
}
