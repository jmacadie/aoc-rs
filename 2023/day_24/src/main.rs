#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use hail_path::HailPath;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    for l in data.lines() {
        let path: HailPath = l.parse().unwrap();
        let osl = path.to_osl();
        let tl = (7, 7).into();
        let br = (27, 27).into();
        println!("{:?}", osl.line);
        let seg = osl.box_intersect(tl, br);
        if seg.is_none() {
            println!("{path:?} is not inside box");
        } else {
            println!("{path:?} gives {}", seg.unwrap());
        }
    }
    0
}

const fn part_two(_data: &str) -> usize {
    0
}

mod hail_path {
    use std::str::FromStr;

    use crate::{one_sided_line::OneSidedLine, point::Point};

    #[derive(Debug)]
    pub struct HailPath {
        position: Point3D,
        velocity: Point3D,
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
        pub fn to_osl(&self) -> OneSidedLine {
            OneSidedLine::new(self.position.to_2d(), self.velocity.to_2d())
        }
    }

    #[derive(Debug)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Point3D {
        const fn to_2d(&self) -> Point {
            Point::new(self.x, self.y)
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
}

mod line_segment {
    use std::fmt::Display;

    use num_traits::Zero;

    use super::line;
    use super::line::Line;
    use super::point::Point;

    #[derive(Debug)]
    pub struct LineSegment {
        pub from: Point,
        pub to: Point,
        pub line: Line,
    }

    impl LineSegment {
        pub fn new(from: Point, to: Point) -> Self {
            // Sort on construction so `from` always has the lower x value
            // or has the lower y value, if there is no slope
            let (a, b) = match ((from.x - to.x).is_zero(), from.x - to.x, from.y - to.y) {
                (true, _, i) if i < 0.0 => (from, to),
                (true, _, i) if i > 0.0 => (to, from),
                (false, i, _) if i < 0.0 => (from, to),
                (false, i, _) if i > 0.0 => (to, from),
                _ => unreachable!(),
            };
            let line = Line::from_points(a, b);
            Self {
                from: a,
                to: b,
                line,
            }
        }

        fn point_at_x(&self, x: f64) -> Option<Point> {
            if x < self.from.x || x > self.to.x {
                return None;
            }
            self.line.point_at_x(x)
        }

        fn on_line(&self, p: Point) -> bool {
            self.line.on_line(p) && self.on_line_unchecked(p)
        }

        pub fn on_line_unchecked(&self, p: Point) -> bool {
            match self.line.slope {
                Some(_) => p.x >= self.from.x && p.x <= self.to.x,
                None => p.y >= self.from.y && p.y <= self.to.y,
            }
        }

        pub fn intersect(&self, other: &Self) -> Intersection {
            match (self.line.intersect(&other.line), self.line.slope) {
                (line::Intersection::Parallel, _) => Intersection::Parallel,
                (line::Intersection::Point(p), _) => {
                    if self.on_line_unchecked(p) && other.on_line_unchecked(p) {
                        Intersection::Point(p)
                    } else {
                        Intersection::None
                    }
                }
                (line::Intersection::SameLine, Some(_)) => {
                    if self.from.x > other.to.x || other.from.x > self.to.x {
                        Intersection::SameLineNoOverlap
                    } else {
                        let from = self.from.x.max(other.from.x);
                        let to = self.to.x.min(other.to.x);
                        let from = self
                            .line
                            .point_at_x(from)
                            .expect("from line has a valid x co-ordinate");
                        let to = self
                            .line
                            .point_at_x(to)
                            .expect("to line has a valid x co-ordinate");
                        Intersection::SameLineOverlapping(Self::new(from, to))
                    }
                }
                (line::Intersection::SameLine, None) => {
                    if self.from.y > other.to.y || other.from.y > self.to.y {
                        Intersection::SameLineNoOverlap
                    } else {
                        let from = self.from.y.max(other.from.y);
                        let to = self.to.y.min(other.to.y);
                        let from = Point::new(self.from.x, from);
                        let to = Point::new(self.from.x, to);
                        Intersection::SameLineOverlapping(Self::new(from, to))
                    }
                }
            }
        }
    }

