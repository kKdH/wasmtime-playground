use anyhow::Context;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

pub fn start(module_file: &str) -> anyhow::Result<()> {
    println!("Start Runtime");

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(&engine, module_file)?;
    linker.module(&mut store, "", &module)?;
    let start_fn = linker
        .get(&mut store, "", "start")
        .unwrap()
        .into_func()
        .unwrap()
        .typed::<(i32, i32), i32>(&store)?;

    let result = start_fn.call(&mut store, (17, 25))?;
    println!("{result}");

    Ok(())
}
