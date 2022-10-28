use crate::{Action, Register};
use std::mem::size_of;
use wasmer::{Instance, WasmCell, MemoryView};

// #[derive(Debug)]
pub struct Engine {
    // register_binding: MemoryView<'a, Register>,
    // register: WasmCell<'a, Register>,
    score: i32,
}

impl Engine {
    pub fn new(instance: &Instance) -> Self {
        let memory = instance.exports.get_memory("memory").unwrap();
        let register_global = instance.exports.get_global("REGISTER").unwrap();
        let register_addr = register_global.get().i32().unwrap() as usize;
        println!("Register address is {:x}", register_addr);
        println!("Size of register is {}", size_of::<Register>());
        let register_binding = memory.view::<Register>();
        let register_wrapper = register_binding
            .get(register_addr / size_of::<Register>())
            .unwrap();
        let register = WasmCell::new(register_wrapper);
        let mut engine = Engine { score: 0 };
        engine.new_game();
        // println!("Engine register is {:?}", engine.register);
        engine
    }

    pub fn new_game(&mut self) {
        // let mut register = self.register.get();
        // register.clear();
        // register.add_tile();
        // register.add_tile();
        // self.register.set(register);
    }

    pub fn game_over(&self) -> bool {
        false
    }

    pub fn can_execute(&self, action: Action) -> bool {
        false
    }

    pub fn execute(&mut self, action: Action) -> i32 {
        0
    }
}
