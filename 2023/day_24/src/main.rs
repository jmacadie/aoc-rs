#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use crate::point::Point;
use crate::segments::Solver;
use hail_path::HailPath;
use line_segment::LineSegment;

const EPSILON: f64 = 0.000_001;

fn is_zero(a: f64) -> bool {
    a.abs() < EPSILON
}

fn equal(a: f64, b: f64) -> bool {
    is_zero(a - b) || (a - b).abs() < a * 0.000_000_000_001
}

pub fn main() {
    let data = include_str!("input.txt");
    let tl = (200_000_000_000_000.0, 200_000_000_000_000.0).into();
    let br = (400_000_000_000_000.0, 400_000_000_000_000.0).into();
    println!("Part 1: {}", part_one::<300>(data, tl, br));
    println!("Part 2: {}", part_two(data));
}

fn part_one<const N: usize>(data: &str, tl: Point, br: Point) -> usize {
    let mut segments = std::array::from_fn(|_| LineSegment::default());
    for (s, l) in segments.iter_mut().zip(data.lines()) {
        let path: HailPath = l.parse().unwrap();
        let osl = path.to_osl();
        let line = osl.box_intersect(tl, br);
        if line.is_some() {
            // let ls = line.unwrap();
            // println!("{path:?}");
            // println!("{osl}");
            // println!("{ls}");
            // for p in [ls.from, ls.to] {
            //     let mul = (p.x - path.position.x) / path.velocity.x;
            //     assert!(mul >= 0.0);
            //     let calc_y = path.velocity.y.mul_add(mul, path.position.y);
            //     println!("{calc_y:.3}, {:.3}", p.y);
            //     assert!((path.velocity.y.mul_add(mul, path.position.y) - p.y).abs() < 5.0);
            // }
            *s = osl
                .box_intersect(tl, br)
                .unwrap_or_else(|| panic!("{osl} crosses the target zone"));
        }
    }
    // for s in &segments {
    //     println!("{s}");
    // }
    // println!();
    let mut solver = Solver::<N>::new(&segments);
    solver.run()
}

const fn part_two(_data: &str) -> usize {
    0
}

mod segments {
    use std::{cmp::Ordering, collections::BinaryHeap};

    use crate::{
        equal,
        line_segment::{Intersection, LineSegment},
        point::Point,
    };

    type SegID = usize;
    type SortID = usize;

    pub struct Solver<const N: usize> {
        events: Events,
        segments: Segments<N>,
        intersections: usize,
    }

