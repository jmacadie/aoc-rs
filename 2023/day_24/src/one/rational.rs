use std::{
    fmt::Display,
    num::ParseIntError,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub struct Rational {
    numerator: i128,
    denominator: i128,
    int: i128,
    frac: i128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Pos,
    Zero,
    Neg,
}

impl Rational {
    #[must_use]
    pub const fn new(numerator: i128, denominator: i128) -> Self {
        Self {
            numerator,
            denominator,
            int: numerator / denominator,
            frac: numerator % denominator,
        }
    }

    #[must_use]
    pub const fn sign(self) -> Sign {
        if self.numerator == 0 {
            return Sign::Zero;
        }
        if self.numerator.is_positive() == self.denominator.is_positive() {
            return Sign::Pos;
        }
        Sign::Neg
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.denominator == rhs.denominator {
            Self::new(self.numerator + rhs.numerator, self.denominator)
        } else {
            Self::new(
                self.numerator * rhs.denominator + self.denominator * rhs.numerator,
                self.denominator * rhs.denominator,
            )
        }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.denominator == rhs.denominator {
            Self::new(self.numerator - rhs.numerator, self.denominator)
        } else {
            Self::new(
                self.numerator * rhs.denominator - self.denominator * rhs.numerator,
                self.denominator * rhs.denominator,
            )
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.numerator == 0 || rhs.numerator == 0 {
            return Self {
                numerator: 0,
                denominator: 1,
                int: 0,
                frac: 0,
            };
        }
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.numerator.is_negative() {
            Self::new(
                -self.numerator * rhs.denominator,
                -self.denominator * rhs.numerator,
            )
        } else {
            Self::new(
                self.numerator * rhs.denominator,
                self.denominator * rhs.numerator,
            )
        }
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.numerator, self.denominator)
    }
}

impl FromStr for Rational {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numerator = s.parse()?;
        Ok(Self {
            numerator,
            denominator: 1,
            int: numerator,
            frac: 0,
        })
    }
}

impl From<i128> for Rational {
    fn from(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
            int: value,
            frac: 0,
        }
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.sign() == Sign::Pos && other.sign() != Sign::Pos {
            return std::cmp::Ordering::Greater;
        }
        if self.sign() == Sign::Neg && other.sign() != Sign::Neg {
            return std::cmp::Ordering::Less;
        }
        if self.denominator == other.denominator {
            return self.numerator.cmp(&other.numerator);
        }
        match self.int.cmp(&other.int) {
            std::cmp::Ordering::Equal => {
                (self.frac * other.denominator).cmp(&(self.denominator * other.frac))
            }
            x => x,
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }
}

impl Eq for Rational {}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)?;
        } else {
            let int = self.numerator / self.denominator;
            let res = self.numerator % self.denominator;
            if res == 0 {
                write!(f, "{int}")?;
            } else {
                write!(f, "{}_{}/{}", int, res, self.denominator)?;
            }
        }
        Ok(())
    }
}
