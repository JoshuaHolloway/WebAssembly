#include <stdio.h>

int main() {

    printf("WASM Ready\n");

    return 1;
}

// > emcc lib/demo.c -s OPTION=value
// > emcc lib/demo.c -s WASM=1 -o public/demo.js