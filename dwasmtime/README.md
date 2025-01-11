前置准备
rustup target add wasm32-wasi


编译wasm
cargo build --target wasm32-wasi


指定执行入口点位置
wasmtime --dir . target/wasm32-wasi/debug/mdp.wasm -- ./dwasmtime/mdp/ex.md