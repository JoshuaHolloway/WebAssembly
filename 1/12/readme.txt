11: Compilation Options (Part 1: Compile with accompanying wasm/loading script and HTML playground page)

-Now that we know how to compile basic wasm with accompanying JavaScript loading code,
 let's explore the Emscrypten compiler a bit further and see how to customize
 and optimize WASM compilation.
-To generate a WASM and matching Emscrypten JavaScript code,
 we specify the output file as being of type .js:
emcc lib/demo.c -s WASM=1 -o demo.js

-Emscrypten allows us to compile with some boilerplate HTML also
emcc lib/demo.c -s WASM=1 -o public/demo.html


> emcc lib/demo.c -s WASM=1 -o public/demo.html
> npm start
Navigate to localhost:2222/demo.html
:)
====================================
12: Compilation Options (Part 2: Generate a simple WASM with no extras)

-Change output from demo.html to demo.wasm and specify SIDE_MODULE with -s SIDE_MODULE=1
-Side module is used to load other WebAssembly modules asside from the main module with the
 accompanying Emscripten JavaScript

> emcc lib/demo.c -s WASM=1 -s SIDE_MODULE=1 -o public/demo.wasm

-3 main type of compiler outputs:
    --1. emcc lib/demo.c -s WASM=1 -o public/demo.html
    --2. emcc lib/demo.c -s WASM=1 -o public/demo.wasm
    --3. emcc lib/demo.c -s WASM=1 -s SIDE_MODULE=1 -o public/demo.wasm