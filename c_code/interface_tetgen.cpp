#include <stdio.h>
#include <stdlib.h>

#include "tetgen.h"

extern "C" {
#include "interface_tetgen.h"
}

struct ExtTetgen* new_tetgen(int npoint, int nsegment, int nregion, int nhole) {
    struct ExtTetgen* tetgen = (struct ExtTetgen*)malloc(sizeof(struct ExtTetgen));
    if (tetgen == NULL) {
        return NULL;
    }
    tetgen->input = new tetgenio;
    tetgen->output = new tetgenio;
    return tetgen;
}

void drop_tetgen(struct ExtTetgen* tetgen) {
    if (tetgen == NULL) {
        return;
    }
    tetgenio* input = reinterpret_cast<tetgenio*>(tetgen->input);
    tetgenio* output = reinterpret_cast<tetgenio*>(tetgen->output);
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