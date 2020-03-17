// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

// Initialize project (expose a C-style dynamic library)
// > cargo new --lib rustwasmhello

// Build (debug mode)
// > cargo build --target wasm32-unknown-unknown

// Builde (release mode)
// > cargo build --release --target=wasm32-unknown-unknown

// Verify the output contains an exported function called add_one via wasm-objdump tool
// > wasm-objdump -x target/wasm32-unknown-unknown/release/rustwasmhello.wasm

// From objdump (16 => 16Pages x (64kB/page) block of linear memory):
// Memory[1]:
// - memory[0] pages: initial=16
