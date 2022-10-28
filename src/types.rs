use crate::{Board, EMPTY_BOARD};

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
    pub left: Board,
    pub up: Board,
    pub right: Board,
    pub down: Board,
    pub null: Board,
}

impl Register {
    pub const fn default() -> Self {
        Register {
            left: EMPTY_BOARD,
            up: EMPTY_BOARD,
            right: EMPTY_BOARD,
            down: EMPTY_BOARD,
            null: EMPTY_BOARD,
        }
    }

    pub fn clear(&mut self) {}

    pub fn add_tile(&mut self) {
        self.left[0][0] = 1;
    }
}
