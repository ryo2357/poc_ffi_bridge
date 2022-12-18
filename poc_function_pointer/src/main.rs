use std::os::raw::c_void;

// kind="static"とすることで静的リンクライブラリをリンクできる
#[link(name = "function_pointer", kind = "static")]
extern "C" {
    fn c_main() -> c_void;
}

fn main() {
    unsafe {
        c_main();
    }
}
