#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::point::Point;
use hail_path::HailPath;
use segments::Solver;

pub fn main() {
    let data = include_str!("input.txt");
    println!(
        "Part 1: {}",
        part_one::<300>(data, 200_000_000_000_000, 400_000_000_000_000)
    );
    println!("Part 2: {}", part_two(data));
}

fn part_one<const N: usize>(data: &str, min: i128, max: i128) -> usize {
    let mut segments = std::array::from_fn(|_| None);
    for (s, l) in segments.iter_mut().zip(data.lines()) {
        let path: HailPath = l.parse().unwrap();
        *s = path.to_ls(min.into(), max.into());
    }
    let mut solver = Solver::<N>::new(&segments);
    solver.run()
}

const fn part_two(_data: &str) -> usize {
    0
}

mod segments {
    use std::{cmp::Ordering, collections::BinaryHeap};

    use crate::{line_segment::LineSegment, point::Point};

    type SegID = usize;
    type SortID = usize;

    pub struct Solver<const N: usize> {
        events: Events,
        segments: Segments<N>,
        intersections: usize,
    }

    impl<const N: usize> Solver<N> {
        pub fn new(data: &[Option<LineSegment>; N]) -> Self {
            let mut events = Events::new();
            data.iter()
                .enumerate()
                .filter(|(_, segment)| segment.is_some())
                .map(|(seg_id, segment)| (seg_id, segment.as_ref().unwrap()))
                .for_each(|(seg_id, segment)| {
                    events.push(Event::Start(segment.from, seg_id));
                    events.push(Event::End(segment.to, seg_id));
                });
            let all = std::array::from_fn(|i| (data[i].clone(), None));
            let segments = Segments::<N> {
                all,
                active: [0; N],
                active_count: 0,
            };
            Self {
                events,
                segments,
                intersections: 0,
            }
        }

        pub fn run(&mut self) -> usize {
            let mut last = Point::default();
            while let Some(e) = self.events.pop() {
                match e {
                    Event::Start(p, seg_id) => {
                        // println!("starting {seg_id} @ {p}");
                        let sorted_idx = self.segments.add(seg_id, p);
                        if let Some(next) = self.segments.get_next_id(sorted_idx) {
                            let next_ls = self.segments.get(next);
                            let ls = self.segments.get(seg_id);
                            if let Some(loc) = ls.intersect(next_ls) {
                                assert!(loc.x > p.x);
                                self.events.push(Event::Intersection(loc, seg_id, next));
                            }
                        }
                        let prev = self.segments.get_prev_id(sorted_idx);
                        if prev.is_some() {
                            let prev_id = prev.unwrap();
                            let prev_ls = self.segments.get(prev_id);
                            let ls = self.segments.get(seg_id);
                            if let Some(loc) = ls.intersect(prev_ls) {
                                assert!(loc.x > p.x);
                                self.events.push(Event::Intersection(loc, prev_id, seg_id));
                            }
                        }
                        last = p;
                    }
                    Event::End(p, seg_id) => {
                        // println!("ending {seg_id} @ {p}");
                        let sorted_idx = self.segments.get_pos(seg_id);
                        assert_eq!(self.segments.active[sorted_idx], seg_id);
                        let next = self.segments.get_next_id(sorted_idx);
                        let prev = self.segments.get_prev_id(sorted_idx);
                        if next.is_some() && prev.is_some() {
                            let next_id = next.unwrap();
                            let prev_id = prev.unwrap();
                            let next_ls = self.segments.get(next_id);
                            let prev_ls = self.segments.get(prev_id);
                            if let Some(loc) = next_ls.intersect(prev_ls) {
                                if loc.x > p.x {
                                    self.events.push(Event::Intersection(loc, prev_id, next_id));
                                }
                            }
                        }
                        self.segments.del(sorted_idx);
                        last = p;
                    }
                    Event::Intersection(p, s1, s2) => {
                        // println!("intersection between {s1} and {s2} @ {p}");
                        self.intersections += 1;
                        let mut lower = self.segments.get_pos(s1);
                        let mut upper = self.segments.get_pos(s2);
                        let lower_seg;
                        let upper_seg = if lower > upper {
                            std::mem::swap(&mut lower, &mut upper);
                            lower_seg = s2;
                            s1
                        } else {
                            lower_seg = s1;
                            s2
                        };
                        assert_eq!(lower + 1, upper);
                        self.segments.swap(lower, upper);
                        let next = self.segments.get_next_id(upper);
                        if next.is_some() {
                            let next_id = next.unwrap();
                            let next_ls = self.segments.get(next_id);
                            let ls = self.segments.get(lower_seg);
                            if let Some(loc) = ls.intersect(next_ls) {
                                if loc.x > p.x {
                                    self.events
                                        .push(Event::Intersection(loc, lower_seg, next_id));
                                }
                            }
                        }
                        let prev = self.segments.get_prev_id(lower);
                        if prev.is_some() {
                            let prev_id = prev.unwrap();
                            let prev_ls = self.segments.get(prev_id);
                            let ls = self.segments.get(upper_seg);
                            if let Some(loc) = ls.intersect(prev_ls) {
                                if loc.x > p.x {
                                    self.events
                                        .push(Event::Intersection(loc, prev_id, upper_seg));
                                }
                            }
                        }
                        assert!(p.x >= last.x);
                        last = p;
                    }
                }
            }
            self.intersections
        }
    }

