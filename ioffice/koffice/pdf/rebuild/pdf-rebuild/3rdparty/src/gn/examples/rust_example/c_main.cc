#include <stdio.h>

int c_lib_get_value();

int main() {
  int val = c_lib_get_value();
  printf("Value from c_lib (via Rust): %d\n", val);
  if (val == 42) {
    printf("SUCCESS!\n");
    return 0;
  }
  return 1;
}
