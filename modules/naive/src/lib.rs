use wasm2048::Action;

#[no_mangle]
pub extern "C" fn evaluate(_action: Action) -> f32 {
    0.
}

#[no_mangle]
pub extern "C" fn update(_reward: i32, _action: Action) {}
