use std::ops::{Add, Sub};

use common::timed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}
impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

mod vectors {
    use super::Pos;

    pub const UP: Pos = Pos { x: 0, y: -1 };
    pub const RIGHT: Pos = Pos { x: 1, y: 0 };
    pub const DOWN: Pos = Pos { x: 0, y: 1 };
    pub const LEFT: Pos = Pos { x: -1, y: 0 };

    pub const ALL: [Pos; 4] = [UP, RIGHT, DOWN, LEFT];
}

type GridNode = u8;

#[derive(Debug, Clone)]
struct Grid {
    width: isize,
    height: isize,
    data: Vec<Vec<GridNode>>,
}
impl Grid {
    fn new(input: &str) -> Self {
        fn char_to_digit(c: char) -> u8 {
            match c {
                '.' => u8::MAX,
                x => x as u8 - b'0',
            }
        }

        let data: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(char_to_digit).collect())
            .collect();

        Self {
            width: data[0].len() as isize,
            height: data.len() as isize,
            data,
        }
    }

    fn get(&self, x: isize, y: isize) -> GridNode {
        assert!(self.is_inside(x, y), "Pos: [{x}, {y}] not inside grid");

        self.data[y as usize][x as usize]
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut GridNode {
        assert!(self.is_inside(x, y), "Pos: [{x}, {y}] not inside grid");

        &mut self.data[y as usize][x as usize]
    }

    fn is_inside(&self, x: isize, y: isize) -> bool {
        let is_negative = x < 0 || y < 0;
        let is_outside = x >= self.width || y >= self.height;

        !is_outside && !is_negative
    }

    fn iter(&self) -> std::iter::FromFn<impl FnMut() -> Option<(GridNode, Pos)> + use<'_>> {
        let mut x = 0;
        let mut y = 0;

        std::iter::from_fn(move || {
            if x >= self.width {
                x = 0;
                y += 1;
            }

            if y >= self.height {
                None
            } else {
                let value = self.get(x, y);
                let ret = (value, Pos { x, y });
                x += 1;
                Some(ret)
            }
        })
    }
}

fn search_for_trails(grid: &mut Grid, pos: &Pos) -> u32 {
    let current = grid.get(pos.x, pos.y);

    let mut score = 0;

    for vec in vectors::ALL {
        let next_pos = *pos + vec;
        if !grid.is_inside(next_pos.x, next_pos.y) {
            continue;
        }

        let next_node = grid.get_mut(next_pos.x, next_pos.y);
        if current == 8 && *next_node == 9 {
            score += 1;

            // set to u8 max to remove from grid
            *next_node = u8::MAX;
        } else if *next_node == current + 1 {
            score += search_for_trails(grid, &next_pos);
        }
    }

    score
}

fn find_trails(grid: &Grid) -> u32 {
    let scores = grid
        .iter()
        .filter(|(n, _)| *n == 0)
        .map(|(_, pos)| search_for_trails(&mut grid.clone(), &pos));

    scores.sum::<u32>()
}

fn search_for_trails_distinct(grid: &Grid, pos: &Pos) -> u32 {
    let current = grid.get(pos.x, pos.y);

    let mut score = 0;

    for vec in vectors::ALL {
        let next_pos = *pos + vec;
        if !grid.is_inside(next_pos.x, next_pos.y) {
            continue;
        }

        let next_node = grid.get(next_pos.x, next_pos.y);

        if current == 8 && next_node == 9 {
            score += 1;
        } else if next_node == current + 1 {
            score += search_for_trails_distinct(grid, &next_pos);
        }
    }

    score
}

fn find_trails_distinct(grid: &Grid) -> u32 {
    let scores = grid
        .iter()
        .filter(|(n, _)| *n == 0)
        .map(|(_, pos)| search_for_trails_distinct(grid, &pos));

    scores.sum::<u32>()
}

fn main() {
    let grid = Grid::new(&common::read_stdin());

    let (time, score) = timed(|| find_trails(&grid));
    println!("Part 1: {score} in {}μs", time.as_micros());

    let (time, score) = timed(|| find_trails_distinct(&grid));
    println!("Part 2: {score} in {}μs", time.as_micros());
}

// Part 1: 776 in 1098μs
// Part 2: 1657 in 326μs

#[cfg(test)]
mod tests {
    use crate::{find_trails, find_trails_distinct, Grid};

    #[test]
    fn grid_test() {
        let grid = Grid::new(include_str!("../example.txt"));

        assert_eq!(grid.width, 8);
        assert_eq!(grid.height, 8);

        assert_eq!(grid.get(0, 0), 8);
        assert_eq!(grid.get(0, 1), 7);
        assert_eq!(grid.get(1, 0), 9);
        assert_eq!(grid.get(4, 6), 9);
    }

    #[test]
    fn part1() {
        let grid = Grid::new(include_str!("../example_simple.txt"));
        assert_eq!(find_trails(&grid), 4);

        let grid = Grid::new(include_str!("../example.txt"));
        assert_eq!(find_trails(&grid), 36);
    }

    #[test]
    fn part2() {
        let grid = Grid::new(include_str!("../example_distinct.txt"));
        assert_eq!(find_trails_distinct(&grid), 3);

        let grid = Grid::new(include_str!("../example.txt"));
        assert_eq!(find_trails_distinct(&grid), 81);
    }
}
