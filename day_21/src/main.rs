#![allow(clippy::comparison_chain)]

use common::{timed, Pos};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[derive(Debug, Hash, PartialEq, Eq)]
struct MemoKey {
    cursor: Pos,
    dest: Pos,
    depth: u8,
}
type Memo = HashMap<MemoKey, u64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeypadButton {
    Number(u8),
    Enter,
    Empty,
}
impl KeypadButton {
    fn pos(&self) -> Pos {
        match self {
            KeypadButton::Number(1) => Pos { x: 0, y: 2 },
            KeypadButton::Number(2) => Pos { x: 1, y: 2 },
            KeypadButton::Number(3) => Pos { x: 2, y: 2 },
            KeypadButton::Number(4) => Pos { x: 0, y: 1 },
            KeypadButton::Number(5) => Pos { x: 1, y: 1 },
            KeypadButton::Number(6) => Pos { x: 2, y: 1 },
            KeypadButton::Number(7) => Pos { x: 0, y: 0 },
            KeypadButton::Number(8) => Pos { x: 1, y: 0 },
            KeypadButton::Number(9) => Pos { x: 2, y: 0 },
            KeypadButton::Number(0) => Pos { x: 1, y: 3 },
            KeypadButton::Number(x) => panic!("invalid key: {x}"),

            KeypadButton::Enter => Pos { x: 2, y: 3 },
            KeypadButton::Empty => Pos { x: 0, y: 3 },
        }
    }
}
impl From<KeypadButton> for Pos {
    fn from(value: KeypadButton) -> Self {
        value.pos()
    }
}
impl Display for KeypadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        match self {
            Self::Number(x) => f.write_str(&x.to_string()),
            Self::Enter => f.write_char('A'),
            Self::Empty => f.write_char(' '),
        }
    }
}

#[derive(Debug)]
struct Keypad;
impl Keypad {
    fn complexity(target: &str, depth: u8) -> u64 {
        let cost = Self::cheapest(target, depth);
        let number: u64 = target.strip_suffix('A').unwrap().parse().unwrap();
        cost * number
    }

    fn cheapest(target: &str, depth: u8) -> u64 {
        let buttons = target.chars().map(|c| match c {
            x @ '0'..='9' => KeypadButton::Number(x as u8 - b'0'),
            'A' => KeypadButton::Enter,
            x => panic!("Unexpected key: {x}"),
        });

        let mut memo = Memo::new();

        let mut cursor = KeypadButton::Enter.pos();
        let mut total_cost = 0;
        for btn in buttons {
            let dest = btn.pos();
            total_cost += Self::cheapest_path(depth, &cursor, &dest, &mut memo);
            cursor = dest;
        }

        total_cost
    }

