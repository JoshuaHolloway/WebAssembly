#include <stdio.h>
#include <string.h>

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

// [14]
int getNum() {
    return 22;
}

// > emcc lib/demo.c -s WASM=1 -s EXPORTED_FUNCTIONS="['_getNum', '_main']" -o public/demo.js

// [15]
int getDoubleNum(int n) {
    return n * 2;
}

// [17]
char * greet() {
    return "Hello";
}

// [18]
char * greet2( char * name ) {
    char * greeting = "Hello ";
    
    // strcat mutates first arg
    strcat(greeting, name);

    return greeting;
}