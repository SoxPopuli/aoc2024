use crate::Pos;

pub const UP: Pos = Pos { x: 0, y: -1 };
pub const RIGHT: Pos = Pos { x: 1, y: 0 };
pub const DOWN: Pos = Pos { x: 0, y: 1 };
pub const LEFT: Pos = Pos { x: -1, y: 0 };

pub const ALL: [Pos; 4] = [UP, RIGHT, DOWN, LEFT];

pub fn opposite(v: Pos) -> Option<Pos> {
    match v {
        UP => Some(DOWN),
        DOWN => Some(UP),
        RIGHT => Some(LEFT),
        LEFT => Some(RIGHT),

        _ => None,
    }
}
