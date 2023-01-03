use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let mut store = Store::<()>::default();
    let module = Module::from_file(store.engine(), "dwasmtime/embed/examples/gcd.wat")?;
    let instance = Instance::new(&mut store, &module, &[])?;

    let gcd = instance.get_typed_func::<(i32, i32), i32>(&mut store, "gcd")?;

    let result = gcd.call(&mut store, (6, 27))?;

    println!("GCD of (6,27) -> {}", result);

    Ok(())
}
