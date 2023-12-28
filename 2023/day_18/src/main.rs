#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{cmp::Ordering, fmt::Display, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> i64 {
    let mut h = Horizontals::new(
        data.lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .map(|ins| (ins.dist, ins.dir)),
    );
    h.area()
}

fn part_two(data: &str) -> i64 {
    let mut h = Horizontals::new(
        data.lines()
            .map(|l| l.parse::<Instruction>().unwrap().colour),
    );
    h.area()
}

struct Horizontals {
    data: Vec<LocatedInterval>,
}

impl Horizontals {
    fn new(source: impl Iterator<Item = (i64, Direction)>) -> Self {
        let mut row = 0;
        let mut col = 0;
        let mut data = Vec::with_capacity(400);
        for (dist, dir) in source {
            match dir {
                Direction::Up => {
                    row -= dist;
                }
                Direction::Down => {
                    row += dist;
                }
                Direction::Left => {
                    data.push(LocatedInterval::new((col - dist, col).into(), row));
                    col -= dist;
                }
                Direction::Right => {
                    data.push(LocatedInterval::new((col, col + dist).into(), row));
                    col += dist;
                }
            }
        }
        data.sort_unstable();
        Self { data }
    }

    fn area(&mut self) -> i64 {
        let mut area = 0;
        let mut verticals = Vec::with_capacity(400);
        while let Some(top) = self.data.pop() {
            area += self.process_row(&top, 0, &mut verticals);
        }
        area
    }

    fn process_row(
        &mut self,
        top: &LocatedInterval,
        skip: usize,
        verticals: &mut Vec<LocatedInterval>,
    ) -> i64 {
        if let Some((row_num, row, mut next_skip, Some(intersection))) = self
            .data
            .iter()
            .enumerate()
            .rev()
            .enumerate()
            .skip(skip)
            .map(|(next_skip, (row_num, row))| (row_num, row, next_skip, top.intersect_any(row)))
            .find(|&(_, _, _, int)| int.is_some())
        {
            let area = (intersection.to - intersection.from + 1)
                * (row.location - top.location + 1)
                + Self::add_verticals(
                    (top.location, row.location).into(),
                    intersection.from,
                    intersection.to,
                    verticals,
                );
            // Update our records for what's left of the bottom row we matched to
            match row.subtract(intersection) {
                (None, None) => {
                    self.data.remove(row_num);
                }
                (Some(a), None) => {
                    let _ = std::mem::replace(&mut self.data[row_num], a);
                    next_skip += 1;
                }
                (Some(a), Some(b)) => {
                    let _ = std::mem::replace(&mut self.data[row_num], a);
                    self.data.insert(row_num, b);
                    next_skip += 2;
                }
                _ => unreachable!(),
            }
            // Process what's left of the top row we popped off
            // If none left, just return the found area
            // If some left over, recursively process the residual
            match top.subtract(intersection) {
                (None, None) => {
                    return area;
                }
                (Some(a), None) => {
                    return area + self.process_row(&a, next_skip, verticals);
                }
                (Some(a), Some(b)) => {
                    return area
                        + self.process_row(&a, next_skip, verticals)
                        + self.process_row(&b, next_skip, verticals);
                }
                _ => unreachable!(),
            }
        }
        0
    }

    fn add_verticals(
        vert: Interval,
        from: i64,
        to: i64,
        verticals: &mut Vec<LocatedInterval>,
    ) -> i64 {
        let left = LocatedInterval::new(vert, from);
        let right = LocatedInterval::new(vert, to);
        let double_count_adj = verticals.iter().fold(0, |acc, v| {
            if let Some(left_double) = v.intersect(&left) {
                return acc - left_double.len();
            }
            if let Some(right_double) = v.intersect(&right) {
                return acc - right_double.len();
            }
            acc
        });
        verticals.push(left);
        verticals.push(right);
        double_count_adj
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Interval {
    from: i64,
    to: i64,
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.from, self.to)?;
        Ok(())
    }
}

impl From<(i64, i64)> for Interval {
    fn from(value: (i64, i64)) -> Self {
        Self {
            from: value.0,
            to: value.1,
        }
    }
}

impl Interval {
    fn intersect(self, other: Self) -> Option<Self> {
        if self.to <= other.from || other.to <= self.from {
            return None;
        }
        let from = std::cmp::max(self.from, other.from);
        let to = std::cmp::min(self.to, other.to);
        Some((from, to).into())
    }

    fn subtract(self, other: Self) -> (Option<Self>, Option<Self>) {
        match (
            self.from.cmp(&other.from),
            self.to.cmp(&other.to),
            self.from.cmp(&other.to),
            self.to.cmp(&other.from),
        ) {
            (_, _, Ordering::Greater | Ordering::Equal, _)
            | (_, _, _, Ordering::Less | Ordering::Equal) => (Some(self), None),
            (Ordering::Greater | Ordering::Equal, Ordering::Less | Ordering::Equal, _, _) => {
                (None, None)
            }
            (Ordering::Greater | Ordering::Equal, Ordering::Greater, _, _) => {
                (Some((other.to, self.to).into()), None)
            }
            (Ordering::Less, Ordering::Less | Ordering::Equal, _, _) => {
                (Some((self.from, other.from).into()), None)
            }
            (Ordering::Less, Ordering::Greater, _, _) => (
                Some((self.from, other.from).into()),
                Some((other.to, self.to).into()),
            ),
        }
    }

    const fn len(self) -> i64 {
        self.to - self.from + 1
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LocatedInterval {
    interval: Interval,
    location: i64,
}

impl LocatedInterval {
    const fn new(interval: Interval, location: i64) -> Self {
        Self { interval, location }
    }

    fn intersect(&self, other: &Self) -> Option<Interval> {
        if self.location != other.location {
            return None;
        }
        self.intersect_any(other)
    }

    fn intersect_any(&self, other: &Self) -> Option<Interval> {
        self.interval.intersect(other.interval)
    }

    fn subtract(&self, int: Interval) -> (Option<Self>, Option<Self>) {
        let (a, b) = self.interval.subtract(int);
        (
            a.map(|int| Self::new(int, self.location)),
            b.map(|int| Self::new(int, self.location)),
        )
    }
}

impl Display for LocatedInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.interval, self.location)?;
        Ok(())
    }
}

impl Ord for LocatedInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        other.location.cmp(&self.location)
    }
}

impl PartialOrd for LocatedInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    dist: i64,
    colour: (i64, Direction),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let (Some(dir), Some(dist), Some(colour), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(format!("Cannot split instruction into three parts: {s}"));
        };
        let dir = dir.parse()?;
        let dist = dist
            .parse()
            .map_err(|_| format!("{dist} cannot be conveted into a number"))?;
        let colour = colour.trim_start_matches("(#").trim_end_matches(')');
        if colour.len() != 6 {
            return Err(format!("The colour {colour} is badly formatted"));
        }
        let l = i64::from_str_radix(&colour[0..5], 16).map_err(|_| {
            format!("Cannot convert the distance part of the colour correctly: {colour}")
        })?;
        let d = match colour.chars().nth(5) {
            Some('0') => Direction::Right,
            Some('1') => Direction::Down,
            Some('2') => Direction::Left,
            Some('3') => Direction::Up,
            _ => {
                return Err(format!("Final colour instruction is not valid: {colour}"));
            }
        };
        Ok(Self {
            dir,
            dist,
            colour: (l, d),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => Err(format!("{s} is not a valid direction")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(62, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(952_408_144_115, part_two(data));
    }
}
