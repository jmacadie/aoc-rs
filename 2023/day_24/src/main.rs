#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use crate::point::Point;
use hail_path::HailPath;

pub fn main() {
    let data = include_str!("input.txt");
    println!(
        "Part 1: {}",
        part_one::<300>(data, 200_000_000_000_000, 400_000_000_000_000)
    );
    println!("Part 2: {}", part_two(data));
}

fn part_one<const N: usize>(data: &str, min: i64, max: i64) -> usize {
    // let mut segments = std::array::from_fn(|_| LineSegment::default());
    // for (s, l) in segments.iter_mut().zip(data.lines()) {
    for l in data.lines() {
        let path: HailPath = l.parse().unwrap();
        path.to_ls(min.into(), max.into());
    }
    // let mut solver = Solver::<N>::new(&segments);
    // solver.run()
    0
}

const fn part_two(_data: &str) -> usize {
    0
}

// mod segments {
//     use std::{cmp::Ordering, collections::BinaryHeap};
//
//     use crate::{line_segment::LineSegment, point::Point};
//
//     type SegID = usize;
//     type SortID = usize;
//
//     pub struct Solver<const N: usize> {
//         events: Events,
//         segments: Segments<N>,
//         intersections: usize,
//     }
//
//     impl<const N: usize> Solver<N> {
//         pub fn new(data: &[LineSegment; N]) -> Self {
//             let mut events = Events::new();
//             data.iter()
//                 .enumerate()
//                 .filter(|(_, segment)| segment.from != Point::default())
//                 .for_each(|(seg_id, segment)| {
//                     events.push(Event::Start(segment.from, seg_id));
//                     events.push(Event::End(segment.to, seg_id));
//                 });
//             let all = std::array::from_fn(|i| (data[i].clone(), None));
//             let segments = Segments::<N> {
//                 all,
//                 active: [0; N],
//                 active_count: 0,
//             };
//             Self {
//                 events,
//                 segments,
//                 intersections: 0,
//             }
//         }
//
//         pub fn run(&mut self) -> usize {
//             let mut last = Point::default();
//             while let Some(e) = self.events.pop() {
//                 match e {
//                     Event::Start(p, seg_id) => {
//                         // println!();
//                         println!("starting {seg_id} @ {p}");
//                         let sorted_idx = self.segments.add(seg_id, p);
//                         let next = self.segments.get_next_id(sorted_idx);
//                         if next.is_some() {
//                             let next_id = next.unwrap();
//                             let next_ls = self.segments.get(next_id);
//                             let ls = self.segments.get(seg_id);
//                             if let Intersection::Point(loc) = ls.intersect(next_ls) {
//                                 assert!(loc.x > p.x);
//                                 self.events.push(Event::Intersection(loc, seg_id, next_id));
//                             }
//                         }
//                         let prev = self.segments.get_prev_id(sorted_idx);
//                         if prev.is_some() {
//                             let prev_id = prev.unwrap();
//                             let prev_ls = self.segments.get(prev_id);
//                             let ls = self.segments.get(seg_id);
//                             if let Intersection::Point(loc) = ls.intersect(prev_ls) {
//                                 assert!(loc.x > p.x);
//                                 self.events.push(Event::Intersection(loc, prev_id, seg_id));
//                             }
//                         }
//                         self.segments.check_order(p);
//                         assert!(p.x >= last.x);
//                         assert!((p.x - last.x).abs() > 5.0 || (p.y - last.y).abs() > 5.0);
//                         last = p;
//                     }
//                     Event::End(p, seg_id) => {
//                         // println!();
//                         println!("ending {seg_id} @ {p}");
//                         let sorted_idx = self.segments.get_pos(seg_id);
//                         assert_eq!(self.segments.active[sorted_idx], seg_id);
//                         let next = self.segments.get_next_id(sorted_idx);
//                         let prev = self.segments.get_prev_id(sorted_idx);
//                         if next.is_some() && prev.is_some() {
//                             let next_id = next.unwrap();
//                             let prev_id = prev.unwrap();
//                             let next_ls = self.segments.get(next_id);
//                             let prev_ls = self.segments.get(prev_id);
//                             if let Intersection::Point(loc) = next_ls.intersect(prev_ls) {
//                                 if loc.x > p.x {
//                                     self.events.push(Event::Intersection(loc, prev_id, next_id));
//                                 }
//                             }
//                         }
//                         self.segments.del(sorted_idx);
//                         self.segments.check_order(p);
//                         assert!(p.x >= last.x);
//                         assert!((p.x - last.x).abs() > 5.0 || (p.y - last.y).abs() > 5.0);
//                         last = p;
//                     }
//                     Event::Intersection(p, s1, s2) => {
//                         // println!();
//                         println!("intersection between {s1} and {s2} @ {p}");
//                         self.intersections += 1;
//                         let mut lower = self.segments.get_pos(s1);
//                         let mut upper = self.segments.get_pos(s2);
//                         let lower_seg;
//                         let upper_seg = if lower > upper {
//                             std::mem::swap(&mut lower, &mut upper);
//                             lower_seg = s2;
//                             s1
//                         } else {
//                             lower_seg = s1;
//                             s2
//                         };
//                         assert_eq!(lower + 1, upper);
//                         self.segments.swap(lower, upper);
//                         let next = self.segments.get_next_id(upper);
//                         // println!("{next:?}");
//                         if next.is_some() {
//                             let next_id = next.unwrap();
//                             let next_ls = self.segments.get(next_id);
//                             let ls = self.segments.get(lower_seg);
//                             if let Intersection::Point(loc) = ls.intersect(next_ls) {
//                                 if loc.x > p.x {
//                                     self.events
//                                         .push(Event::Intersection(loc, lower_seg, next_id));
//                                 }
//                             }
//                         }
//                         let prev = self.segments.get_prev_id(lower);
//                         // println!("{prev:?}");
//                         if prev.is_some() {
//                             let prev_id = prev.unwrap();
//                             let prev_ls = self.segments.get(prev_id);
//                             let ls = self.segments.get(upper_seg);
//                             if let Intersection::Point(loc) = ls.intersect(prev_ls) {
//                                 if loc.x > p.x {
//                                     self.events
//                                         .push(Event::Intersection(loc, prev_id, upper_seg));
//                                 }
//                             }
//                         }
//                         self.segments.check_order(p);
//                         assert!(p.x >= last.x);
//                         assert!((p.x - last.x).abs() > 5.0 || (p.y - last.y).abs() > 5.0);
//                         last = p;
//                     }
//                 }
//             }
//             self.intersections
//         }
//     }
//
//     struct Events {
//         priority_queue: BinaryHeap<Event>,
//         last: Option<Event>,
//     }
//
//     impl Events {
//         fn new() -> Self {
//             Self {
//                 priority_queue: BinaryHeap::with_capacity(100),
//                 last: None,
//             }
//         }
//
//         fn pop(&mut self) -> Option<Event> {
//             let mut value = self.priority_queue.pop();
//             while value.is_some() && value == self.last {
//                 value = self.priority_queue.pop();
//             }
//             self.last = value;
//             value
//         }
//
//         fn push(&mut self, item: Event) {
//             self.priority_queue.push(item);
//         }
//     }
//
//     struct Segments<const N: usize> {
//         all: [(LineSegment, Option<SortID>); N],
//         active: [SegID; N],
//         active_count: usize,
//     }
//
//     impl<const N: usize> Segments<N> {
//         fn check_order(&self, p: Point) {
//             let mut last = self.active[0];
//             self.active[..self.active_count].iter().for_each(|seg| {
//                 let curr = self.all[*seg].0.point_at_x(p.x).unwrap().y;
//                 // println!("{seg}\t{curr}");
//                 assert_eq!(curr, last);
//                 last = curr;
//             });
//         }
//
//         const fn get(&self, segment: SegID) -> &LineSegment {
//             &self.all[segment].0
//         }
//
//         const fn get_pos(&self, segment: SegID) -> usize {
//             self.all[segment]
//                 .1
//                 .expect("The segment has a position in the active array")
//         }
//
//         const fn get_prev_id(&self, sorted_idx: SortID) -> Option<SegID> {
//             if sorted_idx == 0 {
//                 return None;
//             }
//             Some(self.active[sorted_idx - 1])
//         }
//
//         const fn get_prev(&self, sorted_idx: SortID) -> Option<&LineSegment> {
//             let id = self.get_prev_id(sorted_idx);
//             if id.is_none() {
//                 return None;
//             }
//             Some(self.get(id.unwrap()))
//         }
//
//         const fn get_next_id(&self, sorted_idx: SortID) -> Option<SegID> {
//             if (sorted_idx + 1) > self.active_count {
//                 return None;
//             }
//             Some(self.active[sorted_idx + 1])
//         }
//
//         const fn get_next(&self, sorted_idx: SortID) -> Option<&LineSegment> {
//             let id = self.get_next_id(sorted_idx);
//             if id.is_none() {
//                 return None;
//             }
//             Some(self.get(id.unwrap()))
//         }
//
//         fn find(&self, p: Point) -> SortID {
//             fn find_inner(
//                 p: Point,
//                 search: &[SegID],
//                 segments: &[(LineSegment, Option<SortID>)],
//                 start_index: SortID,
//             ) -> SortID {
//                 let mid_idx = search.len() / 2;
//                 let seg_id = search[mid_idx];
//                 let mid_value = segments[seg_id].0.point_at_x(p.x).unwrap().y;
//
//                 if search.len() == 1 {
//                     if p.y > mid_value {
//                         return start_index + 1;
//                     }
//                     return start_index;
//                 }
//                 if search.len() == 2 && p.y > mid_value {
//                     return start_index + 2;
//                 }
//
//                 if p.y > mid_value {
//                     let idx = mid_idx + 1;
//                     find_inner(p, &search[idx..], segments, start_index + idx)
//                 } else {
//                     let idx = mid_idx;
//                     find_inner(p, &search[..idx], segments, start_index)
//                 }
//             }
//
//             if self.active_count == 0 {
//                 return 0;
//             }
//
//             find_inner(p, &self.active[..self.active_count], &self.all, 0)
//         }
//
//         fn add(&mut self, segment: SegID, p: Point) -> SortID {
//             let position = self.find(p);
//             (position..self.active_count).rev().for_each(|i| {
//                 self.increment(i);
//                 self.active[i + 1] = self.active[i];
//             });
//             self.all[segment].1 = Some(position);
//             self.active[position] = segment;
//             self.active_count += 1;
//             // (0..self.active_count).for_each(|i| {
//             //     println!("{i} - {}, {:?}", self.active[i], self.all[self.active[i]]);
//             // });
//             // (0..self.active_count).for_each(|i| {
//             //     assert_eq!(Some(i), self.all[self.active[i]].1);
//             // });
//             position
//         }
//
//         fn del(&mut self, position: SortID) {
//             assert!(self.all[self.active[position]].1.is_some());
//             self.all[self.active[position]].1 = None;
//             (position..self.active_count).for_each(|i| {
//                 if i > position {
//                     self.decrement(i);
//                 }
//                 self.active[i] = self.active[i + 1];
//             });
//             self.active_count -= 1;
//             // (0..self.active_count).for_each(|i| {
//             //     println!("{i} - {}, {:?}", self.active[i], self.all[self.active[i]]);
//             // });
//             // (0..self.active_count).for_each(|i| {
//             //     assert_eq!(Some(i), self.all[self.active[i]].1);
//             // });
//         }
//
//         fn swap(&mut self, lower: SortID, upper: SortID) {
//             assert_eq!(lower + 1, upper); // Should be true?
//             self.increment(lower);
//             self.decrement(upper);
//             self.active.swap(lower, upper);
//             // (0..self.active_count).for_each(|i| {
//             //     println!("{i} - {}, {:?}", self.active[i], self.all[self.active[i]]);
//             // });
//             // (0..self.active_count).for_each(|i| {
//             //     assert_eq!(Some(i), self.all[self.active[i]].1);
//             // });
//         }
//
//         fn increment(&mut self, location: SortID) {
//             if let Some(x) = self.all[self.active[location]].1.as_mut() {
//                 *x += 1;
//             } else {
//                 unreachable!();
//             };
//         }
//
//         fn decrement(&mut self, location: SortID) {
//             if let Some(x) = self.all[self.active[location]].1.as_mut() {
//                 *x -= 1;
//             } else {
//                 unreachable!();
//             };
//         }
//     }
//
//     #[derive(Debug, Clone, Copy)]
//     enum Event {
//         Start(Point, SegID),
//         End(Point, SegID),
//         Intersection(Point, SegID, SegID),
//     }
//
//     impl Event {
//         const fn loc(&self) -> Point {
//             match self {
//                 Self::Start(p, _) | Self::End(p, _) | Self::Intersection(p, _, _) => *p,
//             }
//         }
//     }
//
//     impl Ord for Event {
//         fn cmp(&self, other: &Self) -> Ordering {
//             let a = self.loc();
//             let b = other.loc();
//             match b.x.partial_cmp(&a.x) {
//                 Some(Ordering::Less) => Ordering::Less,
//                 Some(Ordering::Greater) => Ordering::Greater,
//                 Some(Ordering::Equal) => {
//                     b.y.partial_cmp(&a.y)
//                         .expect("co-ordinates with comparable values")
//                 }
//                 None => unreachable!(),
//             }
//         }
//     }
//
//     impl PartialOrd for Event {
//         fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//             Some(self.cmp(other))
//         }
//     }
//
//     impl Eq for Event {}
//
//     impl PartialEq for Event {
//         fn eq(&self, other: &Self) -> bool {
//             self.loc() == other.loc()
//         }
//     }
// }

