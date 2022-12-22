#include <stdio.h>

typedef void (*c_callback)(int);
c_callback main_cb;
int main_val;

void init(c_callback callback,int first_val) {
    main_cb = callback;
    main_val = first_val;
}

void trigger_main() {
  main_cb(main_val);
  main_val += 2;
}