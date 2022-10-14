use std::fs;
use wasmer::{imports, Instance, Module, Store, Value};

fn main() {
    let path = "target/wasm32-unknown-unknown/release/naive.wasm";
    let wasm_bytes = fs::read(&path).unwrap();
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();

    let add_one = instance.exports.get_function("add_one").unwrap();
    let result = add_one.call(&[Value::I32(42)]).unwrap();
    println!("Result = {:?}", result[0]);
}

