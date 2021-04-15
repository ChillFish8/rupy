(module
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (memory $0 0)
 (table $0 1 funcref)
 (export "fib" (func $module/fib))
 (export "memory" (memory $0))
 (func $module/fib (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  i32.const 1223
  local.set $1
  local.get $1
  i32.const 3
  i32.mul
  local.set $2
  local.get $2
  local.get $0
  i32.mul
  local.set $3
  loop $while-continue2|1
   local.get $1
   i32.const 1000
   i32.gt_s
   local.set $4
   local.get $4
   if
    local.get $1
    i32.const 1
    i32.sub
    local.set $1
    br $while-continue2|1
   end
  end
  local.get $3
 )
)