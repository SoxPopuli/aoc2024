use crate::Pos;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,

    pub data: Vec<Vec<T>>,
}
impl<T> Grid<T> {
    pub fn get(&self, p @ Pos { x, y }: &Pos) -> Option<&T> {
        if self.is_inside(p) {
            Some(&self.data[*y as usize][*x as usize])
        } else {
            None
        }
    }

    pub fn is_inside(&self, Pos { x, y }: &Pos) -> bool {
        let is_negative = *x < 0 || *y < 0;
        let is_outside = *x >= self.width || *y >= self.height;

        !is_outside && !is_negative
    }

    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self {
            width: data[0].len() as isize,
            height: data.len() as isize,
            data,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'_ T, Pos)> {
        self.data.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| (c, (x, y).into()))
        })
    }

    /// Iterates over all valid diagonal directions
    pub fn iter_adjacent_diagonal(&self, p: Pos) -> impl Iterator<Item = (&'_ T, Pos)> {
        crate::vectors::DIAGONAL
            .iter()
            .map(move |v| p + *v)
            .filter_map(|p| self.get(&p).map(|x| (x, p)))
    }

    /// Iterates over all valid cardinal directions
    pub fn iter_adjacent_cardinal(&self, p: Pos) -> impl Iterator<Item = (&'_ T, Pos)> {
        crate::vectors::CARDINAL
            .iter()
            .map(move |v| p + *v)
            .filter_map(|p| self.get(&p).map(|x| (x, p)))
    }

    /// Iterates over all valid directions
    pub fn iter_adjacent(&self, p: Pos) -> impl Iterator<Item = (&'_ T, Pos)> {
        crate::vectors::ALL
            .iter()
            .map(move |v| p + *v)
            .filter_map(|p| self.get(&p).map(|x| (x, p)))
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn swap(&mut self, a: Pos, b: Pos) {
        let tmp = self[a].clone();
        self[a] = self[b].clone();
        self[b] = tmp;
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                self[(x, y)].fmt(f)?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
impl<T> Grid<T>
where
    T: Display,
{
    pub fn to_char_grid(&self) -> Grid<char> {
        let grid = self
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x.to_string().chars().next().unwrap())
                    .collect()
            })
            .collect();

        Grid {
            width: self.width,
            height: self.height,
            data: grid,
        }
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;
    fn index(&self, Pos { x, y }: Pos) -> &Self::Output {
        &self.data[y as usize][x as usize]
    }
}
impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, Pos { x, y }: Pos) -> &mut Self::Output {
        &mut self.data[y as usize][x as usize]
    }
}
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y][x]
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y][x]
    }
}
impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        &self.data[y as usize][x as usize]
    }
}
impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        &mut self.data[y as usize][x as usize]
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Default::default(),
        }
    }
}
