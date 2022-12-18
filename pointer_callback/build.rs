fn main() {
    cc::Build::new().file("c_src/bridge.c").compile("bridge");
}
