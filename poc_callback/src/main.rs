extern fn callback(a: i32) {
    println!("I'm called from C with value {0}", a);
}

#[link(name = "use_callback")]
extern {
   fn register_callback(cb: extern fn(i32)) -> i32;
   fn trigger_callback();
}

// make_callbackを使う
#[link(name = "make_callback")]
extern {
    fn make_callback(cb: extern fn(i32), value:i32) -> extern fn(i32);
}

fn main(){
    unsafe{


    }
}


fn _main() {
    unsafe {
        register_callback(callback);
        trigger_callback(); // Triggers the callback
    }
}