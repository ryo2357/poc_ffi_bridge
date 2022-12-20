use std::time::Duration;
use std::thread::sleep;
use tokio::sync::mpsc;
use std::sync::{Mutex, Arc};
struct SenderObject {
    tx:mpsc::Sender<i32>,
}
impl SenderObject {
    fn new(tx:mpsc::Sender<i32>,)-> Self{
        Self{
            tx:tx
        }

    }
    async fn send(&mut self,v:i32) {
        self.tx.send(v).await.unwrap();
    }
}


#[no_mangle]
extern "C" fn callback(target: *mut SenderObject, a: i32) {
    println!("I'm called from C with value {0}", a);
    unsafe {
        // Update the value in RustObject with the value received from the callback:
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async move{
            (*target).send(a).await;
        });
    }
}

#[link(name = "bridge")]
extern {
    fn register_callback(target: *mut SenderObject,
                        cb: extern fn(*mut SenderObject, i32)) -> extern "C" fn(i32);
}
#[link(name = "c_main")]
extern {
    fn init(cb:extern fn(i32), first_value:i32);
    fn trigger_main();
}

async fn ffi_loop(tx:mpsc::Sender<i32>) -> ! {
    // Create the object that will be referenced in the callback:
    let mut rust_object =  Box::new(SenderObject::new(tx));

    unsafe {
        let c_cb = register_callback(&mut *rust_object, callback);
        init(c_cb,9);

        loop{
            trigger_main();
            // println!("rust_object:{:?}",rust_object);
            println!("test");
            sleep(Duration::from_secs(2));
        }
    }
}

async fn ffi_receive_loop(state:Arc<Mutex<Vec<String>>>, rx:&mut mpsc::Receiver<i32>) {
    while let Some(val) = rx.recv().await {
        let message = "get:".to_string() + &val.to_string();
        state.lock().unwrap().push(message);
    }
}

#[tokio::main]
async fn main () {
    let state:Arc<Mutex<Vec<String>>> = Default::default();
    let (tx, mut rx) = mpsc::channel::<i32>(32);

    // tokio::spawnはジョインハンドルを作製した時点では実行されない
    tokio::spawn(async move {
        println!("aaaaa");
        ffi_loop(tx).await;
    });

    // 受信スレッドの記述
    let state_rx = state.clone();
    tokio::spawn (async move {
        println!("bbbb");
        ffi_receive_loop(state_rx, &mut rx).await;
    });

    loop{
        println!("state:{:?}",state);
        sleep(Duration::from_secs(5));
    }

}