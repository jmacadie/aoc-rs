use super::Bound;
use crate::utils::gcd::gcd;

use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Div, Mul, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Linear {
    constant: i32,
    coefficient: i32,
    denominator: u32,
}

impl Add for Linear {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_denom = i32::try_from(self.denominator).unwrap();
        let rhs_denom = i32::try_from(rhs.denominator).unwrap();
        let constant = self.constant * rhs_denom + rhs.constant * self_denom;
        let coefficient = self.coefficient * rhs_denom + rhs.coefficient * self_denom;
        let denominator = self.denominator * rhs.denominator;
        Self {
            constant,
            coefficient,
            denominator,
        }
    }
}

impl Sub for Linear {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_denom = i32::try_from(self.denominator).unwrap();
        let rhs_denom = i32::try_from(rhs.denominator).unwrap();
        let constant = self.constant * rhs_denom - rhs.constant * self_denom;
        let coefficient = self.coefficient * rhs_denom - rhs.coefficient * self_denom;
        let denominator = self.denominator * rhs.denominator;
        Self {
            constant,
            coefficient,
            denominator,
        }
    }
}

impl SubAssign for Linear {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for Linear {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let constant = self.constant * rhs;
        let coefficient = self.coefficient * rhs;
        let denominator = self.denominator;
        Self {
            constant,
            coefficient,
            denominator,
        }
    }
}

impl Div<i32> for Linear {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        let (constant, coefficient, denominator) = if rhs < 0 {
            (
                -self.constant,
                -self.coefficient,
                self.denominator * rhs.unsigned_abs(),
            )
        } else {
            (
                self.constant,
                self.coefficient,
                self.denominator * rhs.unsigned_abs(),
            )
        };
        Self {
            constant,
            coefficient,
            denominator,
        }
    }
}

impl From<i32> for Linear {
    fn from(value: i32) -> Self {
        Self {
            constant: value,
            coefficient: 0,
            denominator: 1,
        }
    }
}

impl From<&i32> for Linear {
    fn from(value: &i32) -> Self {
        (*value).into()
    }
}

impl Linear {
    pub const fn variable() -> Self {
        Self {
            constant: 0,
            coefficient: 1,
            denominator: 1,
        }
    }

    pub const fn is_const(&self) -> bool {
        self.coefficient == 0
    }

    pub fn as_bound(&self) -> Option<Bound> {
        match self.coefficient.cmp(&0) {
            Ordering::Greater => {
                // c + kx >= 0
                // x >= -c/k
                Some(Bound::Lower((-self.constant).div_ceil(self.coefficient)))
            }
            Ordering::Less => {
                // c + kx >= 0
                // x <= -c/k
                Some(Bound::Upper((-self.constant).div_floor(self.coefficient)))
            }
            Ordering::Equal => None,
        }
    }

    pub const fn is_integral_at(&self, x: i32) -> bool {
        (self.constant + self.coefficient * x)
            .unsigned_abs()
            .rem_euclid(self.denominator)
            == 0
    }

    pub const fn is_upper_bound(&self) -> bool {
        self.coefficient < 0
    }

    pub fn reduced(&self) -> Self {
        let gcd = gcd(gcd(self.denominator, self.constant), self.coefficient);
        if gcd == 0 {
            return *self;
        }
        let gcd_i32 = i32::try_from(gcd).unwrap();
        Self {
            constant: self.constant / gcd_i32,
            coefficient: self.coefficient / gcd_i32,
            denominator: self.denominator / gcd,
        }
    }
}

impl Display for Linear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (constant, coefficient, denominator) =
            (self.constant, self.coefficient, self.denominator);
        let has_coefficient = coefficient != 0;
        let has_denominator = denominator != 1;
        let has_constant = !has_coefficient || constant != 0;

        write!(f, "`")?;
        match (has_constant, has_coefficient, has_denominator) {
            (true, false, false) => write!(f, "{constant}")?,
            (true, false, true) => write!(f, "{constant} / {denominator}")?,
            (false, true, false) => write!(f, "{coefficient}X")?,
            (false, true, true) => write!(f, "{coefficient}X / {denominator}")?,
            (true, true, false) => match (coefficient, coefficient.signum()) {
                (1, 1) => write!(f, "{constant} + X")?,
                (-1, -1) => write!(f, "{constant} - X")?,
                (_, 1) => write!(f, "{constant} + {coefficient}X")?,
                (_, -1) => write!(f, "{constant} - {}X", -coefficient)?,
                (_, _) => unreachable!(),
            },
            (true, true, true) => match (coefficient, coefficient.signum()) {
                (1, 1) => write!(f, "({constant} + X) / {denominator}")?,
                (-1, -1) => write!(f, "({constant} - X) / {denominator}")?,
                (_, 1) => write!(f, "({constant} + {coefficient}X) / {denominator}")?,
                (_, -1) => write!(f, "({constant} - {}X) / {denominator}", -coefficient)?,
                (_, _) => unreachable!(),
            },
            (false, false, _) => unreachable!(),
        }
        write!(f, "`")?;
        Ok(())
    }
}
