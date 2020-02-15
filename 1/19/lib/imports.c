#include <emscripten.h>
#include <stdio.h>

int main() {

    printf("WASM Ready\n");

    // -Call JS function (eval)
    // -Give the browser a string of JS-code and execute it
    emscripten_run_script("console.log('Hello from C!')");

    return 1;
}

// > emcc lib/imports.c -s WASM=1 -o public/imports.js