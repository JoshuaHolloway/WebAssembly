// -Wherever you add mod [module_name];
//  in the code, think of that module
//  as being injected at exactly
//  that spot in the hierarchy.
mod board;
// -Each Rust library has a single root module,
//  the name of which is specified in the .toml file
//  (the name setting under the [package section]).
// -By convention, the code for the root module
//  is found in the lib.rs file, so by including
//  mod board; at the top of lib.rs file,
//  we are declaring a submodule named board
//  that exists directly under the root.
// -Elements in the board submodule can be
//  referenced from anywhere using the
//  rustycheckers::board prefix.

// Local toolchain
// > rustup toolchain list
// < stable-x86_64-pc-windows-msvc (default)

// See the targets Rust supports
// > rustup target list

// Add the WebAssembly target
// > rustup target add wasm32-unknown-unknown

// Initialize project
// > cargo new --lib rustycheckers

// Edit cargo.toml to indicate that this project will expose a C-Style dynamic library
// (The lib is then used by other linkers to produce a WebAssembly module)
// [lib]
// crate-type = ["cdylib"]

// Use 2015 syntax (remove from the .toml file)
// edition = "2018"

mod game;
