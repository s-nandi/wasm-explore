use anyhow::Result;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

struct WasiHostCtx {
    ctx: WasiCtx,
    table: wasmtime::component::ResourceTable,
}

impl WasiHostCtx {
    fn new() -> Self {
        Self {
            ctx: WasiCtxBuilder::new()
                .env("env-key", "env-val")
                .inherit_stdio()
                .build(),
            table: ResourceTable::new(),
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

fn main() -> Result<()> {
    println!("Starting");
    let mut config = Config::default();
    config.async_support(false);
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let wasi_view = WasiHostCtx::new();
    let mut store = Store::new(&engine, wasi_view);

    println!();
    // Moving this outside the loop doesn't work before it leads to multiple-definition errors
    let mut linker = Linker::new(&engine);
    linker.allow_shadowing(false);

    // let file = "hello_world_export.wasm";
    let file = "hello_world_export_wasi_and_myimport_imports.wasm";

    let component = load_file(&engine, file);

    let res: Box<dyn std::fmt::Debug>;
    let mut root = linker.instance("component:dep/dep")?;
    root.func_wrap("myimport", |_wasi_view, _: ()| {
        Ok(("imported string into interface",))
    })?;

    let instance = linker
        .instantiate(&mut store, &component)
        .expect("Failed to instantiate component");

    let func = instance
        .get_typed_func::<(), (String,)>(&mut store, "hello-world")
        .expect("Could not load valid function instance");
    let val = func.call(&mut store, ())?;
    res = Box::new(val);

    println!("Result: {res:?}");

    Ok(())
}
