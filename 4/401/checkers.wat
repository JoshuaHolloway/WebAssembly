;; The memory declaration $mem must have at least One 64-kB Page of memory
(module
    (memory $mem 1)

    (func $indexForPosition (param $x i32) (param $y i32) (result i32)
        (i32.add
            (i32.mul
                (i32.const 8)
                (get_local $y)
            )
            (get_local $x)
        )
    )
    ;; Offset = ( x + y * 8 ) * 4   [8 is for number of rows, and 4 is for bytes per 32-bit integer]
    (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
        (i32.mul
            (call $indexForPosition (get_local $x) (get_local $y))
            (i32.const 4)
        )
    )
)

;; wat2wasm checkers.wat -o checkers.wasm