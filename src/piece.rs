use rand::Rng;

/// Contains all of the possible pieces as well as their rotations
/// All of them have unique rotations except for the O (Square Block) Piece
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

/// Returns the piece that the enum represents
pub fn get_piece(piece: Piece) -> &'static [[u8; 4]; 4] {
    match piece {
        Piece::O => O,

        Piece::I => I0,
        Piece::IR => IR,
        Piece::I2 => I2,
        Piece::IL => IL,

        Piece::J => J0,
        Piece::JR => JR,
        Piece::J2 => J2,
        Piece::JL => JL,

        Piece::L => L0,
        Piece::LR => LR,
        Piece::L2 => L2,
        Piece::LL => LL,

        Piece::S => S0,
        Piece::SR => SR,
        Piece::S2 => S2,
        Piece::SL => SL,

        Piece::T => T0,
        Piece::TR => TR,
        Piece::T2 => T2,
        Piece::TL => TL,

        Piece::Z => Z0,
        Piece::ZR => ZR,
        Piece::Z2 => Z2,
        Piece::ZL => ZL,
    }
}

/// Returns the rotation of the piece in a clockwise fashion
pub fn rotate(piece: Piece) -> Piece {
    match piece {
        Piece::O => Piece::O,

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

/// Returns a random piece in the initial spawn orientation
pub fn random_piece() -> Piece {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=6) {
        0 => Piece::I,
        1 => Piece::J,
        2 => Piece::L,
        3 => Piece::O,
        4 => Piece::S,
        5 => Piece::T,
        6 => Piece::Z,
        _ => panic!("RNG went out of range for random piece."),
    }
}
