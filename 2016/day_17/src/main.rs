#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::BinaryHeap, fmt::Display};

use md5::{Digest, Md5};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> Path {
    solve_min(data.trim())
}

fn part_two(data: &str) -> usize {
    let max_path = solve_max(data.trim());
    max_path.len - max_path.root
}

fn solve_max(seed: &str) -> Path {
    let mut heap = BinaryHeap::new();
    let mut max_path = Path::new(seed);

    let s = State::new(seed);

    for m in s.valid_moves() {
        heap.push(m);
    }

    while let Some(s) = heap.pop() {
        if s.location.x == 4 && s.location.y == 4 {
            max_path = s.path;
            continue;
        }
        for m in s.valid_moves() {
            heap.push(m);
        }
    }
    max_path
}

fn solve_min(seed: &str) -> Path {
    let mut heap = BinaryHeap::new();

    let s = State::new(seed);

    for m in s.valid_moves() {
        heap.push(m);
    }

    while let Some(s) = heap.pop() {
        if s.location.x == 4 && s.location.y == 4 {
            return s.path;
        }
        for m in s.valid_moves() {
            heap.push(m);
        }
    }
    Path::new("")
}

#[derive(PartialEq, Eq)]
struct State {
    step: u16,
    heuristic: u16,
    location: Point,
    path: Path,
}

impl State {
    fn new(seed: &str) -> Self {
        Self::from_parts(0, Point::default(), Path::new(seed))
    }

    fn from_parts(step: u16, location: Point, path: Path) -> Self {
        let mut output = Self {
            step,
            heuristic: 0,
            location,
            path,
        };
        output.compute_heuristic();
        output
    }

    fn compute_heuristic(&mut self) {
        let x: u16 = self.location.x.into();
        let y: u16 = self.location.y.into();
        self.heuristic = self.step + 8 - x - y;
    }

    fn valid_moves(&self) -> Vec<Self> {
        let mut moves = Vec::new();

        let dirs = self.get_directions();
        if dirs.open_up() && self.location.x > 1 {
            let new = Self::from_parts(self.step + 1, self.location.move_up(), self.path.move_up());
            moves.push(new);
        }
        if dirs.open_down() && self.location.x < 4 {
            let new = Self::from_parts(
                self.step + 1,
                self.location.move_down(),
                self.path.move_down(),
            );
            moves.push(new);
        }
        if dirs.open_left() && self.location.y > 1 {
            let new = Self::from_parts(
                self.step + 1,
                self.location.move_left(),
                self.path.move_left(),
            );
            moves.push(new);
        }
        if dirs.open_right() && self.location.y < 4 {
            let new = Self::from_parts(
                self.step + 1,
                self.location.move_right(),
                self.path.move_right(),
            );
            moves.push(new);
        }

        moves
    }

    fn get_directions(&self) -> Directions {
        let raw_digest = Md5::digest(self.path.get());
        let digest: &[u8] = raw_digest.as_ref();
        let up = digest[0] >> 4;
        let down = digest[0] & 0b1111;
        let left = digest[1] >> 4;
        let right = digest[1] & 0b1111;

        let mut out = 0;
        for dir in [right, left, down, up] {
            out <<= 1;
            if dir > 10 {
                out |= 1;
            }
        }
        out.into()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heuristic
            .cmp(&self.heuristic)
            .then_with(|| self.step.cmp(&other.step))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl Point {
    const fn move_up(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    const fn move_down(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    const fn move_left(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    const fn move_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    root: usize,
    data: [u8; 1_000],
    len: usize,
}

impl Path {
    fn new(seed: &str) -> Self {
        let seed_u8 = seed.as_bytes();
        let len = seed.len();
        let data = std::array::from_fn(|i| if i < len { seed_u8[i] } else { 0 });
        Self {
            root: len,
            data,
            len,
        }
    }

    fn get(&self) -> &[u8] {
        &self.data[..self.len]
    }

    const fn move_up(&self) -> Self {
        let mut data = self.data;
        data[self.len] = 85; // 'U'
        Self {
            root: self.root,
            data,
            len: self.len + 1,
        }
    }

    const fn move_down(&self) -> Self {
        let mut data = self.data;
        data[self.len] = 68; // 'D'
        Self {
            root: self.root,
            data,
            len: self.len + 1,
        }
    }

    const fn move_left(&self) -> Self {
        let mut data = self.data;
        data[self.len] = 76; // 'L'
        Self {
            root: self.root,
            data,
            len: self.len + 1,
        }
    }

    const fn move_right(&self) -> Self {
        let mut data = self.data;
        data[self.len] = 82; // 'R'
        Self {
            root: self.root,
            data,
            len: self.len + 1,
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.data[self.root..self.len]).unwrap();
        write!(f, "{s}")?;
        Ok(())
    }
}

#[derive(Debug)]
struct Directions(u8);
impl From<u8> for Directions {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Directions {
    const fn open_up(&self) -> bool {
        self.0 & 1 == 1
    }

    const fn open_down(&self) -> bool {
        self.0 & 0b10 == 0b10
    }

    const fn open_left(&self) -> bool {
        self.0 & 0b100 == 0b100
    }

    const fn open_right(&self) -> bool {
        self.0 & 0b1000 == 0b1000
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!("DDRRRD", format!("{}", part_one("ihgpwlah")));
        assert_eq!("DDUDRLRRUDRD", format!("{}", part_one("kglvqrro")));
        assert_eq!(
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR",
            format!("{}", part_one("ulqzkmiv"))
        );
    }

    #[test]
    fn two() {
        assert_eq!(370, part_two("ihgpwlah"));
        assert_eq!(492, part_two("kglvqrro"));
        assert_eq!(830, part_two("ulqzkmiv"));
    }
}
