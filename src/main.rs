use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Starting App");

    runtime::start("target/wasm32-wasi/debug/example_plugin.wasm")?;

    Ok(())
}
