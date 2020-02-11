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