    fn cheapest_path(depth: u8, cursor: &Pos, dest: &Pos, memo: &mut Memo) -> u64 {
        let mut queue = VecDeque::from_iter([Visit {
            pos: *cursor,
            ..Default::default()
        }]);
        let mut total_cost = u64::MAX;

        while let Some(next) = queue.pop_front() {
            if next.pos == dest {
                let moves = {
                    let mut m = next.moves.clone();
                    m.push(Move::Activate);
                    m
                };
                let cost = DirectionalPad::cheapest(&moves, depth, memo);
                total_cost = total_cost.min(cost);
            } else if next.pos == KeypadButton::Empty.pos() {
                continue;
            } else {
                if next.pos.y < dest.y {
                    queue.push_back(next.move_down());
                } else if next.pos.y > dest.y {
                    queue.push_back(next.move_up());
                }

                if next.pos.x < dest.x {
                    queue.push_back(next.move_right());
                } else if next.pos.x > dest.x {
                    queue.push_back(next.move_left());
                }
            }
        }

        total_cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectionalButton {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Empty,
}
impl From<DirectionalButton> for Pos {
    fn from(value: DirectionalButton) -> Self {
        value.pos()
    }
}
impl DirectionalButton {
    fn pos(&self) -> Pos {
        match self {
            DirectionalButton::Up => Pos { x: 1, y: 0 },
            DirectionalButton::Down => Pos { x: 1, y: 1 },
            DirectionalButton::Left => Pos { x: 0, y: 1 },
            DirectionalButton::Right => Pos { x: 2, y: 1 },
            DirectionalButton::Enter => Pos { x: 2, y: 0 },
            DirectionalButton::Empty => Pos { x: 0, y: 0 },
        }
    }
}

#[derive(Debug)]
struct DirectionalPad;
impl DirectionalPad {
    fn cheapest(moves: &[Move], depth: u8, memo: &mut Memo) -> u64 {
        if depth == 1 {
            return moves.len() as u64;
        }

        let mut cost = 0;
        let mut cursor = DirectionalButton::Enter.pos();
        for m in moves {
            let dest = m.as_dir_btn().pos();
            cost += Self::cheapest_path(&cursor, &dest, depth, memo);
            cursor = dest;
        }

        cost
    }

    fn cheapest_path(cursor: &Pos, dest: &Pos, depth: u8, memo: &mut Memo) -> u64 {
        if let Some(cached) = memo.get(&MemoKey {
            cursor: *cursor,
            dest: *dest,
            depth,
        }) {
            return *cached;
        }

        let mut queue = VecDeque::from_iter([Visit {
            pos: *cursor,
            ..Default::default()
        }]);
        let mut total_cost = u64::MAX;

        while let Some(next) = queue.pop_front() {
            if next.pos == dest {
                let moves = {
                    let mut m = next.moves.clone();
                    m.push(Move::Activate);
                    m
                };
                let cost = Self::cheapest(&moves, depth - 1, memo);
                total_cost = total_cost.min(cost);
            } else if next.pos == DirectionalButton::Empty.pos() {
                continue;
            } else {
                if next.pos.y < dest.y {
                    queue.push_back(next.move_down());
                } else if next.pos.y > dest.y {
                    queue.push_back(next.move_up());
                }

                if next.pos.x < dest.x {
                    queue.push_back(next.move_right());
                } else if next.pos.x > dest.x {
                    queue.push_back(next.move_left());
                }
            }
        }

        memo.insert(
            MemoKey {
                cursor: *cursor,
                dest: *dest,
                depth,
            },
            total_cost,
        );
        total_cost
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Visit {
    pos: Pos,
    moves: Vec<Move>,
}
impl Visit {
    fn move_left(&self) -> Self {
        Self {
            pos: Pos {
                x: self.pos.x - 1,
                ..self.pos
            },
            moves: {
                let mut m = self.moves.clone();
                m.push(Move::Left);
                m
            },
        }
    }

    fn move_right(&self) -> Self {
        Self {
            pos: Pos {
                x: self.pos.x + 1,
                ..self.pos
            },
            moves: {
                let mut m = self.moves.clone();
                m.push(Move::Right);
                m
            },
        }
    }

    fn move_up(&self) -> Self {
        Self {
            pos: Pos {
                y: self.pos.y - 1,
                ..self.pos
            },
            moves: {
                let mut m = self.moves.clone();
                m.push(Move::Up);
                m
            },
        }
    }

    fn move_down(&self) -> Self {
        Self {
            pos: Pos {
                y: self.pos.y + 1,
                ..self.pos
            },
            moves: {
                let mut m = self.moves.clone();
                m.push(Move::Down);
                m
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Activate,
}
impl Move {
    fn char(&self) -> char {
        match self {
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Right => '>',
            Move::Activate => 'A',
        }
    }
    fn as_dir_btn(&self) -> DirectionalButton {
        match self {
            Move::Up => DirectionalButton::Up,
            Move::Down => DirectionalButton::Down,
            Move::Left => DirectionalButton::Left,
            Move::Right => DirectionalButton::Right,
            Move::Activate => DirectionalButton::Enter,
        }
    }
}
impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char(self.char())
    }
}

fn main() {
    let input = common::read_stdin()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let (time, complexity): (_, u64) =
        timed(|| input.iter().map(|x| Keypad::complexity(x, 3)).sum());
    println!("Part 1: {complexity} in {}μs", time.as_micros());

    let (time, complexity): (_, u64) =
        timed(|| input.iter().map(|x| Keypad::complexity(x, 26)).sum());
    println!("Part 2: {complexity} in {}μs", time.as_micros());
}

// Part 1: 138764 in 207μs
// Part 2: 169137886514152 in 2593μs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cost = Keypad::cheapest("029A", 3);
        assert_eq!(cost, 68);
        assert_eq!(Keypad::complexity("029A", 3), 68 * 29);
    }
}
