use std::fmt::Display;

use super::{
    point::Point,
    rational::{Rational, Sign},
};

#[derive(Clone, Debug)]
pub struct LineSegment {
    pub from: Point,
    pub direction: Point,
    pub to: Point,
    c: Rational,
}

impl LineSegment {
    #[must_use]
    pub fn new(from: Point, direction: Point, to: Point) -> Self {
        let a = from;
        let b = to;
        let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };
        let d = if direction.x.sign() == Sign::Pos {
            direction
        } else {
            -direction
        };
        Self {
            from: a,
            direction: d,
            to: b,
            c: a.x * d.y - a.y * d.x,
        }
    }

    #[must_use]
    pub fn intersect(&self, other: &Self) -> Option<Point> {
        let det = self.dx() * other.dy() - self.dy() * other.dx();
        if det == 0.into() {
            return None;
        }
        let int_x = self.dx() * other.c - other.dx() * self.c;
        let int_x = int_x / det;
        if self.x_in_range(int_x) && other.x_in_range(int_x) {
            let int_y = self.dy() * other.c - other.dy() * self.c;
            let int_y = int_y / det;
            return Some((int_x, int_y).into());
        }
        None
    }

    #[must_use]
    pub fn x_in_range(&self, x: Rational) -> bool {
        if self.direction.x.sign() == Sign::Pos {
            x >= self.from.x && x <= self.to.x
        } else {
            x >= self.to.x && x <= self.from.x
        }
    }

    #[must_use]
    pub fn point_at_x(&self, x: Rational) -> Option<Point> {
        if !self.x_in_range(x) {
            return None;
        }
        let s = (x - self.from.x) / self.dx();
        let y = self.from.y + s * self.dy();
        Some((x, y).into())
    }

    #[must_use]
    pub const fn dx(&self) -> Rational {
        self.direction.x
    }

    #[must_use]
    pub const fn dy(&self) -> Rational {
        self.direction.y
    }
}

impl Display for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}, 󰇂{}", self.from, self.to, self.direction)?;
        Ok(())
    }
}
