import ctypes
from typing import Tuple
import wasmtime


def _decode_utf8(mem: wasmtime.Memory, store: wasmtime.Storelike, ptr: int, len: int) -> str:
                        ptr = ptr & 0xffffffff
                        len = len & 0xffffffff
                        if ptr + len > mem.data_len(store):
                            raise IndexError('string out of bounds')
                        base = mem.data_ptr(store)
                        base = ctypes.POINTER(ctypes.c_ubyte)(
                            ctypes.c_ubyte.from_address(ctypes.addressof(base.contents) + ptr)
                        )
                        return ctypes.string_at(base, len).decode('utf-8')
                

def _encode_utf8(val: str, realloc: wasmtime.Func, mem: wasmtime.Memory, store: wasmtime.Storelike) -> Tuple[int, int]:
                        bytes = val.encode('utf8')
                        ptr = realloc(store, 0, 0, 1, len(bytes))
                        assert(isinstance(ptr, int))
                        ptr = ptr & 0xffffffff
                        if ptr + len(bytes) > mem.data_len(store):
                            raise IndexError('string out of bounds')
                        base = mem.data_ptr(store)
                        base = ctypes.POINTER(ctypes.c_ubyte)(
                            ctypes.c_ubyte.from_address(ctypes.addressof(base.contents) + ptr)
                        )
                        ctypes.memmove(base, bytes, len(bytes))
                        return (ptr, len(bytes))
                
