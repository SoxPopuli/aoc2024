use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}
impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let delta_x = self.x.abs_diff(other.x);
        let delta_y = self.y.abs_diff(other.y);

        let squared = (delta_x * delta_x) + (delta_y + delta_y);

        (squared as f64).sqrt()
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        let delta_x = self.x.abs_diff(other.x);
        let delta_y = self.y.abs_diff(other.y);

        delta_x + delta_y
    }

    pub fn normalize(&self) -> (f64, f64) {
        let len = (((self.x * self.x) + (self.y * self.y)) as f64).sqrt();

        let x = self.x as f64 / len;
        let y = self.y as f64 / len;

        (x, y)
    }

    pub fn normalize_int(&self) -> Self {
        let (x, y) = self.normalize();
        Self {
            x: x.round() as isize,
            y: y.round() as isize,
        }
    }
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
impl Sub<isize> for Pos {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl Mul<isize> for Pos {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
impl PartialEq<&Self> for Pos {
    fn eq(&self, other: &&Self) -> bool {
        self == *other
    }
}
