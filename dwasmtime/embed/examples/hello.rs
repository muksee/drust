use anyhow::Result;
use wasmtime::*;

struct MyState {
    name: String,
    count: usize,
}

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "dwasmtime/embed/examples/hello.wat")?;

    let mut store = Store::new(
        &engine,
        MyState {
            name: "hello, world".to_string(),
            count: 0,
        },
    );

    let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
        println!("calling back");
        println!("> {}", caller.data().name);
        caller.data_mut().count += 1;
    });

    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

    run.call(&mut store, ())?;

    Ok(())
}
