from wasmtime import Store, Module, Instance, Func, FuncType, ValType

store = Store()
module = Module(store.engine, """
  (module
    (import "" "log_import" (func $log (param i32)))
    (func $myadder (param i32 i32)
        local.get 0
        local.get 1
        i32.add
        call $log
    )
    (export "run" (func $myadder))
  )
""")

def bar(v: int):
    print(f"Calculated sum: {v}!")

foo = Func(store, FuncType([ValType.i32()], []), bar)

instance = Instance(store, module, [foo])
run = instance.exports(store)["run"]
res = run(store, 1, 33)
print("Result: ", res)