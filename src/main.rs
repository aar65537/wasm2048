use wasmer::{imports, Instance, Module, Store, MemoryView};

fn main() {
    let path = "target/wasm32-unknown-unknown/debug/naive.wasm";
    let wasm_bytes = std::fs::read(&path).unwrap();
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes).unwrap();
    let instance = Instance::new(&module, &imports! {}).unwrap();
    let memory = instance.exports.get_memory("memory").unwrap();
    let memory_view: MemoryView<u8> = memory.view();
    println!("Memory View Size = {}", memory_view.len() / 65536);
    let evaluate = instance.exports.get_function("evaluate").unwrap();
    let result = evaluate.call(&[]).unwrap();
    println!("Result = {:?}", result[0]);
}