    struct Events {
        priority_queue: BinaryHeap<Event>,
        last: Option<Event>,
    }

    impl Events {
        fn new() -> Self {
            Self {
                priority_queue: BinaryHeap::with_capacity(100),
                last: None,
            }
        }

        fn pop(&mut self) -> Option<Event> {
            let mut value = self.priority_queue.pop();
            while value.is_some() && value == self.last {
                value = self.priority_queue.pop();
            }
            self.last = value;
            value
        }

        fn push(&mut self, item: Event) {
            self.priority_queue.push(item);
        }
    }

    struct Segments<const N: usize> {
        all: [(Option<LineSegment>, Option<SortID>); N],
        active: [SegID; N],
        active_count: usize,
    }

    impl<const N: usize> Segments<N> {
        const fn get(&self, segment: SegID) -> &LineSegment {
            self.all[segment].0.as_ref().unwrap()
        }

        const fn get_pos(&self, segment: SegID) -> usize {
            self.all[segment]
                .1
                .expect("The segment has a position in the active array")
        }

        const fn get_prev_id(&self, sorted_idx: SortID) -> Option<SegID> {
            if sorted_idx == 0 {
                return None;
            }
            Some(self.active[sorted_idx - 1])
        }

        const fn get_next_id(&self, sorted_idx: SortID) -> Option<SegID> {
            if (sorted_idx + 1) >= self.active_count {
                return None;
            }
            Some(self.active[sorted_idx + 1])
        }

        fn find(&self, p: Point) -> SortID {
            fn find_inner(
                p: Point,
                search: &[SegID],
                segments: &[(Option<LineSegment>, Option<SortID>)],
                start_index: SortID,
            ) -> SortID {
                let mid_idx = search.len() / 2;
                let seg_id = search[mid_idx];
                let mid_value = segments[seg_id]
                    .0
                    .as_ref()
                    .unwrap()
                    .point_at_x(p.x)
                    .unwrap()
                    .y;

                if search.len() == 1 {
                    if p.y > mid_value {
                        return start_index + 1;
                    }
                    return start_index;
                }
                if search.len() == 2 && p.y > mid_value {
                    return start_index + 2;
                }

                if p.y > mid_value {
                    let idx = mid_idx + 1;
                    find_inner(p, &search[idx..], segments, start_index + idx)
                } else {
                    let idx = mid_idx;
                    find_inner(p, &search[..idx], segments, start_index)
                }
            }

            if self.active_count == 0 {
                return 0;
            }

            find_inner(p, &self.active[..self.active_count], &self.all, 0)
        }

