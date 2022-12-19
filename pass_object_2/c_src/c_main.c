#include <stdio.h>

typedef void (*rust_callback)(int);
rust_callback main_cb;
int main_val;

void main_init(rust_callback callback,int first_val) {
  main_cb = callback;
  main_val = first_val;
}

void trigger_main() {
  main_cb(main_val);
}