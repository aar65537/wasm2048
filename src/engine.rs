mod agent;
mod engine;

pub use engine::Engine;
pub use agent::WasmAgent;

use crate::{Action, Register};
use wasmer::{FromToNativeWasmType, ValueType};

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