fn main(){
    cc::Build::new().file("c_src/c_main.c").compile("c_main");
    cc::Build::new().file("c_src/c_state.c").compile("c_state");
}