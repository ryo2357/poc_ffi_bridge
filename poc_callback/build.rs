fn main(){
    cc::Build::new().file("c_src/use_callback.c").compile("use_callback");
    cc::Build::new().file("c_src/make_callback.c").compile("make_callback");
}