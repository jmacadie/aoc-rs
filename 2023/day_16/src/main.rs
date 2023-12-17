#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<110>(data));
    println!("Part 2: {}", part_two::<110>(data));
}

fn part_one<const N: usize>(data: &str) -> usize {
    let mut f = data.parse::<Floor<N>>().unwrap();
    f.run(Location::default());
    f.count_on()
}

fn part_two<const N: usize>(data: &str) -> usize {
    let mut f = data.parse::<Floor<N>>().unwrap();
    (0..N)
        .map(|x| Location::new(x, 0, Direction::East))
        .chain((0..N).map(|x| Location::new(x, N - 1, Direction::West)))
        .chain((0..N).map(|x| Location::new(0, x, Direction::South)))
        .chain((0..N).map(|x| Location::new(N - 1, x, Direction::North)))
        .map(|loc| {
            f.run(loc);
            f.count_on()
        })
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Floor<const N: usize> {
    map: [[Tile; N]; N],
    beams: [[Lights; N]; N],
}

impl<const N: usize> FromStr for Floor<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = [[Tile::Empty; N]; N];
        s.lines().enumerate().for_each(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .for_each(|(col, &x)| match x {
                    b'.' => {}
                    b'\\' => map[row][col] = Tile::Backslash,
                    b'/' => map[row][col] = Tile::Slash,
                    b'|' => map[row][col] = Tile::Vertical,
                    b'-' => map[row][col] = Tile::Horizontal,
                    _ => unreachable!(),
                });
        });
        Ok(Self {
            map,
            beams: [[Lights::default(); N]; N],
        })
    }
}

impl<const N: usize> Floor<N> {
    fn run(&mut self, start: Location) {
        self.beams.iter_mut().flatten().for_each(|x| x.data = 0);
        self.follow_beam(start);
    }

    fn follow_beam(&mut self, mut current: Location) {
        while self.can_continue(current) {
            if !self.beams[current.row][current.col].add(current.dir) {
                return;
            }
            let next = current.next(N);
            if next.is_none() {
                return;
            }
            current = next.unwrap();
        }
        match self.map[current.row][current.col] {
            Tile::Empty => unreachable!(),
            Tile::Slash => match current.dir {
                Direction::North | Direction::South => {
                    self.step_and_follow(current.turn(Turn::Right));
                }
                Direction::East | Direction::West => self.step_and_follow(current.turn(Turn::Left)),
            },
            Tile::Backslash => match current.dir {
                Direction::North | Direction::South => {
                    self.step_and_follow(current.turn(Turn::Left));
                }
                Direction::East | Direction::West => {
                    self.step_and_follow(current.turn(Turn::Right));
                }
            },
            Tile::Vertical => match current.dir {
                Direction::North | Direction::South => unreachable!(),
                Direction::East | Direction::West => {
                    self.step_and_follow(current.turn(Turn::Right));
                    self.step_and_follow(current.turn(Turn::Left));
                }
            },
            Tile::Horizontal => match current.dir {
                Direction::East | Direction::West => unreachable!(),
                Direction::North | Direction::South => {
                    self.step_and_follow(current.turn(Turn::Right));
                    self.step_and_follow(current.turn(Turn::Left));
                }
            },
        }
    }

    fn step_and_follow(&mut self, current: Location) {
        if self.beams[current.row][current.col].add(current.dir) {
            if let Some(next) = current.next(N) {
                self.follow_beam(next);
            }
        }
    }

    fn can_continue(&self, from: Location) -> bool {
        let tile = self.map[from.row][from.col];
        tile == Tile::Empty
            || ((from.dir == Direction::South || from.dir == Direction::North)
                && tile == Tile::Vertical)
            || ((from.dir == Direction::East || from.dir == Direction::West)
                && tile == Tile::Horizontal)
    }

    fn count_on(&self) -> usize {
        self.beams.iter().flatten().filter(|&&l| l.on()).count()
    }
}

#[derive(Debug, Clone, Copy)]
struct Location {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Location {
    const fn new(row: usize, col: usize, dir: Direction) -> Self {
        Self { row, col, dir }
    }

    const fn next(self, lim: usize) -> Option<Self> {
        match self.dir {
            Direction::North => {
                if self.row == 0 {
                    return None;
                }
                Some(Self {
                    row: self.row - 1,
                    col: self.col,
                    dir: self.dir,
                })
            }
            Direction::West => {
                if self.col == 0 {
                    return None;
                }
                Some(Self {
                    row: self.row,
                    col: self.col - 1,
                    dir: self.dir,
                })
            }
            Direction::South => {
                if self.row == lim - 1 {
                    return None;
                }
                Some(Self {
                    row: self.row + 1,
                    col: self.col,
                    dir: self.dir,
                })
            }
            Direction::East => {
                if self.col == lim - 1 {
                    return None;
                }
                Some(Self {
                    row: self.row,
                    col: self.col + 1,
                    dir: self.dir,
                })
            }
        }
    }

    const fn turn(self, dir: Turn) -> Self {
        match (dir, self.dir) {
            (Turn::Left, Direction::East) | (Turn::Right, Direction::West) => Self {
                row: self.row,
                col: self.col,
                dir: Direction::North,
            },
            (Turn::Left, Direction::West) | (Turn::Right, Direction::East) => Self {
                row: self.row,
                col: self.col,
                dir: Direction::South,
            },
            (Turn::Left, Direction::North) | (Turn::Right, Direction::South) => Self {
                row: self.row,
                col: self.col,
                dir: Direction::West,
            },
            (Turn::Left, Direction::South) | (Turn::Right, Direction::North) => Self {
                row: self.row,
                col: self.col,
                dir: Direction::East,
            },
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new(0, 0, Direction::East)
    }
}

#[derive(Debug, Clone, Copy)]
struct Lights {
    data: u8,
}

impl Lights {
    const fn new() -> Self {
        Self { data: 0 }
    }

    fn add(&mut self, dir: Direction) -> bool {
        let new = dir.as_u8();
        if new & self.data == 1 {
            return false;
        }
        self.data |= new;
        true
    }

    const fn on(self) -> bool {
        self.data != 0
    }
}

impl Default for Lights {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    const fn as_u8(self) -> u8 {
        match self {
            Self::North => 1,
            Self::West => 2,
            Self::South => 4,
            Self::East => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Backslash,
    Slash,
    Vertical,
    Horizontal,
    Empty,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(46, part_one::<10>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(51, part_two::<10>(data));
    }
}