    impl Display for LineSegment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} -> {}", self.from, self.to)?;
            Ok(())
        }
    }

    pub enum Intersection {
        Point(Point),
        None,
        Parallel,
        SameLineNoOverlap,
        SameLineOverlapping(LineSegment),
    }
}

mod one_sided_line {
    use std::fmt::Display;

    use num_traits::Zero;

    use crate::line_segment::LineSegment;

    use super::line;
    use super::line::Line;
    use super::point::Point;

    #[derive(Debug, Clone)]
    pub struct OneSidedLine {
        from: Point,
        direction: Point,
        pub line: Line,
        slope: Slope,
    }

    impl OneSidedLine {
        pub fn new(from: Point, direction: Point) -> Self {
            let to = from + direction;
            let line = Line::from_points(from, to);
            Self {
                from,
                direction,
                line,
                slope: direction.into(),
            }
        }

        fn on_line(&self, p: Point) -> bool {
            self.line.on_line(p) && self.on_line_unchecked(p)
        }

        fn on_line_unchecked(&self, p: Point) -> bool {
            self.direction.same_direction(p - self.from)
        }

        pub fn box_intersect(&self, top_left: Point, bottom_right: Point) -> Option<LineSegment> {
            let top_right = Point::new(bottom_right.x, top_left.y);
            let bottom_left = Point::new(top_left.x, bottom_right.y);
            let top = LineSegment::new(top_left, top_right);
            let bottom = LineSegment::new(bottom_left, bottom_right);
            let right = LineSegment::new(bottom_right, top_right);
            let left = LineSegment::new(bottom_left, top_left);

            let (int_1, int_2): (Option<Point>, Option<Point>) = [top, bottom, right, left]
                .iter()
                .fold((None, None), |intersections, side| {
                    match self.intersect(side) {
                        Intersection::Point(p) => {
                            match (intersections.0.is_none(), intersections.1.is_none()) {
                                (true, _) => (Some(p), None),
                                (false, true) => (intersections.0, Some(p)),
                                (false, false) => unreachable!(),
                            }
                        }
                        Intersection::None | Intersection::Parallel => intersections,
                        _ => unreachable!(),
                    }
                });
            match (int_1.is_none(), int_2.is_none()) {
                (true, _) => None,
                (false, true) => Some(LineSegment::new(self.from, int_1.unwrap())),
                (false, false) => Some(LineSegment::new(int_1.unwrap(), int_2.unwrap())),
            }
        }

        pub fn intersect(&self, other: &LineSegment) -> Intersection {
            match (self.line.intersect(&other.line), self.line.slope) {
                (line::Intersection::Parallel, _) => Intersection::Parallel,
                (line::Intersection::Point(p), _) => {
                    if self.on_line_unchecked(p) && other.on_line_unchecked(p) {
                        Intersection::Point(p)
                    } else {
                        Intersection::None
                    }
                }
                (line::Intersection::SameLine, Some(_)) => {
                    if (other.to.x < self.from.x && self.slope == Slope::Increasing)
                        || (other.from.x > self.from.x && self.slope == Slope::Decreasing)
                    {
                        Intersection::SameLineNoOverlap
                    } else if other.from.x < self.from.x && self.slope == Slope::Increasing {
                        Intersection::SameLineOverlappingLimited(LineSegment::new(
                            self.from, other.to,
                        ))
                    } else if other.to.x > self.from.x && self.slope == Slope::Decreasing {
                        Intersection::SameLineOverlappingLimited(LineSegment::new(
                            other.from, self.from,
                        ))
                    } else {
                        Intersection::SameLineOverlappingLimited(LineSegment::new(
                            other.from, other.to,
                        ))
                    }
                }
                (line::Intersection::SameLine, None) => {
                    if (other.to.y < self.from.y && self.slope == Slope::Increasing)
                        || (other.from.y > self.from.y && self.slope == Slope::Decreasing)
                    {
                        Intersection::SameLineNoOverlap
                    } else if other.from.y < self.from.y && self.slope == Slope::Increasing {
                        Intersection::SameLineOverlappingLimited(LineSegment::new(
                            self.from, other.to,
                        ))
                    } else if other.to.y < self.from.y && self.slope == Slope::Decreasing {
                        Intersection::SameLineOverlappingLimited(LineSegment::new(
                            other.from, self.from,
                        ))
                    } else {
                        Intersection::SameLineOverlappingLimited(LineSegment::new(
                            other.from, other.to,
                        ))
                    }
                }
            }
        }

