;; wat2wasm will fail
(module
  (func $inner
    (result i32)
    (local $l i32)
    ;; 99 is on the stack in the calling function
    local.set $l

    i32.const 2
  )
  (func (export "main")
    (result i32)

    i32.const 99 ;; push 99 onto stack - [99]
    call $inner  ;; 99 is on the stack here
  )
)
