use common::timed;

use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: isize,
    height: isize,

    nodes: HashMap<char, Vec<Position>>,
    antinodes: HashSet<Position>,
}
impl Grid {
    fn is_inside(&self, pos: &Position) -> bool {
        let is_negative = pos.x < 0 || pos.y < 0;
        let is_too_large = pos.x >= self.width || pos.y >= self.height;
        !is_negative && !is_too_large
    }
}

fn create_antinodes(grid: Grid) -> Grid {
    let mut antinodes = HashSet::default();

    for (_, positions) in grid.nodes.iter() {
        for pos in positions {
            let other_positions = positions.iter().filter(|p| *p != pos).map(|other| {
                let vector = Position::new(other.x - pos.x, other.y - pos.y);

                (*other, vector)
            });

            other_positions.for_each(|(other_pos, vec)| {
                // Add antinode to (other_pos + vec) and (current_pos - vec)
                [(other_pos + vec), (*pos - vec)]
                    .into_iter()
                    .filter(|p| grid.is_inside(p))
                    .for_each(|p| {
                        antinodes.insert(p);
                    });
            });
        }
    }

    Grid { antinodes, ..grid }
}

fn create_antinodes_extended(grid: Grid) -> Grid {
    let mut antinodes = HashSet::default();

    for (_, positions) in grid.nodes.iter() {
        for pos in positions {
            let other_positions = positions.iter().filter(|p| *p != pos).map(|other| {
                let vector = Position::new(other.x - pos.x, other.y - pos.y);

                (*other, vector)
            });

            other_positions.for_each(|(other_pos, vec)| {
                // Add antinode to (other_pos + vec) and (current_pos - vec)
                // Plus on antenna itself
                // Repeat while inside grid
                let mut node_pos = other_pos;
                while grid.is_inside(&node_pos) {
                    antinodes.insert(node_pos);
                    node_pos = node_pos + vec;
                }

                node_pos = *pos;
                while grid.is_inside(&node_pos) {
                    antinodes.insert(node_pos);
                    node_pos = node_pos - vec;
                }
            });
        }
    }

    Grid { antinodes, ..grid }
}

fn create_grid(input: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;
    let mut nodes = HashMap::<_, _>::default();
    let mut antinodes = HashSet::default();

    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if width < x {
                width = x;
            }

            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    nodes
                        .entry(ch)
                        .and_modify(|pos: &mut Vec<Position>| {
                            pos.push(Position::new(x as isize, y as isize))
                        })
                        .or_insert(vec![Position::new(x as isize, y as isize)]);
                }

                '#' => {
                    antinodes.insert(Position::new(x as isize, y as isize));
                }
                _ => {}
            }
        }

        height = y;
    }

    Grid {
        width: (width + 1) as isize,
        height: (height + 1) as isize,
        nodes,
        antinodes,
    }
}

fn main() {
    let grid = create_grid(&common::read_stdin());

    let (time, with_antinodes) = timed(|| create_antinodes(grid.clone()));
    println!(
        "Part 1: {} in {}μs",
        with_antinodes.antinodes.len(),
        time.as_micros()
    );

    let (time, with_antinodes_extended) = timed(|| create_antinodes_extended(grid));
    println!(
        "Part 2: {} in {}μs",
        with_antinodes_extended.antinodes.len(),
        time.as_micros()
    );
}

// Part 1: 299 in 70μs
// Part 2: 1032 in 267μs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grid_test() {
        let grid = create_grid(include_str!("../example.txt"));

        assert_eq!(grid.width, 12);
        assert_eq!(grid.height, 12);

        assert_eq!(
            grid.nodes,
            HashMap::from_iter([
                (
                    '0',
                    vec![
                        Position::new(8, 1),
                        Position::new(5, 2),
                        Position::new(7, 3),
                        Position::new(4, 4),
                    ]
                ),
                (
                    'A',
                    vec![
                        Position::new(6, 5),
                        Position::new(8, 8),
                        Position::new(9, 9),
                    ]
                ),
            ])
        );
    }

    #[test]
    fn create_antinodes_test() {
        let grid = create_grid(include_str!("../example.txt"));
        let grid = create_antinodes(grid);
        assert_eq!(grid.antinodes.len(), 14);
    }

    #[test]
    fn create_antinodes_extended_test() {
        let grid = create_grid(include_str!("../example.txt"));
        let grid = create_antinodes_extended(grid);
        assert_eq!(grid.antinodes.len(), 34);
    }
}
