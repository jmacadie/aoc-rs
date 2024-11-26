use std::{fmt::Display, str::FromStr};

use super::{
    line_segment::LineSegment,
    point_3d::Point3D,
    rational::{Rational, Sign},
};

#[derive(Debug)]
pub struct HailPath {
    pub position: Point3D,
    pub velocity: Point3D,
}

impl FromStr for HailPath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s
            .split_once(" @ ")
            .ok_or_else(|| format!("Cannot split {s} into position & velocity"))?;
        let pos = pos.parse()?;
        let vel = vel.parse()?;
        Ok(Self {
            position: pos,
            velocity: vel,
        })
    }
}

impl HailPath {
    #[must_use]
    pub fn to_ls(&self, min: Rational, max: Rational) -> Option<LineSegment> {
        #[derive(PartialEq, Eq)]
        enum Axis {
            X,
            Y,
        }

        let box_intersect = |axis: Axis, bound: Rational| {
            let (a, b, da, db) = match axis {
                Axis::X => (
                    self.position.x,
                    self.position.y,
                    self.velocity.x,
                    self.velocity.y,
                ),
                Axis::Y => (
                    self.position.y,
                    self.position.x,
                    self.velocity.y,
                    self.velocity.x,
                ),
            };
            let s = (bound - a) / da;
            if s.sign() == Sign::Pos {
                let computed = b + db * s;
                if computed >= min && computed <= max {
                    if axis == Axis::X {
                        Some((bound, computed).into())
                    } else {
                        Some((computed, bound).into())
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        let left = box_intersect(Axis::X, min);
        let right = box_intersect(Axis::X, max);
        let top = box_intersect(Axis::Y, min);
        let bottom = box_intersect(Axis::Y, max);

        let (int_1, int_2) = [left, top, bottom, right].into_iter().fold(
            (None, None),
            |(a, b), intersection| match (intersection, a) {
                (None, _) => (a, b),
                (Some(x), None) => (Some(x), None),
                (Some(x), Some(_)) => (a, Some(x)),
            },
        );

        let int_1 = int_1?;
        let int_2 = int_2.unwrap_or_else(|| self.position.to_2d());
        Some(LineSegment::new(int_1, self.velocity.to_2d(), int_2))
    }
}

impl Display for HailPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, 󰇂{}", self.position, self.velocity)?;
        Ok(())
    }
}
