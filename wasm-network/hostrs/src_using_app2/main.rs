use std::collections::HashMap;

use anyhow::{Context, Result};
use wasmtime::component::{Component, Linker, Resource};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{command, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "app2/wit",
});

use component::dependency::imports;

struct MyBlob {
    name: String,
}

struct WasiHostCtx {
    ctx: WasiCtx,
    table: wasmtime::component::ResourceTable,
    state: HashMap<u32, MyBlob>,
}

impl WasiHostCtx {
    fn new() -> Self {
        Self {
            ctx: WasiCtxBuilder::new()
                .env("env-key", "env-val")
                .inherit_stdio()
                .build(),
            table: ResourceTable::new(),
            state: HashMap::new(),
        }
    }
}

impl WasiView for WasiHostCtx {
    fn table(&self) -> &wasmtime::component::ResourceTable {
        &self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn load_file(engine: &Engine, file: &str) -> Component {
    println!("Loading {file}");
    let component = Component::from_file(&engine, file).expect("Failed to load file");
    println!("Done loading {file}");
    check_resources_required(&component);
    component
}

fn check_resources_required(component: &Component) {
    let required = component
        .resources_required()
        .expect("No imports required?");
    let num_requirements = required.num_tables;
    println!("# requirements = {num_requirements:?}");
}

impl imports::Host for WasiHostCtx {
    fn myimport(&mut self) -> wasmtime::Result<Resource<imports::Blob>> {
        let table = &mut self.state;
        let id = 0;
        if !table.contains_key(&id) {
            let new_blob = MyBlob {
                name: "foo".to_owned(),
            };
            table.insert(id, new_blob);
        };
        return Ok(Resource::new_own(id));
    }
}

impl imports::HostBlob for WasiHostCtx {
    fn name(&mut self, self_: Resource<imports::Blob>) -> wasmtime::Result<String> {
        let id = self_.rep();
        let state = &self.state[&id];
        let name = state.name.clone();
        Ok(name)
    }

    fn drop(&mut self, rep: Resource<imports::Blob>) -> wasmtime::Result<()> {
        let id = rep.rep();
        self.state.remove(&id);
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("Starting");
    let mut config = Config::default();
    config.async_support(false);
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let wasi_view = WasiHostCtx::new();
    let mut store = Store::new(&engine, wasi_view);

    let mut linker = Linker::new(&engine);
    linker.allow_shadowing(false);

    Example::add_to_linker(&mut linker, |state: &mut WasiHostCtx| state)?;

    let file = "hello_world_export_wasi_and_myimport_imports.wasm";
    let component = load_file(&engine, file);

    command::sync::add_to_linker(&mut linker).context("Failed to link command world")?;

    let (bindings, _) = Example::instantiate(&mut store, &component, &linker)?;
    let res = bindings.call_hello_world(&mut store);
    println!("Result: {res:?}");

    Ok(())
}
