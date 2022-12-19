#[derive(Debug)]
struct RustObject {
    a: i32,
    state:Vec<i32>,
    // Other members...
}
impl RustObject {
    fn new(val:i32)-> Self{
        let mut state = Vec::<i32>::new();
        state.push(val);
        Self{
            a:val,
            state:state,
        }

    }

    fn add(&mut self,v:i32) {
        self.state.push(v)
    }
}

#[no_mangle]
unsafe extern "C" fn callback(target: *mut RustObject, a: i32) {
    println!("I'm called from C with value {0}", a);
    unsafe {
        // Update the value in RustObject with the value received from the callback:
        (*target).a = a;
        (*target).add(a);
    }
}

#[link(name = "bridge")]
extern {
    fn register_callback(target: *mut RustObject,
                        cb: unsafe extern fn(*mut RustObject, i32)) -> unsafe extern "C" fn(i32);
    fn trigger_callback(val:i32);
}
#[link(name = "c_main")]
extern {
    fn init(cb:unsafe extern fn(i32), first_value:i32);
    fn trigger_main();
}

fn main() {
    // Create the object that will be referenced in the callback:
    let mut rust_object = Box::new(RustObject::new(5));

    unsafe {
        let c_cb = register_callback(&mut *rust_object, callback);
        c_cb(5);
        c_cb(6);
        c_cb(8);

        println!("test");

        init(c_cb,9);
    
        trigger_main();
        trigger_main();
        trigger_main();
        trigger_main();
    }

    println!("rust_object:{:?}",rust_object);
}