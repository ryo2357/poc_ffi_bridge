fn main() {
    cc::Build::new().file("c_src/bridge.c").compile("bridge");
    cc::Build::new().file("c_src/c_main.c").compile("c_main");
}
