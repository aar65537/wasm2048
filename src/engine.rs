mod agent;

pub use agent::WasmAgent;

use wasmer::FromToNativeWasmType;
use crate::Action;

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