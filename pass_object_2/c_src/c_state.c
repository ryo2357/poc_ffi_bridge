#include <stdio.h>

typedef void (*rust_callback)(int);
rust_callback state_cb;
void* target;


void state_init(void* rust_target,rust_callback callback) {
  state_cb = callback;
  target = rust_target;
}

void state_trigger(int val) {
    state_cb(target, val);
}

