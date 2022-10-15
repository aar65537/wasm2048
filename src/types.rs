#[repr(C)]
pub enum Action {
    Left,
    Up,
    Right,
    Down,
    Null,
}

type Board = [[u32; 4]; 4];
const DEFAULT_BOARD: Board = [[0; 4]; 4];

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Register {
    pub left: Board,
    pub up: Board,
    pub right: Board,
    pub down: Board,
}

impl Register {
    pub const fn default() -> Self {
        Register {
            left: DEFAULT_BOARD,
            up: DEFAULT_BOARD,
            right: DEFAULT_BOARD,
            down: DEFAULT_BOARD,
        }
    }

    pub fn clear(&mut self) {

    }
}
