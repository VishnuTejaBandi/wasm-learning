use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

fn main() -> Result<()> {
    let mut config = Config::default();
    config.cache_config_load_default()?;
    let engine = Engine::new(&config)?;

    bindgen!({world: "vt-plugin", path: "../vt-plugin/wit/world.wit"});

    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker)?;

    let mut store = Store::new(
        &engine,
        MyState {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            http: WasiHttpCtx::new(),
        },
    );
    // Load the plugin file from disk
    let component = Component::from_file(&engine, "./asura.wasm")?;

    // Setup an Instance and a world Component that can be called
    // Instance needs to be not dropped when the world component is called.
    let vt = VtPlugin::instantiate(&mut store, &component, &linker)?;

    // Here our `greet` function doesn't take any parameters for the component,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    let greeting = vt.call_get_manga_list(&mut store)?.unwrap();
    println!("{greeting:?}");
    Ok(())
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
    http: WasiHttpCtx,
}

impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for MyState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
