https://wasdk.github.io/WasmFiddle/
===================================
C:

#include <string.h>

void numLog(int n);
void stringLog(char * offset, int length);

int main() { 
  return 42;
}

void greet() {
  char * msg = "Hello from C!";
  strLog(msg, strlen(msg));
}

void getDoubleNumber(int x) {
  numLog( x * 2 );
}

===================================
JS:

var wasmModule = new WebAssembly.Module(wasmCode);
var wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
log(wasmInstance.exports.main());
