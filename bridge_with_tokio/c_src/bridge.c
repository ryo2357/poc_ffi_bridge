#include <stdio.h>

typedef void (*rust_callback)(void*, int);
typedef void (*c_callback)(int);
void* cb_target;
rust_callback bridge_cb;


void trigger_callback(int val) {
  bridge_cb(cb_target, val); // Will call callback(&rustObject, 7) in Rust.
}

void* register_callback(void* callback_target, rust_callback callback) {
    cb_target = callback_target;
    bridge_cb = callback;
    return trigger_callback;
}


