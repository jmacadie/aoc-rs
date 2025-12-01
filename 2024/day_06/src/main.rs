#![warn(clippy::all, clippy::pedantic)]

use std::ops::{Add, Mul};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<130>(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one<const N: usize>(data: &'static str) -> usize {
    let _g = Grid::<N>::new(data);
    // println!("{p:?}, {d:?}");
    N
}

const fn part_two(_data: &str) -> usize {
    0
}

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
struct Position {
    p: Point,
    d: Direction,
}

impl Position {
    fn new(p: Point, d: Direction) -> Self {
        Self { p, d }
    }
}

#[derive(Debug)]
struct Grid<const N: usize> {
    data: &'static [u8],
    position: Position,
    visited: Vec<u8>,
}

impl<const N: usize> Grid<N> {
    fn new(data: &'static str) -> Self {
        let visited_size = (N * N / 8) + 1;
        let idx = data.bytes().position(|c| c == b'^').unwrap();
        let row = i16::try_from(idx / (N + 1)).unwrap();
        let col = i16::try_from(idx % (N + 1)).unwrap();
        let position = Position::new(Point::new(row, col), Direction::North);

        Self {
            data: data.as_bytes(),
            position,
            visited: Vec::with_capacity(visited_size),
        }
    }

    fn record_visit(&mut self) {
        let row = usize::try_from(self.position.p.row).unwrap();
        let col = usize::try_from(self.position.p.col).unwrap();
        let idx = row * N + col;
        let visited_idx_maj = idx / 8;
        let visited_idx_min = idx % 8;
        self.visited[visited_idx_maj] &= 1 << visited_idx_min;
    }

    fn loc(index: usize) -> Point {
        let row = (index / (N + 1)).try_into().unwrap();
        let col = (index % (N + 1)).try_into().unwrap();
        Point::new(row, col)
    }

    fn val(&self, p: Point) -> u8 {
        let row = usize::try_from(p.row).unwrap();
        let col = usize::try_from(p.col).unwrap();
        let idx = row * (N + 1) + col;
        self.data[idx]
    }

    fn in_bounds(p: Point) -> bool {
        let lim = N.try_into().unwrap();
        p.row >= 0 && p.row < lim && p.col >= 0 && p.col < lim
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn as_vector(self) -> Point {
        match self {
            Self::North => Point::new(-1, 0),
            Self::East => Point::new(0, 1),
            Self::South => Point::new(1, 0),
            Self::West => Point::new(0, -1),
        }
    }

    fn all() -> impl Iterator<Item = Self> {
        [Self::North, Self::East, Self::South, Self::West].into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one::<10>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