    impl<const N: usize> Solver<N> {
        pub fn new(data: &[LineSegment; N]) -> Self {
            let mut events = Events::new();
            data.iter()
                .enumerate()
                .filter(|(_, segment)| segment.from != Point::default())
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
                        // println!();
                        println!("starting {seg_id} @ {p}");
                        let sorted_idx = self.segments.add(seg_id, p);
                        let next = self.segments.get_next_id(sorted_idx);
                        if next.is_some() {
                            let next_id = next.unwrap();
                            let next_ls = self.segments.get(next_id);
                            let ls = self.segments.get(seg_id);
                            if let Intersection::Point(loc) = ls.intersect(next_ls) {
                                assert!(loc.x > p.x);
                                self.events.push(Event::Intersection(loc, seg_id, next_id));
                            }
                        }
                        let prev = self.segments.get_prev_id(sorted_idx);
                        if prev.is_some() {
                            let prev_id = prev.unwrap();
                            let prev_ls = self.segments.get(prev_id);
                            let ls = self.segments.get(seg_id);
                            if let Intersection::Point(loc) = ls.intersect(prev_ls) {
                                assert!(loc.x > p.x);
                                self.events.push(Event::Intersection(loc, prev_id, seg_id));
                            }
                        }
                        self.segments.check_order(p);
                        assert!(p.x >= last.x);
                        assert!((p.x - last.x).abs() > 5.0 || (p.y - last.y).abs() > 5.0);
                        last = p;
                    }
                    Event::End(p, seg_id) => {
                        // println!();
                        println!("ending {seg_id} @ {p}");
                        let sorted_idx = self.segments.get_pos(seg_id);
                        assert_eq!(self.segments.active[sorted_idx], seg_id);
                        let next = self.segments.get_next_id(sorted_idx);
                        let prev = self.segments.get_prev_id(sorted_idx);
                        if next.is_some() && prev.is_some() {
                            let next_id = next.unwrap();
                            let prev_id = prev.unwrap();
                            let next_ls = self.segments.get(next_id);
                            let prev_ls = self.segments.get(prev_id);
                            if let Intersection::Point(loc) = next_ls.intersect(prev_ls) {
                                if loc.x > p.x {
                                    self.events.push(Event::Intersection(loc, prev_id, next_id));
                                }
                            }
                        }
                        self.segments.del(sorted_idx);
                        self.segments.check_order(p);
                        assert!(p.x >= last.x);
                        assert!((p.x - last.x).abs() > 5.0 || (p.y - last.y).abs() > 5.0);
                        last = p;
                    }
                    Event::Intersection(p, s1, s2) => {
                        // println!();
                        println!("intersection between {s1} and {s2} @ {p}");
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
                        // println!("{next:?}");
                        if next.is_some() {
                            let next_id = next.unwrap();
                            let next_ls = self.segments.get(next_id);
                            let ls = self.segments.get(lower_seg);
                            if let Intersection::Point(loc) = ls.intersect(next_ls) {
                                if loc.x > p.x {
                                    self.events
                                        .push(Event::Intersection(loc, lower_seg, next_id));
                                }
                            }
                        }
                        let prev = self.segments.get_prev_id(lower);
                        // println!("{prev:?}");
                        if prev.is_some() {
                            let prev_id = prev.unwrap();
                            let prev_ls = self.segments.get(prev_id);
                            let ls = self.segments.get(upper_seg);
                            if let Intersection::Point(loc) = ls.intersect(prev_ls) {
                                if loc.x > p.x {
                                    self.events
                                        .push(Event::Intersection(loc, prev_id, upper_seg));
                                }
                            }
                        }
                        self.segments.check_order(p);
                        assert!(p.x >= last.x);
                        assert!((p.x - last.x).abs() > 5.0 || (p.y - last.y).abs() > 5.0);
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
        all: [(LineSegment, Option<SortID>); N],
        active: [SegID; N],
        active_count: usize,
    }

    impl<const N: usize> Segments<N> {
        fn check_order(&self, p: Point) {
            let mut last = 0.0;
            self.active[..self.active_count].iter().for_each(|seg| {
                let curr = self.all[*seg].0.point_at_x(p.x).unwrap().y;
                // println!("{seg}\t{curr}");
                assert!(curr > last - 5.0);
                last = curr;
            });
        }