        fn intersect_osl(&self, other: &Self) -> Intersection {
            match (self.line.intersect(&other.line), self.line.slope) {
                (line::Intersection::Parallel, _) => Intersection::Parallel,
                (line::Intersection::Point(p), _) => {
                    if self.on_line_unchecked(p) && other.on_line_unchecked(p) {
                        Intersection::Point(p)
                    } else {
                        Intersection::None
                    }
                }
                (line::Intersection::SameLine, Some(_)) => {
                    if self.slope == Slope::Increasing && other.slope == Slope::Increasing {
                        let line = if self.from.x > other.from.x {
                            self.clone()
                        } else {
                            other.clone()
                        };
                        Intersection::SameLineOverlappingUnlimited(line)
                    } else if self.slope == Slope::Decreasing && other.slope == Slope::Decreasing {
                        let line = if self.from.x < other.from.x {
                            self.clone()
                        } else {
                            other.clone()
                        };
                        Intersection::SameLineOverlappingUnlimited(line)
                    } else if (self.slope == Slope::Increasing
                        && other.slope == Slope::Decreasing
                        && self.from.x < other.from.x)
                        || (self.slope == Slope::Decreasing
                            && other.slope == Slope::Increasing
                            && self.from.x > other.from.x)
                    {
                        let line = LineSegment::new(self.from, other.from);
                        Intersection::SameLineOverlappingLimited(line)
                    } else {
                        Intersection::SameLineNoOverlap
                    }
                }
                (line::Intersection::SameLine, None) => {
                    if self.slope == Slope::Increasing && other.slope == Slope::Increasing {
                        let line = if self.from.y > other.from.y {
                            self.clone()
                        } else {
                            other.clone()
                        };
                        Intersection::SameLineOverlappingUnlimited(line)
                    } else if self.slope == Slope::Decreasing && other.slope == Slope::Decreasing {
                        let line = if self.from.y < other.from.y {
                            self.clone()
                        } else {
                            other.clone()
                        };
                        Intersection::SameLineOverlappingUnlimited(line)
                    } else if (self.slope == Slope::Increasing
                        && other.slope == Slope::Decreasing
                        && self.from.y < other.from.y)
                        || (self.slope == Slope::Decreasing
                            && other.slope == Slope::Increasing
                            && self.from.y > other.from.y)
                    {
                        let line = LineSegment::new(self.from, other.from);
                        Intersection::SameLineOverlappingLimited(line)
                    } else {
                        Intersection::SameLineNoOverlap
                    }
                }
            }
        }
    }

    impl Display for OneSidedLine {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} -> ..., {}", self.from, self.direction)?;
            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Slope {
        Increasing,
        Decreasing,
    }

    impl From<Point> for Slope {
        fn from(value: Point) -> Self {
            fn dir_from_value(a: f64) -> Slope {
                if a.signum() > 0.0 {
                    Slope::Increasing
                } else {
                    Slope::Decreasing
                }
            }
            if value.x.is_zero() {
                dir_from_value(value.y)
            } else {
                dir_from_value(value.x)
            }
        }
    }

