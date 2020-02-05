https://wasdk.github.io/WasmFiddle/
===================================
C:

void numLog(int n);
void stringLog(char * msg);

int main() { 
  return 42;
}

void greet() {
  strLog("Hello from C!");
}

void getDoubleNumber(int x) {
  numLog( x * 2 );
}

===================================
JS:

var wasmModule = new WebAssembly.Module(wasmCode);
var wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
log(wasmInstance.exports.main());