        const fn get(&self, segment: SegID) -> &LineSegment {
            &self.all[segment].0
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

        const fn get_prev(&self, sorted_idx: SortID) -> Option<&LineSegment> {
            let id = self.get_prev_id(sorted_idx);
            if id.is_none() {
                return None;
            }
            Some(self.get(id.unwrap()))
        }

        const fn get_next_id(&self, sorted_idx: SortID) -> Option<SegID> {
            if (sorted_idx + 1) > self.active_count {
                return None;
            }
            Some(self.active[sorted_idx + 1])
        }

        const fn get_next(&self, sorted_idx: SortID) -> Option<&LineSegment> {
            let id = self.get_next_id(sorted_idx);
            if id.is_none() {
                return None;
            }
            Some(self.get(id.unwrap()))
        }

        fn find(&self, p: Point) -> SortID {
            fn find_inner(
                p: Point,
                search: &[SegID],
                segments: &[(LineSegment, Option<SortID>)],
                start_index: SortID,
            ) -> SortID {
                let mid_idx = search.len() / 2;
                let seg_id = search[mid_idx];
                let mid_value = segments[seg_id].0.point_at_x(p.x).unwrap().y;

                if search.len() == 1 {
                    if !equal(p.y, mid_value) && p.y > mid_value {
                        return start_index + 1;
                    }
                    return start_index;
                }
                if search.len() == 2 && !equal(p.y, mid_value) && p.y > mid_value {
                    return start_index + 2;
                }

                if !equal(p.y, mid_value) && p.y > mid_value {
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
            // (0..self.active_count).for_each(|i| {
            //     println!("{i} - {}, {:?}", self.active[i], self.all[self.active[i]]);
            // });
            // (0..self.active_count).for_each(|i| {
            //     assert_eq!(Some(i), self.all[self.active[i]].1);
            // });
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
            // (0..self.active_count).for_each(|i| {
            //     println!("{i} - {}, {:?}", self.active[i], self.all[self.active[i]]);
            // });
            // (0..self.active_count).for_each(|i| {
            //     assert_eq!(Some(i), self.all[self.active[i]].1);
            // });
        }

        fn swap(&mut self, lower: SortID, upper: SortID) {
            assert_eq!(lower + 1, upper); // Should be true?
            self.increment(lower);
            self.decrement(upper);
            self.active.swap(lower, upper);
            // (0..self.active_count).for_each(|i| {
            //     println!("{i} - {}, {:?}", self.active[i], self.all[self.active[i]]);
            // });
            // (0..self.active_count).for_each(|i| {
            //     assert_eq!(Some(i), self.all[self.active[i]].1);
            // });
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
    use std::str::FromStr;

    use crate::{one_sided_line::OneSidedLine, point::Point};

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
        pub fn to_osl(&self) -> OneSidedLine {
            OneSidedLine::new(self.position.to_2d(), self.velocity.to_2d())
        }
    }

    #[derive(Debug)]
    pub struct Point3D {
        pub x: f64,
        pub y: f64,
        pub z: f64,
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

    use crate::equal;

    use super::line;
    use super::line::Line;
    use super::point::Point;

    #[derive(Debug, Clone)]
    pub struct LineSegment {
        pub from: Point,
        pub to: Point,
        pub line: Line,
    }

    impl LineSegment {
        pub fn new(from: Point, to: Point) -> Self {
            // Sort on construction so `from` always has the lower x value
            // or has the lower y value, if there is no slope
            let (a, b) = match (equal(from.x, to.x), from.x - to.x, from.y - to.y) {
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

        pub fn point_at_x(&self, x: f64) -> Option<Point> {
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

    impl Default for LineSegment {
        fn default() -> Self {
            Self {
                from: Point::default(),
                to: (1, 1).into(),
                line: Line::default(),
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

    use crate::is_zero;
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
            write!(f, "{} -> , {}", self.from, self.direction)?;
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
            if is_zero(value.x) {
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

    use crate::{equal, is_zero, point::Point};

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
            let slope = if is_zero(dx) { None } else { Some(dy / dx) };
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
            if is_zero(det) {
                // This tests whether the y axis intercept is different
                // If there is no y axis intercept, in the case of `x = ...` lines,
                // then the x axis intercept is used instead
                // If it is, these lines are parallel and will never intersect
                if equal(self.intercept, other.intercept) {
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

    impl Default for Line {
        fn default() -> Self {
            Self {
                dx: 1.0,
                dy: 1.0,
                c: 0.0,
                slope: Some(1.0),
                y_intercept: Some(0.0),
                intercept: 0.0,
            }
        }
    }

    pub enum Intersection {
        Point(Point),
        Parallel,
        SameLine,
    }
}

mod point {
    use crate::equal;
    use crate::is_zero;

    use std::{
        fmt::Display,
        ops::{Add, Sub},
    };

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
                if is_zero(a) {
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
            equal(self.x, other.x) && equal(self.y, other.y)
        }
    }

    impl Default for Point {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0 }
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
        let tl = (7, 7).into();
        let br = (27, 27).into();
        assert_eq!(2, part_one::<5>(data, tl, br));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
