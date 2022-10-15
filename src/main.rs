use std::path::Path;
use wasmer::{Module, Store};
use wasm2048::Action;
use wasm2048::engine::WasmAgent;

fn main() {
    let path_str = "./target/wasm32-unknown-unknown/debug/naive.wasm";
    let path = Path::new(path_str);
    let store = Store::default();
    let module = Module::from_file(&store, path).unwrap();
    let agent = WasmAgent::new(&module);
    let result = agent.wasm_evaluate.call(Action::Left).unwrap();
    println!("Result = {:?}", result);
    agent.wasm_update.call(0, Action::Left).unwrap();
}

