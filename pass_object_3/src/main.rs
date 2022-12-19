#[derive(Debug)]
#[repr(C)]
struct RustObject {
    pub a: i32,
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
        self.a = v;
        self.state.push(v);
    }
}

extern "C" fn main_callback(data: i32) {
    // c_main(LJX通信ライブラリ想定)から呼ばれるcallback
    println!("I'm called from c_main with value {0}", data);
    unsafe {
        let target = get_struct();
        (*target).add(data);
    }
}


#[link(name = "c_main", kind = "static")]
extern  {
    fn main_init(cb:extern fn(i32), first_value:i32);
    fn trigger_main();
}

#[link(name = "c_state", kind = "static")]
extern  {
    fn set_struct(target: *mut RustObject);
    fn get_struct() -> *mut RustObject;
}

fn main() {
    let mut rust_object = Box::new(RustObject::new(5));

    unsafe {
        set_struct(&mut *rust_object);
        main_init(main_callback,8);

        // error: process didn't exit successfully: `target\debug\pass_object_2.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
        trigger_main();
        trigger_main();
        trigger_main();
        trigger_main();
    }

    println!("rust_object:{:?}",rust_object);

}