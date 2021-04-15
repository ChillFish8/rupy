(module
 (memory $0 1)
 (data (i32.const 12) "\1c\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\08\00\00\00y\00e\00e\00t\00\00\00\00\00")
 (table $0 1 funcref)
 (export "fib" (func $module/fib))
 (export "memory" (memory $0))
 (func $module/bar
 (local $0 i32)
  i32.const 32
  local.set $0
 )
 (func $module/fib
  call $module/bar
 )
 (type $none_=>_none (func))
)