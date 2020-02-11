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

====================================
13: Compilation Options (Part 3: Compiler-Optimization Flags)

-We can modify each of these.
-Let's re-compile the standard JS-extension again:
> emcc lib/demo.c -s WASM=1 -o public/demo.js

>ls -lh public/
total 135K
-rw-r--r-- 1 josh 197121 106K Feb 10 18:25 demo.js
-rw-r--r-- 1 josh 197121  22K Feb 10 18:25 demo.wasm
-rw-r--r-- 1 josh 197121  208 Feb 10 18:22 index.html
-rw-r--r-- 1 josh 197121  256 Feb 10 18:22 main.wasm
-rw-r--r-- 1 josh 197121  220 Feb 10 18:22 program.wasm

-The large size of demo.js and demo.wasm will have a very large impact on page load times.
-Let's apply compiler optimizations
    --O1 Optimization Level 1
    --O2 Optimization Level 2
    --O3 Optimization Level 3

> emcc lib/demo.c -s WASM=1 -O2 -o public/demo.js

>ls -lh public/
total 39K
-rw-r--r-- 1 josh 197121  23K Feb 10 18:29 demo.js
-rw-r--r-- 1 josh 197121 9.7K Feb 10 18:29 demo.wasm
-rw-r--r-- 1 josh 197121  208 Feb 10 18:22 index.html
-rw-r--r-- 1 josh 197121  256 Feb 10 18:22 main.wasm
-rw-r--r-- 1 josh 197121  220 Feb 10 18:22 program.wasm

-Note the massive reduction in size of demo.js and demo.wasm

