#include <stdint.h>

extern "C" {

int* copy_array(int* in_array, int length) {
  int out_array[length];
  for (int i=0; i<length; i++) {
    out_array[i] = in_array[i];
  }
  return out_array;
}

}