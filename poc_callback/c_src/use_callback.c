#include <stdio.h>

typedef void (*rust_callback)(int);
rust_callback cb;

int register_callback(rust_callback callback) {
    cb = callback;
    return 1;
}

void trigger_callback() {
  if (cb != NULL) {
    cb(7);
  } else {
    printf("callback function is not set!");
  }
}