        fn add(&mut self, segment: SegID, p: Point) -> SortID {
            let position = self.find(p);
            (position..self.active_count).rev().for_each(|i| {
                self.increment(i);
                self.active[i + 1] = self.active[i];
            });
            self.all[segment].1 = Some(position);
            self.active[position] = segment;
            self.active_count += 1;
            position
        }

        fn del(&mut self, position: SortID) {
            assert!(self.all[self.active[position]].1.is_some());
            self.all[self.active[position]].1 = None;
            (position..self.active_count).for_each(|i| {
                if i > position {
                    self.decrement(i);
                }
                self.active[i] = self.active[i + 1];
            });
            self.active_count -= 1;
        }

        fn swap(&mut self, lower: SortID, upper: SortID) {
            assert_eq!(lower + 1, upper); // Should be true?
            self.increment(lower);
            self.decrement(upper);
            self.active.swap(lower, upper);
        }

        fn increment(&mut self, location: SortID) {
            if let Some(x) = self.all[self.active[location]].1.as_mut() {
                *x += 1;
            } else {
                unreachable!();
            };
        }

        fn decrement(&mut self, location: SortID) {
            if let Some(x) = self.all[self.active[location]].1.as_mut() {
                *x -= 1;
            } else {
                unreachable!();
            };
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Event {
        Start(Point, SegID),
        End(Point, SegID),
        Intersection(Point, SegID, SegID),
    }

    impl Event {
        const fn loc(&self) -> Point {
            match self {
                Self::Start(p, _) | Self::End(p, _) | Self::Intersection(p, _, _) => *p,
            }
        }
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> Ordering {
            let a = self.loc();
            let b = other.loc();
            match b.x.partial_cmp(&a.x) {
                Some(Ordering::Less) => Ordering::Less,
                Some(Ordering::Greater) => Ordering::Greater,
                Some(Ordering::Equal) => {
                    b.y.partial_cmp(&a.y)
                        .expect("co-ordinates with comparable values")
                }
                None => unreachable!(),
            }
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for Event {}

    impl PartialEq for Event {
        fn eq(&self, other: &Self) -> bool {
            self.loc() == other.loc()
        }
    }
}

mod hail_path {
    use std::{fmt::Display, str::FromStr};

    use crate::{
        line_segment::LineSegment,
        point::Point,
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

    #[derive(Debug)]
    pub struct Point3D {
        pub x: Rational,
        pub y: Rational,
        pub z: Rational,
    }

    impl Point3D {
        fn to_2d(&self) -> Point {
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
}

mod line_segment {
    use std::fmt::Display;

    use crate::{
        rational::{Rational, Sign},
        Point,
    };

    #[derive(Clone, Debug)]
    pub struct LineSegment {
        pub from: Point,
        pub direction: Point,
        pub to: Point,
        c: Rational,
    }

    impl LineSegment {
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

        pub fn x_in_range(&self, x: Rational) -> bool {
            if self.direction.x.sign() == Sign::Pos {
                x >= self.from.x && x <= self.to.x
            } else {
                x >= self.to.x && x <= self.from.x
            }
        }

        pub fn point_at_x(&self, x: Rational) -> Option<Point> {
            if !self.x_in_range(x) {
                return None;
            }
            let s = (x - self.from.x) / self.dx();
            let y = self.from.y + s * self.dy();
            Some((x, y).into())
        }

        pub const fn dx(&self) -> Rational {
            self.direction.x
        }

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
}

mod point {
    use crate::rational::Rational;

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
}

mod rational {
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
        pub const fn new(numerator: i128, denominator: i128) -> Self {
            Self {
                numerator,
                denominator,
                int: numerator / denominator,
                frac: numerator % denominator,
            }
        }

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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_one::<5>(data, 7, 27));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