    pub enum Intersection {
        Point(Point),
        None,
        Parallel,
        SameLineNoOverlap,
        SameLineOverlappingLimited(LineSegment),
        SameLineOverlappingUnlimited(OneSidedLine),
    }
}

mod line {
    use num_traits::Zero;

    use crate::point::Point;

    #[derive(Debug, Clone)]
    pub struct Line {
        dx: f64,
        dy: f64,
        c: f64,
        pub slope: Option<f64>,
        y_intercept: Option<f64>,
        intercept: f64,
    }

    impl Line {
        pub fn from_points(a: Point, b: Point) -> Self {
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let c = a.x.mul_add(b.y, -(b.x * a.y));
            let slope = if dx.is_zero() { None } else { Some(dy / dx) };
            let y_intercept = slope.map(|s| a.x.mul_add(-s, a.y));
            let intercept = y_intercept.unwrap_or_else(|| a.x + a.y * dx / dy);

            Self {
                dx,
                dy,
                c,
                slope,
                y_intercept,
                intercept,
            }
        }

        pub fn point_at_x(&self, x: f64) -> Option<Point> {
            // If the line has no slope then the line is of the
            // form `x = ...` and has no meaningful point at a given x co-ordinate
            self.slope.map(|_| self.point_at_x_unchecked(x))
        }

        fn point_at_x_unchecked(&self, x: f64) -> Point {
            let y = x.mul_add(
                self.slope.expect("this is not just an 'x =' line"),
                self.y_intercept.expect("there is a y value for {x}"),
            );
            (x, y).into()
        }

        fn determinant(&self, other: &Self) -> f64 {
            self.dx.mul_add(other.dy, -(self.dy * other.dx))
        }

        pub fn intersect(&self, other: &Self) -> Intersection {
            let det = self.determinant(other);

            // a zero determinant means the lines have the same slope
            // they are either parallel or the same line
            if det.is_zero() {
                // This tests whether the y axis intercept is different
                // If there is no y axis intercept, in the case of `x = ...` lines,
                // then the x axis intercept is used instead
                // If it is, these lines are parallel and will never intersect
                if (self.intercept - other.intercept).is_zero() {
                    return Intersection::Parallel;
                }
                return Intersection::SameLine;
            }

            let x = other.c.mul_add(self.dx, -(self.c * other.dx)) / det;
            let y = other.c.mul_add(self.dy, -(self.c * other.dy)) / det;
            Intersection::Point((x, y).into())
        }

        pub fn on_line(&self, p: Point) -> bool {
            self.point_at_x(p.x).map_or(false, |pl| pl == p)
        }
    }

    pub enum Intersection {
        Point(Point),
        Parallel,
        SameLine,
    }
}

mod point {
    use core::f64;
    use std::{
        fmt::Display,
        ops::{Add, Sub},
    };

    use num_traits::Zero;

    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub const fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn same_direction(self, other: Self) -> bool {
            #[derive(PartialEq, Eq)]
            enum Sign {
                Pos,
                Zero,
                Neg,
            }
            fn sign(a: f64) -> Sign {
                if a.is_zero() {
                    return Sign::Zero;
                }
                match a.signum() {
                    x if x > 0.0 => Sign::Pos,
                    _ => Sign::Neg,
                }
            }
            sign(self.x) == sign(other.x) && sign(self.y) == sign(other.y)
        }
    }

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            (self.x - other.x).is_zero() && (self.y - other.y).is_zero()
        }
    }

    impl From<(f64, f64)> for Point {
        fn from(value: (f64, f64)) -> Self {
            Self::new(value.0, value.1)
        }
    }

    impl From<(i32, i32)> for Point {
        fn from(value: (i32, i32)) -> Self {
            Self::new(value.0.into(), value.1.into())
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

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({:.3}, {:.3})", self.x, self.y)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
