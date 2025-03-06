fn main() {
    cc::Build::new()
        .file("c_src/mycfuncs.c")
        .compile("mycfuncs");
}
