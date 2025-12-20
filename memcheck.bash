#!/bin/bash

cargo run --features with_tetgen --bin mem_check_triangle_build
cargo run --features with_tetgen --bin mem_check_tetgen_build
cargo valgrind run --features with_tetgen --bin mem_check_triangle_build
cargo valgrind run --features with_tetgen --bin mem_check_tetgen_build
