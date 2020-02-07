
[10]:
-Created lib/demo.c
-Running below will create demo.js and demo.wasm

> emcc lib/demo.c -s WASM=1 -o public/demo.js

-See 1-10.png

-demo.js is the glue code to run demo.wasm.

-Link the Emscrypten JavaScript by adding a src tag
 in the index.html file (public/index.html)

-Create index.html in /public:

 <html>
    <head></head>
    <body>
        <h1>Using Emscrypten</h1>
        <script src="demo.js"></script>
    </body>
 </html>

-Run the server: 
npm start