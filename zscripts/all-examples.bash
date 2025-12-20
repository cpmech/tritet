#!/bin/bash

for example in examples/*.rs; do
    filename="$(basename "$example")"
    filekey="${filename%%.*}"
    cargo run --example $filekey --features with_tetgen
done
