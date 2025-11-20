#!/bin/bash

cargo clean
cargo build --bin tetgen2msh --release
cargo build --bin trigen2msh --release

sudo cp ~/rust_modules/release/tetgen2msh /usr/local/bin/
sudo cp ~/rust_modules/release/trigen2msh /usr/local/bin/

echo "Installed tetgen2msh and trigen2msh to /usr/local/bin/"
