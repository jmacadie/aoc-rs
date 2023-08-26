#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> String {
    let mut t = Tubes::new(data);
    t.follow_tube();
    t.found_letters()
}

fn part_two(data: &str) -> u32 {
    let mut t = Tubes::new(data);
    t.follow_tube();
    t.distance
}

const VERTICAL: u8 = b'|';
const HORIZONTAL: u8 = b'-';
const CORNER: u8 = b'+';

#[derive(Debug)]
struct Tubes<'a> {
    data: &'a [u8],
    row_length: usize,
    location: Location,
    letters: [Option<u8>; 20],
    at_end: bool,
    distance: u32,
}

impl<'a> Tubes<'a> {
    fn new(data: &'a str) -> Self {
        let data = data.as_bytes();
        let row_length = data.iter().position(|&b| b == b'\n').unwrap() + 1;
        let start = data.iter().position(|&b| b == b'|').unwrap();
        Self {
            data,
            row_length,
            location: Location::new(start, 0),
            letters: [None; 20],
            at_end: false,
            distance: 0,
        }
    }

    fn follow_tube(&mut self) {
        while !self.at_end {
            self.step();
        }
    }

    fn step(&mut self) {
        let mut next = self.location.step();
        match (self.get_val(next), next.direction) {
            (Some(VERTICAL | HORIZONTAL), _) => {} // nothing to do
            (Some(CORNER), Direction::Up | Direction::Down) => {
                // Turn right / left
                let left = self.get_val(next.turn(Direction::Left).step());
                let right = self.get_val(next.turn(Direction::Right).step());
                match (left, right) {
                    (Some(HORIZONTAL), _) => next = next.turn(Direction::Left),
                    (Some(ch), _) if ch.is_ascii_uppercase() => next = next.turn(Direction::Left),
                    (_, Some(HORIZONTAL)) => next = next.turn(Direction::Right),
                    (_, Some(ch)) if ch.is_ascii_uppercase() => next = next.turn(Direction::Right),
                    (_, _) => unreachable!(),
                }
            }
            (Some(CORNER), Direction::Left | Direction::Right) => {
                // Turn up / down
                let up = self.get_val(next.turn(Direction::Up).step());
                let down = self.get_val(next.turn(Direction::Down).step());
                match (up, down) {
                    (Some(VERTICAL), _) => next = next.turn(Direction::Up),
                    (Some(ch), _) if ch.is_ascii_uppercase() => next = next.turn(Direction::Up),
                    (_, Some(VERTICAL)) => next = next.turn(Direction::Down),
                    (_, Some(ch)) if ch.is_ascii_uppercase() => next = next.turn(Direction::Down),
                    (_, _) => unreachable!(),
                }
            }
            (Some(ch), _) if ch.is_ascii_uppercase() => {
                // record the letter
                self.add_letter(ch);
            }
            (_, _) => {
                // At end of tube
                self.at_end = true;
            }
        }
        self.location = next;
        self.distance += 1;
    }

    fn add_letter(&mut self, letter: u8) {
        if let Some(index) = self.letters.iter().position(Option::is_none) {
            self.letters[index] = Some(letter);
        }
    }

    const fn get_val(&self, loc: Location) -> Option<u8> {
        let index = loc.point.y * (self.row_length) + loc.point.x;
        if index >= self.data.len() {
            return None;
        }
        Some(self.data[index])
    }

    fn found_letters(&self) -> String {
        let letter_vec = self.letters.iter().filter_map(|&l| l).collect();
        String::from_utf8(letter_vec).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
struct Location {
    point: Point,
    direction: Direction,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Self {
            point: Point::new(x, y),
            direction: Direction::default(),
        }
    }

    const fn step(&self) -> Self {
        Self {
            point: self.point.step(self.direction),
            direction: self.direction,
        }
    }

    const fn turn(&self, dir: Direction) -> Self {
        Self {
            point: self.point,
            direction: dir,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    const fn step(&self, dir: Direction) -> Self {
        match dir {
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Self {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn new() -> Self {
        Self::Down
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!("ABCDEF", part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(38, part_two(data));
    }
}
