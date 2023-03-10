// FFI definition (normally it's defined in a *-sys crate)
mod ffi {
    extern crate libc;

    // cf. https://doc.rust-lang.org/book/ffi.html#representing-opaque-structs
    pub enum MyStruct {}

    extern "C" {
        pub fn mystruct_new() -> *mut MyStruct;
        pub fn mystruct_free(myStruct: *mut MyStruct) -> *mut MyStruct;
        pub fn mystruct_set_number(myStruct: *mut MyStruct, number: i32);
        pub fn mystruct_hello(myStruct: *mut MyStruct);
        pub fn mystruct_set_callback(
            mystruct: *mut MyStruct,
            func: extern "C" fn(*mut MyStruct, i32, *mut libc::c_void) -> i32,
            callback_data: *mut libc::c_void,
        );
    }
}

extern crate libc;

// wrapper part

// Rust世界のコード用のデータを入れておく「内側」
// 「内側」をBox（ヒープ）で持つことが出来、ポインタに変換してC関数のポインタに渡すことができる。
struct MyStructInner {
    callback: Box<dyn Fn(&MyStructInner, i32) -> i32>,
}
// 「内側」とC関数からもらったポインタを格納しておく「外側」
struct MyStruct {
    raw: *mut ffi::MyStruct,
    inner: Box<MyStructInner>, // save the real data in a Box to keep in the heap
}
// C側でポインタを作っているため、Rustの構造体がドロップされる際に解放されるように
impl Drop for MyStruct {
    fn drop(&mut self) {
        unsafe {
            ffi::mystruct_free(self.raw);
        }
    }
}

// Cに渡すコールバック関数
extern "C" fn callback_wrapper(
    _: *mut ffi::MyStruct,
    number: i32,
    callback_data: *mut libc::c_void,
) -> i32 {
    unsafe {
        // MyStructInner内のcallbackを実行
        let inner = &mut *(callback_data as *mut MyStructInner);
        let result = (inner.callback)(inner, number);
        result
    }
}

impl MyStruct {
    pub fn new() -> MyStruct {
        unsafe {
            let mut mystruct = MyStruct {
                raw: ffi::mystruct_new(),
                inner: Box::new(MyStructInner {
                    callback: Box::new(|_, _| 0),
                }),
            };

            ffi::mystruct_set_callback(
                mystruct.raw,
                callback_wrapper,
                &mut *mystruct.inner as *mut _ as *mut libc::c_void,
            );

            mystruct
        }
    }

    pub fn set_number(&mut self, number: i32) {
        unsafe {
            ffi::mystruct_set_number(self.raw, number);
        }
    }

    pub fn hello(&self) {
        unsafe {
            ffi::mystruct_hello(self.raw);
        }
    }

    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(&MyStructInner, i32) -> i32 + 'static,
    {
        self.inner.callback = Box::new(callback);
    }
}

// a code in an application
fn main() {
    let mut mystruct = MyStruct::new();
    // ffiの生ポインタとcallback関数を中に入れた構造体を持つ

    mystruct.set_number(11);
    mystruct.set_callback(|_, i| {
        println!("Hello, World: {}", i);
        12345
    });
    mystruct.hello();
}
