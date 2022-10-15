use wasm2048::{Action, Register};

#[no_mangle]
pub static mut BOARD: Register = Register { left: [10; 16] };

#[no_mangle]
pub extern "C" fn evaluate(_action: Action) -> f32 {
    0.
}

#[no_mangle]
pub extern "C" fn update(_reward: i32, _action: Action) {}
