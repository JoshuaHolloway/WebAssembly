#include <stdio.h>

int main() {

    printf("WASM Ready - Testing with Emscripten HTML\n");

    return 1;
}

// > emcc lib/demo.c -s OPTION=value
// > emcc lib/demo.c -s WASM=1 -o public/demo.js

// [11]
// > emcc lib/demo.c -s WASM=1 -o public/demo.html
// > npm start
// Navigate to localhost:2222/demo.html