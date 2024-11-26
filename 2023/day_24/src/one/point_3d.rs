use std::{fmt::Display, str::FromStr};

use super::{point::Point, rational::Rational};

#[derive(Debug)]
pub struct Point3D {
    pub x: Rational,
    pub y: Rational,
    pub z: Rational,
}

impl Point3D {
    #[must_use]
    pub fn to_2d(&self) -> Point {
        (self.x, self.y).into()
    }
}

impl FromStr for Point3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", ");
        let (Some(x), Some(y), Some(z), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(format!(
                "Cannot split {s} into three parts for the co-ordinates"
            ));
        };
        let x = x
            .trim()
            .parse()
            .map_err(|_| format!("x co-ordinate {x} cannot be converted into a number"))?;
        let y = y
            .trim()
            .parse()
            .map_err(|_| format!("y co-ordinate {y} cannot be converted into a number"))?;
        let z = z
            .trim()
            .parse()
            .map_err(|_| format!("z co-ordinate {z} cannot be converted into a number"))?;
        Ok(Self { x, y, z })
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)?;
        Ok(())
    }
}
