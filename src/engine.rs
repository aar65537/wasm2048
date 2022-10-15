mod agent;

pub use agent::WasmAgent;

use crate::{Action, Register};
use std::mem::size_of;
use wasmer::{FromToNativeWasmType, Instance, MemoryView, ValueType};

unsafe impl FromToNativeWasmType for Action {
    type Native = i32;

    fn from_native(native: Self::Native) -> Self {
        match native {
            0 => Action::Left,
            1 => Action::Up,
            2 => Action::Right,
            3 => Action::Down,
            _ => Action::Null,
        }
    }

    fn to_native(self) -> Self::Native {
        self as i32
    }
}

unsafe impl ValueType for Register {}

#[derive(Debug)]
pub struct Engine {
    register: Register,
}

impl Engine {
    pub fn new(instance: &Instance) -> Self {
        let memory = instance.exports.get_memory("memory").unwrap();
        let register_global = instance.exports.get_global("BOARD").unwrap();
        let register_addr = register_global.get().i32().unwrap() as usize;
        println!("Register address is {:x}", register_addr);
        println!("Size of register is {}", size_of::<Register>());
        let register_binding: MemoryView<Register> = memory.view::<Register>();
        let register = register_binding
            .get(register_addr / size_of::<Register>())
            .unwrap()
            .get();
        let mut engine = Engine { register };
        engine.new_game();
        println!("Engine is {:?}", engine);
        engine
    }

    pub fn new_game(&mut self) {
        self.register.clear();
        self.add_tile();
        self.add_tile();
    }

    pub fn add_tile(&mut self) {
        self.register.left[0][0] = 1;
    }
}
