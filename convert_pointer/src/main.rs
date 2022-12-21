#![allow(unused)]
pub type LONG = std::os::raw::c_int;

#[link(name = "pointer")]
extern {
    fn sum(arr:*const LONG, size:usize) -> LONG;
    fn set_pointer(arr:*const LONG, size:usize);
    fn sum_from_pointer() -> LONG;
    fn make_pointer_array() -> *const LONG;
    fn get_sequential_array(size:usize) -> *const LONG;
    fn free_array(arr:*const LONG);
}

fn rust_to_c(){
    let mut slice = &[1, 13, 5];
    let c_return = unsafe { sum(slice.as_ptr(), slice.len())};
    // スライスは編集できない
    // slice[2] = 15;

    println!("slice:{:?}", slice);
    println!("c_return:{:?}",c_return );

    
    let mut vec = vec![1, 13, 5];
    let c_return = unsafe { sum(vec.as_ptr(), vec.len()) };
    // ベクタも要素の直接編集はできない
    // vec[1] = 15;

    println!("vec:{:?}", slice);
    println!("c_return:{:?}",c_return );

    let mut arr = [1,13,8];
    let c_return = unsafe { sum((&arr).as_ptr(), slice.len())};
    arr[1] = 15;
    
    println!("vec:{:?}", arr);
    println!("c_return:{:?}",c_return );
}

fn arr_pointer(){
    // ポインタを渡して参照元を編集することもできる
    // 結構危険そう
    let mut arr = [1,13,8];
    unsafe{ set_pointer((&arr).as_ptr(), arr.len());}

    let c_return = unsafe { sum_from_pointer()};
    println!("c_return:{:?}",c_return );

    arr[1] = 15;
    let c_return = unsafe { sum_from_pointer()};
    println!("c_return:{:?}",c_return );
    
}

fn c_to_rust(){
    let size = 5;
    let ptr = unsafe { get_sequential_array(size) };
    let slice = unsafe { std::slice::from_raw_parts(ptr, size) };
    println!("slice:{:?}",slice);
    // Vec::from_raw_partsはバッファの所有権を持っているため、破棄するときにメモリを開放してしまう
    // スライスを複製してベクタにしている
    let vec = slice.to_vec();
    println!("vec:{:?}",vec);
    println!("slice:{:?}",slice);

    drop(slice);  // 安全のため．
    unsafe {
        free_array(ptr);
    }
}

fn main(){
    c_to_rust();
}


