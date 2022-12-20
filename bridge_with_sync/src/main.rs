use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::sync::{Mutex, Arc, mpsc};

struct SenderObject {
    tx:mpsc::Sender<i32>,
}
impl SenderObject {
    fn new(tx:mpsc::Sender<i32>,)-> Self{
        Self{
            tx:tx
        }
    }
    fn send(&mut self,v:i32) {
        self.tx.send(v).unwrap();
    }
}


#[no_mangle]
extern "C" fn callback(target: *mut SenderObject, a: i32) {
    println!("I'm called from C with value {0}", a);
    unsafe {
        (*target).send(a);
    }
}

#[link(name = "bridge")]
#[allow(improper_ctypes)]
extern {
    fn register_callback(target: *mut SenderObject,
                        cb: extern fn(*mut SenderObject, i32)) -> extern "C" fn(i32);
}
#[link(name = "c_main")]
extern {
    fn init(cb:extern fn(i32), first_value:i32);
    fn trigger_main();
}

fn ffi_loop(tx:mpsc::Sender<i32>) -> ! {
    // Create the object that will be referenced in the callback:
    let mut rust_object =  Box::new(SenderObject::new(tx));

    unsafe {
        let c_cb = register_callback(&mut *rust_object, callback);
        init(c_cb,9);

        loop{
            trigger_main();
            // println!("rust_object:{:?}",rust_object);
            sleep(Duration::from_secs(2));
        }
    }
}

fn ffi_receive_loop(state:Arc<Mutex<Vec<String>>>, rx:&mut mpsc::Receiver<i32>) {
    loop{
        let val = rx.recv().unwrap();
        let message = "get:".to_string() + &val.to_string();
        state.lock().unwrap().push(message);
    }
}

fn main () {
    let state:Arc<Mutex<Vec<String>>> = Default::default();
    let (tx, mut rx) = mpsc::channel();

   
    thread::spawn(move|| {
        ffi_loop(tx);
    });

    // 受信スレッドの記述
    let state_rx = state.clone();
    thread::spawn ( move|| {
        ffi_receive_loop(state_rx, &mut rx);
    });

    loop{
        println!("state:{:?}",state);
        sleep(Duration::from_secs(5));
    }

}