#ifndef INTERFACE_TETGEN_H
#define INTERFACE_TETGEN_H

struct ExtTetgen {
    void* input;   // pointer to tetgenio
    void* output;  // pointer to tetgenio
};

struct ExtTetgen* new_tetgen(int npoint, int nsegment, int nregion, int nhole);

void drop_tetgen(struct ExtTetgen* tetgen);

#endif  // INTERFACE_TETGEN_H