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
