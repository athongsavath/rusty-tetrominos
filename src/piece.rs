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

    IL,
    JL,
    LL,
    SL,
    TL,
    ZL,

    IR,
    JR,
    LR,
    SR,
    TR,
    ZR,

    I2,
    J2,
    L2,
    S2,
    T2,
    Z2,
}

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

pub fn rotate(piece: Piece) -> Piece {
    match piece {
        Piece::O => Piece::O,

        // Not sure how to handle I piece
        Piece::I => Piece::IR,
        Piece::IR => Piece::I2,
        Piece::I2 => Piece::IL,
        Piece::IL => Piece::I,

        Piece::J => Piece::JR,
        Piece::JR => Piece::J2,
        Piece::J2 => Piece::JL,
        Piece::JL => Piece::J,

        Piece::L => Piece::LR,
        Piece::LR => Piece::L2,
        Piece::L2 => Piece::LL,
        Piece::LL => Piece::L,

        Piece::S => Piece::SR,
        Piece::SR => Piece::S2,
        Piece::S2 => Piece::SL,
        Piece::SL => Piece::S,

        Piece::T => Piece::TR,
        Piece::TR => Piece::T2,
        Piece::T2 => Piece::TL,
        Piece::TL => Piece::T,

        Piece::Z => Piece::ZR,
        Piece::ZR => Piece::Z2,
        Piece::Z2 => Piece::ZL,
        Piece::ZL => Piece::Z,
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
