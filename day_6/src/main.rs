use std::collections::HashSet;

use common::{read_stdin, timed};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
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
impl std::ops::Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct Guard {
    position: Position,
    direction: Direction,
}
impl Guard {
    fn step(&self, grid: &Grid) -> Guard {
        let mut direction = self.direction;
        let mut next_pos = self.position + direction.vector();
        let mut loop_count = 0;

        while grid.is_obstruction(&next_pos) {
            if loop_count >= 4 {
                panic!("Infinite loop?");
            }

            direction = direction.rotate();
            next_pos = self.position + direction.vector();

            loop_count += 1;
        }

        Guard {
            position: next_pos,
            direction,
        }
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

    fn with_obstruction(mut self, obstruction: Position) -> Self {
        self.obstructions.insert(obstruction);
        self
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
    let mut visited = HashSet::<_>::from_iter([guard.position]);

    while guard.position.is_inside_grid(grid) {
        guard = guard.step(grid);
        if guard.position.is_inside_grid(grid) {
            visited.insert(guard.position);
        }
    }

    visited
}

/// Returns if hit max iter
fn get_in_loop(grid: &Grid, mut guard: Guard) -> bool {
    let mut visited = HashSet::<_>::from_iter([guard.clone()]);

    while guard.position.is_inside_grid(grid) {
        guard = guard.step(grid);
        if visited.contains(&guard) {
            return true;
        } else if guard.position.is_inside_grid(grid) {
            visited.insert(guard.clone());
        } else {
            break;
        }
    }

    false
}

fn create_loops(grid: &Grid, guard: Guard) -> i32 {
    let mut loops = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            let new_grid = grid.clone().with_obstruction(Position { x, y });

            if get_in_loop(&new_grid, guard.clone()) {
                loops += 1;
            }
        }
    }

    loops
}

fn main() {
    let input = read_stdin();
    let (grid, guard) = build_grid(&input);

    let (time, visited) = timed(|| get_visited_squares(&grid, guard.clone()));
    println!("Part 1: {} in {}μs", visited.len(), time.as_micros());

    let (time, loops) = timed(|| create_loops(&grid, guard.clone()));
    println!("Part 2: {loops} in {}s", time.as_secs());
}

// Part 1: 5131 in 1231μs
// Part 2: 1784 in 17s

#[cfg(test)]
mod tests {
    use crate::*;

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

    #[test]
    fn part2() {
        let (grid, guard) = build_grid(include_str!("../example.txt"));

        let loops = create_loops(&grid, guard);
        assert_eq!(loops, 6);
    }
}
