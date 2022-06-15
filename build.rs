fn main() {
    cc::Build::new()
        .file("c_code/main.c")
        .file("c_code/triangle.c")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .compile("c_code_main");
}
