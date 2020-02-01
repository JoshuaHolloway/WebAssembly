https://wasdk.github.io/WasmFiddle/
===================================
C:

void consoleLog(int n);

int main() { 
  return 42;
}

void getDoubleNumber(int x) {
  consoleLog( x * 2 );
}

===================================
JS:

var wasmModule = new WebAssembly.Module(wasmCode);
var wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
log(wasmInstance.exports.main());
