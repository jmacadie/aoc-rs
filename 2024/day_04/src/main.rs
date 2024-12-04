#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::ops::{Add, Mul};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<140>(data));
    println!("Part 2: {}", part_two::<140>(data));
}

fn part_one<const N: usize>(data: &'static str) -> usize {
    Grid::<N>::new(data).find_xmas()
}

fn part_two<const N: usize>(data: &'static str) -> usize {
    Grid::<N>::new(data).find_x_mas()
}

type GridVal = u8;

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    row: i16,
    col: i16,
}

impl Point {
    const fn new(row: i16, col: i16) -> Self {
        Self { row, col }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.as_vector()
    }
}

impl Mul<i16> for Point {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl Mul<i16> for Direction {
    type Output = Point;

    fn mul(self, rhs: i16) -> Self::Output {
        self.as_vector() * rhs
    }
}

#[derive(Debug)]
struct Grid<const N: usize> {
    data: &'static [GridVal],
}

impl<const N: usize> Grid<N> {
    const fn new(data: &'static str) -> Self {
        Self {
            data: data.as_bytes(),
        }
    }

    fn loc(index: usize) -> Point {
        let row = (index / (N + 1)).try_into().unwrap();
        let col = (index % (N + 1)).try_into().unwrap();
        Point::new(row, col)
    }

    fn val(&self, p: Point) -> GridVal {
        let row = usize::try_from(p.row).unwrap();
        let col = usize::try_from(p.col).unwrap();
        let idx = row * (N + 1) + col;
        self.data[idx]
    }

    fn find_vals(&self, val: GridVal) -> impl Iterator<Item = Point> + '_ {
        self.data
            .iter()
            .enumerate()
            .filter(move |&(_, v)| *v == val)
            .map(|(idx, _)| Self::loc(idx))
    }

    fn in_bounds(p: Point) -> bool {
        let lim = N.try_into().unwrap();
        p.row >= 0 && p.row < lim && p.col >= 0 && p.col < lim
    }

    fn count_mas(&self, p: Point) -> usize {
        Direction::all()
            .map(|dir| (p + dir, p + dir * 2, p + dir * 3))
            .filter(|&(p1, p2, p3)| {
                Self::in_bounds(p3)
                    && self.val(p1) == b'M'
                    && self.val(p2) == b'A'
                    && self.val(p3) == b'S'
            })
            .count()
    }

    fn find_xmas(&self) -> usize {
        self.find_vals(b'X').map(|p| self.count_mas(p)).sum()
    }

    fn is_x_mas(&self, p: Point) -> bool {
        let tl = p + Direction::NorthWest;
        let tr = p + Direction::NorthEast;
        let br = p + Direction::SouthEast;
        let bl = p + Direction::SouthWest;
        if !Self::in_bounds(tl)
            || !Self::in_bounds(tr)
            || !Self::in_bounds(br)
            || !Self::in_bounds(bl)
        {
            return false;
        }

        let top_left = self.val(tl);
        let top_right = self.val(tr);
        let bot_right = self.val(br);
        let bot_left = self.val(bl);
        ((top_left == b'M' && bot_right == b'S') || (top_left == b'S' && bot_right == b'M'))
            && ((top_right == b'M' && bot_left == b'S') || (top_right == b'S' && bot_left == b'M'))
    }

    fn find_x_mas(&self) -> usize {
        self.find_vals(b'A').filter(|&p| self.is_x_mas(p)).count()
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Direction {
    const fn as_vector(self) -> Point {
        match self {
            Self::NorthWest => Point::new(-1, -1),
            Self::North => Point::new(-1, 0),
            Self::NorthEast => Point::new(-1, 1),
            Self::East => Point::new(0, 1),
            Self::SouthEast => Point::new(1, 1),
            Self::South => Point::new(1, 0),
            Self::SouthWest => Point::new(1, -1),
            Self::West => Point::new(0, -1),
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        [
            Self::NorthWest,
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(18, part_one::<10>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(9, part_two::<10>(data));
    }
}