mod hail_path {
    use std::str::FromStr;

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

            let test = |axis: Axis, bound: Rational| {
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
                    println!("({a}, {b}) -> ({da}, {db}) = {computed}");
                    println!("{computed} is greater than {min}: {}", computed > min);
                    println!("{computed} is less than {max}: {}", computed < max);
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

            let left_side_intercept = test(Axis::X, min);
            let right_side_intercept = test(Axis::X, max);
            let top_side_intercept = test(Axis::Y, min);
            let bottom_side_intercept = test(Axis::Y, max);

            println!("{left_side_intercept:?}");
            println!("{right_side_intercept:?}");
            println!("{top_side_intercept:?}");
            println!("{bottom_side_intercept:?}");

            left_side_intercept
                .map(|p| LineSegment::new(p, self.velocity.to_2d(), self.position.to_2d()))
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
}

mod line_segment {
    use crate::{
        rational::{Rational, Sign},
        Point,
    };

    #[derive(Clone, Debug)]
    pub struct LineSegment {
        from: Point,
        direction: Point,
        to: Point,
        c1: Rational,
        c2: Rational,
    }

    impl LineSegment {
        pub fn new(from: Point, direction: Point, to: Point) -> Self {
            let xdy = from.x * direction.y;
            let ydx = from.y * direction.x;
            Self {
                from,
                direction,
                to,
                c1: xdy + ydx,
                c2: xdy - ydx,
            }
        }

