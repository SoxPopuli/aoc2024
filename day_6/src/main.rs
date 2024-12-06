use std::collections::{ HashSet, HashMap };

use common::read_stdin;

#[derive(Debug, Clone, Copy, Default)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn vector(&self) -> Position {
        match self {
            Self::Up => Position { x: 0, y: -1 },
            Self::Right => Position { x: 1, y: 0 },
            Self::Down => Position { x: 0, y: 1 },
            Self::Left => Position { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}
impl std::ops::Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_inside_grid(&self, grid: &Grid) -> bool {
        let negative_position = self.x < 0 || self.y < 0;
        let pos_too_high = self.x >= grid.width || self.y >= grid.height;

        !(negative_position || pos_too_high)
    }
}

#[derive(Debug, Default, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}
impl Guard {
    fn step(&mut self, grid: &Grid) -> Position {
        let vector = self.direction.vector();
        let mut next_pos = self.position + vector;
        let mut loop_count = 0;

        while grid.is_obstruction(&next_pos) {
            if loop_count >= 4 {
                panic!("Infinite loop?");
            }

            self.direction = self.direction.rotate();
            next_pos = self.position + self.direction.vector();

            loop_count += 1;
        }

        self.position = next_pos;

        next_pos
    }
}

#[derive(Debug, Default, Clone)]
struct Grid {
    width: isize,
    height: isize,
    obstructions: HashSet<Position>,
}
impl Grid {
    fn is_obstruction(&self, pos: &Position) -> bool {
        self.obstructions.contains(pos)
    }
}

fn build_grid(input: &str) -> (Grid, Guard) {
    let mut width = 0;
    let mut height = 0;

    let mut guard = Guard::default();
    let mut grid = Grid::default();

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if x > width {
                width = x;
            }

            match c {
                '^' => {
                    guard = Guard {
                        position: Position::new(x as isize, y as isize),
                        direction: Direction::Up,
                    };
                }
                '#' => {
                    grid.obstructions
                        .insert(Position::new(x as isize, y as isize));
                }

                _ => {}
            }
        }

        height = y;
    }

    grid.width = width as isize + 1;
    grid.height = height as isize + 1;

    (grid, guard)
}

fn get_visited_squares(grid: &Grid, mut guard: Guard) -> HashSet<Position> {
    let mut visited = HashSet::<_>::from_iter([
        guard.position
    ]);

    while guard.position.is_inside_grid(grid) {
        let new_pos = guard.step(grid);
        if new_pos.is_inside_grid(grid) {
            visited.insert(new_pos);
        }
    }

    visited
}

fn is_looping(grid: &Grid, mut guard: Guard) {
    
}

fn part2(grid: &Grid, mut guard: Guard) {
    // Run part1 to get visited squares
    let visited = get_visited_squares(grid, guard.clone());
    // Limit obstruction placement to in front of visited squares
    // (No point in obstructing a square they never visit)

    // Place new obstructions for each direction per visited square
}

fn main() {
    let input = read_stdin();
    let (grid, guard) = build_grid(&input);

    let visited = get_visited_squares(&grid, guard.clone());
    println!("Part 1: {}", visited.len());
}

#[cfg(test)]
mod tests {
    use crate::{build_grid, get_visited_squares, Position};

    #[test]
    fn part1() {
        let (grid, guard) = build_grid(include_str!("../example.txt"));

        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);

        assert_eq!(grid.obstructions.len(), 8);

        assert_eq!(guard.position, Position { x: 4, y: 6 });

        let visited = get_visited_squares(&grid, guard);

        assert_eq!(visited.len(), 41);
    }
}
