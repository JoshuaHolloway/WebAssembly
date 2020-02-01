https://wasdk.github.io/WasmFiddle/
===================================
C:

int main() { 
  return 42;
}

int getDoubleNumber(int x) {
  return x * 2;
}

===================================
JS:

var wasmModule = new WebAssembly.Module(wasmCode);
var wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
log(wasmInstance.exports.main());