        pub fn intersect(&self, other: &Self) -> Option<Point> {
            let det = self.dx() * other.dy() - self.dy() * other.dx();
            let int_x = self.dx() * other.c1 - other.dx() * self.c1;
            let int_x = int_x / det;
            if self.x_in_range(int_x) && other.x_in_range(int_x) {
                let int_y = self.dy() * other.c2 - other.dy() * self.c2;
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
}

mod point {
    use crate::rational::Rational;

    use std::{
        fmt::Display,
        ops::{Add, Sub},
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        pub x: Rational,
        pub y: Rational,
    }

    impl Point {
        pub const fn new(x: i64, y: i64) -> Self {
            Self {
                x: Rational::new(x, 1),
                y: Rational::new(y, 1),
            }
        }

        pub fn same_direction(self, other: Self) -> bool {
            self.x.sign() == other.x.sign() && self.y.sign() == other.y.sign()
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
    impl From<(i64, i64)> for Point {
        fn from(value: (i64, i64)) -> Self {
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

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({:.3}, {:.3})", self.x, self.y)?;
            Ok(())
        }
    }
}

mod rational {
    use std::{
        fmt::Display,
        num::ParseIntError,
        ops::{Add, Div, Mul, Sub},
        str::FromStr,
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Rational {
        numerator: i64,
        denominator: i64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Sign {
        Pos,
        Zero,
        Neg,
    }

    impl Rational {
        pub const fn new(numerator: i64, denominator: i64) -> Self {
            Self {
                numerator,
                denominator,
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
                Self {
                    numerator: self.numerator + rhs.numerator,
                    denominator: self.denominator,
                }
            } else {
                Self {
                    numerator: self.numerator * rhs.denominator + self.denominator * rhs.numerator,
                    denominator: self.denominator * rhs.denominator,
                }
            }
        }
    }

    impl Sub for Rational {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            if self.denominator == rhs.denominator {
                Self {
                    numerator: self.numerator - rhs.numerator,
                    denominator: self.denominator,
                }
            } else {
                Self {
                    numerator: self.numerator * rhs.denominator - self.denominator * rhs.numerator,
                    denominator: self.denominator * rhs.denominator,
                }
            }
        }
    }

    impl Mul for Rational {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                numerator: self.numerator * rhs.numerator,
                denominator: self.denominator * rhs.denominator,
            }
        }
    }

    impl Div for Rational {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            if rhs.numerator.is_negative() {
                Self {
                    numerator: -self.numerator * rhs.denominator,
                    denominator: -self.denominator * rhs.numerator,
                }
            } else {
                Self {
                    numerator: self.numerator * rhs.denominator,
                    denominator: self.denominator * rhs.numerator,
                }
            }
        }
    }

    impl FromStr for Rational {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let numerator = s.parse()?;
            Ok(Self {
                numerator,
                denominator: 1,
            })
        }
    }
    impl From<i64> for Rational {
        fn from(value: i64) -> Self {
            Self {
                numerator: value,
                denominator: 1,
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
            (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
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
                    write!(f, "{}.{}/{}", int, res, self.denominator)?;
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
