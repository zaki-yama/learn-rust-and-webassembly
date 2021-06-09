(module
 (func $add (param f32) (param f32) (result f32)
   get_local 0
   get_local 1
   f32.add)
 (export "add" (func 0))
)
