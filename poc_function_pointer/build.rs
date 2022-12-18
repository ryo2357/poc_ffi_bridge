fn main() {
    cc::Build::new()
        .file("c_src/function_pointer.c")
        .compile("function_pointer");
}
