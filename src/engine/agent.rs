use crate::engine::Engine;
use crate::Action;
use wasmer::{imports, ExportError, Instance, Memory, Module, NativeFunc};

pub struct WasmAgent {
    // memory: &'a Memory
    instance: Instance,
    pub engine: Engine,
    pub wasm_evaluate: NativeFunc<Action, f32>,
    pub wasm_update: NativeFunc<(i32, Action), ()>,
}

impl<'a> WasmAgent {
    pub fn new(module: &Module) -> Self {
        let instance = Instance::new(&module, &imports! {}).unwrap();
        let engine = Engine::new(&instance);
        let wasm_evaluate: NativeFunc<Action, f32> =
            instance.exports.get_native_function("evaluate").unwrap();
        let wasm_update: NativeFunc<(i32, Action), ()> =
            instance.exports.get_native_function("update").unwrap();
        WasmAgent {
            instance,
            engine,
            wasm_evaluate,
            wasm_update,
        }
    }

    pub fn memory(&self) -> Result<&Memory, ExportError> {
        self.instance.exports.get_memory("memory")
    }
}
