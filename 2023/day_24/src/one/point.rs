use super::rational::Rational;

use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: Rational,
    pub y: Rational,
}

impl Point {
    #[must_use]
    pub const fn new(x: i128, y: i128) -> Self {
        Self {
            x: Rational::new(x, 1),
            y: Rational::new(y, 1),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
        }
    }
}

impl From<(i128, i128)> for Point {
    fn from(value: (i128, i128)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Rational, Rational)> for Point {
    fn from(value: (Rational, Rational)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}
