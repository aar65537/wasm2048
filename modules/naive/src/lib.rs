use wasm2048::{Action, Register};

#[no_mangle]
pub static mut REGISTER: Register = Register::default();

#[no_mangle]
pub extern "C" fn evaluate(_action: Action) -> f32 {
    if unsafe { REGISTER.left[0][0] } == 0 {
        return 10.;
    }
    0.
}

#[no_mangle]
pub extern "C" fn update(_reward: i32, _action: Action) {
    unsafe { REGISTER.left[0][3] = 5 }
}
