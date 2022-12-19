#include <stdio.h>

void* target;


void set_struct(void* rust_target) {
  target = rust_target;
}

void* get_struct() {
    return target;
}

