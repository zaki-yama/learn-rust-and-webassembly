(module
  ;; param i32: length of string
  (import "env" "print_string" (func $print_string(param i32) ))
  ;; memory 1: allocate 1 page(smallest chunk of memory) of linear memory
  (import "env" "buffer" (memory 1))
  (global $start_string (import "env" "start_string") i32)
  (global $string_len i32 (i32.const 12))
  (data (global.get $start_string) "hello world!")
  (func (export "helloworld")
    (call $print_string (global.get $string_len))
  )
)
