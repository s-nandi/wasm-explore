from .imports import RootImports
from .intrinsics import _decode_utf8, _encode_utf8
import pathlib
import wasmtime

class Root:
    
    def __init__(self, store: wasmtime.Store, import_object: RootImports) -> None:
        path = pathlib.Path(__file__).parent / ('root.core1.wasm')
        module = wasmtime.Module.from_file(store.engine, path)
        instance0 = wasmtime.Instance(store, module, []).exports(store)
        path = pathlib.Path(__file__).parent / ('root.core0.wasm')
        module = wasmtime.Module.from_file(store.engine, path)
        instance1 = wasmtime.Instance(store, module, [
            instance0["0"],
        ]).exports(store)
        def lowering0_callee(caller: wasmtime.Caller, arg0: int, arg1: int, arg2: int, arg3: int) -> None:
            ptr = arg0
            len0 = arg1
            list = _decode_utf8(self._core_memory0, caller, ptr, len0)
            ptr1 = arg2
            len2 = arg3
            list3 = _decode_utf8(self._core_memory0, caller, ptr1, len2)
            import_object.network_provider.get(list, list3)
        lowering0_ty = wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), ], [])
        trampoline0 = wasmtime.Func(store, lowering0_ty, lowering0_callee, access_caller = True)
        core_memory0 = instance1["memory"]
        assert(isinstance(core_memory0, wasmtime.Memory))
        self._core_memory0 = core_memory0
        path = pathlib.Path(__file__).parent / ('root.core2.wasm')
        module = wasmtime.Module.from_file(store.engine, path)
        instance2 = wasmtime.Instance(store, module, [
            trampoline0,
            instance0["$imports"],
        ]).exports(store)
        realloc0 = instance1["cabi_realloc"]
        assert(isinstance(realloc0, wasmtime.Func))
        self._realloc0 = realloc0
        lift_callee0 = instance1["run"]
        assert(isinstance(lift_callee0, wasmtime.Func))
        self.lift_callee0 = lift_callee0
    def run(self, caller: wasmtime.Store, uri: str, filename: str) -> None:
        ptr, len0 = _encode_utf8(uri, self._realloc0, self._core_memory0, caller)
        ptr1, len2 = _encode_utf8(filename, self._realloc0, self._core_memory0, caller)
        self.lift_callee0(caller, ptr, len0, ptr1, len2)
