(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.const 4
    i32.shl
    local.get 1
    i32.const 3
    i32.shr_u
    i32.add)
  (export "pow2_mul" (func 0)))
