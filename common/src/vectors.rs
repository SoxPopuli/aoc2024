use crate::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vector {
    Up,
    Right,
    Down,
    Left,

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
impl Vector {
    pub fn to_pos(&self) -> Pos {
        use Vector::*;
        match self {
            Up => UP,
            Right => RIGHT,
            Down => DOWN,
            Left => LEFT,
            UpLeft => UP_LEFT,
            UpRight => UP_RIGHT,
            DownLeft => DOWN_LEFT,
            DownRight => DOWN_RIGHT,
        }
    }

    pub fn rotate_clockwise(&self) -> Self {
        use Vector::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
            UpLeft => UpRight,
            UpRight => DownRight,
            DownLeft => UpLeft,
            DownRight => DownLeft,
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Self {
        use Vector::*;
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
            UpLeft => DownLeft,
            UpRight => UpLeft,
            DownLeft => DownRight,
            DownRight => UpRight,
        }
    }

    pub fn cardinal() -> [Vector; 4] {
        use Vector::*;
        [Up, Right, Down, Left]
    }

    pub fn diagonal() -> [Vector; 4] {
        use Vector::*;
        [UpLeft, UpRight, DownLeft, DownRight]
    }

    pub fn all() -> [Vector; 8] {
        use std::mem::transmute;
        unsafe { transmute([Vector::cardinal(), Vector::diagonal()]) }
    }
}
impl From<Vector> for Pos {
    fn from(value: Vector) -> Self {
        value.to_pos()
    }
}

pub const UP: Pos = Pos::new(0, -1);
pub const RIGHT: Pos = Pos::new(1, 0);
pub const DOWN: Pos = Pos::new(0, 1);
pub const LEFT: Pos = Pos::new(-1, 0);

pub const CARDINAL: [Pos; 4] = [UP, RIGHT, DOWN, LEFT];

pub const UP_LEFT: Pos = Pos::new(-1, -1);
pub const UP_RIGHT: Pos = Pos::new(1, -1);
pub const DOWN_LEFT: Pos = Pos::new(-1, 1);
pub const DOWN_RIGHT: Pos = Pos::new(1, 1);

pub const DIAGONAL: [Pos; 4] = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];

pub const ALL: [Pos; 8] = [
    UP, RIGHT, DOWN, LEFT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];
