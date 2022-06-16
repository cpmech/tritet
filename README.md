# Triangle and tetrahedron mesh generators

This project presents a Rust code to generate triangle and tetrahedron meshes by calling [Triangle](https://www.cs.cmu.edu/~quake/triangle.html)
and [Tetgen](http://tetgen.org/). However, the code here does not create a one-to-one interface to these two awesome libraries. Also, this create
tries to keep it as simple as possible.

One important aspect of this crate is that all the data structures accessed by the C-code is allocated on the "C-side", by (carefully) using
"malloc." Therefore, there is no "pointers" going forth and back from Rust to C. We then make use of [Valgrind](https://valgrind.org/) and tests
to make sure all is (hopefully) all right.

The resulting Rust interface to Triangle and Tetgen is a lightweight and low-level set of functions that could be used in other more
"high-level" projects.

... 🚧 This is a work in progress 🚧 ...
