#!/bin/bash

cargo run --bin mem_check_triangle_build
cargo run --bin mem_check_tetgen_build
cargo valgrind run --bin mem_check_triangle_build
cargo valgrind run --bin mem_check_tetgen_build
