mod agent;

pub use agent::WasmAgent;

use wasmer::{FromToNativeWasmType, Instance, Global, ValueType};
use crate::{Action, Register};

unsafe impl FromToNativeWasmType for Action{
    type Native = i32;

    fn from_native(native: Self::Native) -> Self {
        match native {
            0 => Action::Left,
            1 => Action::Up,
            2 => Action::Right,
            3 => Action::Down,
            _ => Action::Null
        }
    }

    fn to_native(self) -> Self::Native {
        self as i32
    }
}

unsafe impl ValueType for Register {}

pub struct Engine {
    // register: &'a Register,
}

impl<'a> Engine {

    pub fn new(instance: &'a Instance) -> Self {
        let board: &'a Global = instance.exports.get_global("BOARD").unwrap();
        let board_type = board.ty();
        let board_value = board.get();
        let memory = instance.exports.get_memory("memory").unwrap();
        let binding = memory.view::<u8>();
        let view = binding.get(1048576);
        println!("Board type = {:?}", board_type);
        println!("Board value = {:?}", board_value);
        println!("Board = {:?}", view);
        Engine {}
    }
}