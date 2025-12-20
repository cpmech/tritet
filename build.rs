use std::env;

fn compile_triangle() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if (target_os == "linux") | (target_os == "macos") {
        cc::Build::new()
            .file("c_code/triangle.c")
            .file("c_code/tricall_report.c")
            .file("c_code/interface_triangle.c")
            .flag_if_supported("-Wno-sign-compare")
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .flag_if_supported("-Wno-maybe-uninitialized")
            .flag_if_supported("-Wno-old-style-definition")
            .compile("c_code_interface_triangle");
    } else {
        cc::Build::new()
            .file("c_code/triangle.c")
            .file("c_code/tricall_report.c")
            .file("c_code/interface_triangle.c")
            .define("NO_TIMER", None)
            .compile("c_code_interface_triangle");
    }
}

#[cfg(feature = "with_tetgen")]
fn compile_tetgen() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if (target_os == "linux") | (target_os == "macos") {
        cc::Build::new()
            .cpp(true)
            .file("c_code/predicates.cxx")
            .file("c_code/tetgen.cxx")
            .file("c_code/interface_tetgen.cpp")
            .flag_if_supported("-Wno-int-to-pointer-cast")
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .flag_if_supported("-Wno-maybe-uninitialized")
            .compile("c_code_interface_tetgen");
    } else {
        cc::Build::new()
            .cpp(true)
            .file("c_code/predicates.cxx")
            .file("c_code/tetgen.cxx")
            .file("c_code/interface_tetgen.cpp")
            .compile("c_code_interface_tetgen");
    }
}

#[cfg(feature = "with_tetgen")]
fn main() {
    compile_triangle();
    compile_tetgen();
}

#[cfg(not(feature = "with_tetgen"))]
fn main() {
    compile_triangle();
}
