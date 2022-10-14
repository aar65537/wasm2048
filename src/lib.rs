#[cfg(feature = "engine")]
use wasmer::FromToNativeWasmType;

#[cfg(feature = "engine")]
pub mod agent;

#[repr(C)]
pub enum Action {
    Left,
    Up,
    Right,
    Down,
}

#[cfg(feature = "engine")]
unsafe impl FromToNativeWasmType for Action{
    type Native = i32;

    fn from_native(native: Self::Native) -> Self {
        match native {
            0 => Action::Left,
            1 => Action::Up,
            2 => Action::Right,
            3 => Action::Down,
            _ => panic!("Wasm didn't return valid action.")
        }
    }

    fn to_native(self) -> Self::Native {
        self as i32
    }
}

#[repr(C)]
pub struct Prediction {
    pub left: i32,
    pub up: i32,
    pub right: i32,
    pub down: i32,
}