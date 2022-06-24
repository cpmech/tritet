#!/bin/bash

cargo valgrind run --bin mem_check_triangle_build
cargo valgrind run --bin mem_check_tetgen_build
