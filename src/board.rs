use crate::piece::Piece;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

pub struct Board {
    width: usize,
    height: usize,
    board: [[u8; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            board: [[0u8; WIDTH]; HEIGHT],
        }
    }

    pub fn detect_collision(&self, piece: Piece, row: u16, column: u16) -> bool {
        false
    }
}
