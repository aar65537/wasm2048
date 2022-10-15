#[repr(C)]
pub enum Action {
    Left,
    Up,
    Right,
    Down,
    Null,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Register {
    pub left: [u32; 16],
}

impl Register {
    pub fn default() -> Self {
        Register { left: [0;16] }
    }
}