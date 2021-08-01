(module
 (type $i32_=>_none (func (param i32)))
 (type $i32_i32_i32_i32_=>_none (func (param i32 i32 i32 i32)))
 (type $none_=>_none (func))
 ;; the declare command at the top of the AssemblyScript created an import
 ;; that imports the console_log function inside of the outer as_hello
 ;; object. AssemblyScript requires its imports in the AssemblyScript file name
 ;; not including the .ts extension
 (import "as_hello" "console_log" (func $as_hello/console_log (param i32)))
 (import "env" "abort" (func $~lib/builtins/abort (param i32 i32 i32 i32)))
 (global $~lib/memory/__stack_pointer (mut i32) (i32.const 16444))
 ;; using a string automatically creates the memory expression
 (memory $0 1)
 (data (i32.const 12) ",")
 ;; the data line below wraps because the line is too long
 ;; The "hello world!" string is preceded by a header and has a hex 00 byte in
 ;; between every letter in the string. This is because AssemblyScript uses
 ;; the UTF-16 character set instead of ASCII as we did when we were manipulating
 ;; string data in WAT.
 (data (i32.const 24) "\01\00\00\00\18\00\00\00h\00e\00l\00l\00o\00 \00w\00o\00r\00l\00d\00!")
 ;; The module exports our function with the AssemblyScript name we gave it.
 (export "HelloWorld" (func $as_hello/HelloWorld))
 (export "memory" (memory $0))
 (func $as_hello/HelloWorld
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.sub
  global.set $~lib/memory/__stack_pointer
  global.get $~lib/memory/__stack_pointer
  i32.const 60
  i32.lt_s
  if
   i32.const 16464
   i32.const 16512
   i32.const 1
   i32.const 1
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/memory/__stack_pointer
  i32.const 0
  i32.store
  global.get $~lib/memory/__stack_pointer
  i32.const 32
  i32.store
  i32.const 32
  call $as_hello/console_log
  global.get $~lib/memory/__stack_pointer
  i32.const 4
  i32.add
  global.set $~lib/memory/__stack_pointer
 )
)
