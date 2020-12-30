use rand::Rng;

#[derive(Clone, Copy)]
pub enum Piece {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

//struct Piece {}

//impl Piece {
//fn rotate() {}
//}

//pub trait Rotate {
//fn rotate_cw(&self) -> Self;
//fn rotate_ccw(&self) -> Self;
//}

pub static O: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]];

pub static I0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]];
pub static J0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]];
pub static L0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]];
pub static S0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
pub static T0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]];
pub static Z0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]];

pub static IR: &'static [[u8; 4]; 4] = &[[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]];
pub static JR: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
pub static LR: &'static [[u8; 4]; 4] = &[[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
pub static SR: &'static [[u8; 4]; 4] = &[[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
pub static TR: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0]];
pub static ZR: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0]];

pub static I2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]];
pub static J2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0]];
pub static L2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0]];
pub static S2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0]];
pub static T2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0]];
pub static Z2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0]];

pub static IL: &'static [[u8; 4]; 4] = &[[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
pub static JL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0]];
pub static LL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
pub static SL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]];
pub static TL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]];
pub static ZL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0]];

struct IPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl IPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &I0 }
    }
}

impl JPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &J0 }
    }
}

struct JPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl LPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &L0 }
    }
}

struct LPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl OPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &O }
    }
}

struct OPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

struct SPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl SPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &S0 }
    }
}

struct TPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl TPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &T0 }
    }
}

struct ZPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl ZPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &Z0 }
    }
}

pub fn random_piece() -> Piece {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..7) {
        0 => Piece::I,
        1 => Piece::J,
        2 => Piece::L,
        3 => Piece::O,
        4 => Piece::S,
        5 => Piece::T,
        6 => Piece::Z,
        _ => panic!("This should not be possible"),
    }
}
