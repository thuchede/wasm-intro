(module
  (export "add" (func $add))
  (import "js" "log" (func $log (param i32)))
  (func $add (param $a i32) (param $b i32) (result i32)
    (local $c i32) 
    get_local $a
    get_local $b
    i32.add
    set_local $c
    get_local $c
    call $log
    get_local $c
  )
)