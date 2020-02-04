https://wasdk.github.io/WasmFiddle/
===================================
C:

main.c

===================================
JS:

var wasmModule = new WebAssembly.Module(wasmCode);
var wasmInstance = new WebAssembly.Instance(wasmModule, wasmImports);
log(wasmInstance.exports.main());

===================================

Generated Text:

(module
 (type $FUNCSIG$i (func (result i32)))
 (type $FUNCSIG$vi (func (param i32)))
 (type $FUNCSIG$iii (func (param i32 i32) (result i32)))
 (import "env" "numLog" (func $numLog (param i32)))
 (import "env" "strLog" (func $strLog (param i32 i32) (result i32)))
 (table 0 anyfunc)
 (memory $0 1)
 (data (i32.const 16) "Hello from C!\00")
 (export "memory" (memory $0))
 (export "main" (func $main))
 (export "greet" (func $greet))
 (export "getDoubleNumber" (func $getDoubleNumber))
 (func $main (; 2 ;) (result i32)
  (i32.const 42)
 )
 (func $greet (; 3 ;)
  (drop
   (call $strLog
    (i32.const 16)
    (i32.const 13)
   )
  )
 )
 (func $getDoubleNumber (; 4 ;) (param $0 i32)
  (call $numLog
   (i32.shl
    (get_local $0)
    (i32.const 1)
   )
  )
 )
)

\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
//////////////////////////////////

https://webassembly.studio/
===================================

Modified .WAT:

(module
 (type $FUNCSIG$i (func (result i32)))
 (type $FUNCSIG$vi (func (param i32)))
 (type $FUNCSIG$iii (func (param i32 i32) (result i32)))
 (import "env" "numLog" (func $numLog (param i32)))
 (import "env" "strLog" (func $strLog (param i32 i32) (result i32)))
 ;;(memory $0 1);; MOVED TO
 (import "env"  "memory" (memory $0 2));; MODIFIED TO
 (table 0 anyfunc)
 ;;(memory $0 1);; MOVED FROM
 (data (i32.const 16) "Hello from C!\00")
 (export "memory" (memory $0))
 (export "main" (func $main))
 (export "greet" (func $greet))
 (export "getDoubleNumber" (func $getDoubleNumber))
 (func $main (; 2 ;) (result i32)
  (i32.const 42)
 )
 (func $greet (; 3 ;)
  (drop
   (call $strLog
    (i32.const 16)
    (i32.const 13)
   )
  )
 )
 (func $getDoubleNumber (; 4 ;) (param $0 i32)
  (call $numLog
   (i32.shl
    (get_local $0)
    (i32.const 1)
   )
  )
 )
)

===================================