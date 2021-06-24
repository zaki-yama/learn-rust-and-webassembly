(module
  ;; Imported JavaScript function (below) takes position and length
  (import "env" "str_pos_len" (func $str_pos_len (param i32 i32)))
  (import "env" "buffer" (memory 1))
  ;; 30 characters string
  (data (i32.const 256) "Know the length of this string")
  ;; 35 characters
  (data (i32.const 384) "Also know the length of this string")

  (func (export "main")
    ;; length of the first string is 30 characters
    (call $str_pos_len (i32.const 256) (i32.const 30))
    ;; length of the second string is 35 characters
    (call $str_pos_len (i32.const 384) (i32.const 35))
  )
)
