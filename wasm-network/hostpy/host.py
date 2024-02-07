import functools
from pathlib import Path
from typing import Callable
import requests

from wasmtime import Config, Engine, Store
from hostbindings.imports import network_provider
import hostbindings

URI = "https://www.rust-lang.org/logos/rust-logo-512x512.png"
FILENAME = "temp.png"

def to_output_path(filename: str) -> Path:
    assert not Path(filename).is_absolute()
    full_path = Path("output").joinpath(filename)
    return full_path

class HostNetworkProvider(network_provider.HostNetworkProvider):
    def get(self, uri: str, filename: str) -> None:
        path = to_output_path(filename)
        r = requests.get(uri)
        with open(path, "wb") as f:
            f.write(r.content)
        print(f"host.py :: Saved {uri} response to {path}")

# functools.partial loses type information
# so this lets us add explicit typing
class HostBindings:
    root: hostbindings.Root
    store: Store

    def __init__(self, store: Store, imports: hostbindings.RootImports):
        root = hostbindings.Root(store, imports)
        self.root = root
        self.store = store
    
    def run(self, uri: str, filename: str) -> None:
        return self.root.run(caller=self.store, uri=uri, filename=filename)

def setup_module_bindings() -> HostBindings:
    config = Config()
    engine = Engine(config)
    store = Store(engine)
    host_imports = hostbindings.RootImports(
        network_provider=HostNetworkProvider(),
    )
    return HostBindings(store, host_imports)

def main():
    bindings = setup_module_bindings()
    bindings.run(URI, FILENAME)

if __name__ == "__main__":
    main()
