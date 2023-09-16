#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let mut cluster = Cluster::from_input(data);
    for _ in 0..10_000 {
        cluster.step_simple();
    }
    cluster.infections
}

fn part_two(data: &str) -> u32 {
    let mut cluster = Cluster::from_input(data);
    for _ in 0..10_000_000 {
        cluster.step_complex();
    }
    cluster.infections
}

const SIZE: usize = 9_999;

struct Cluster {
    nodes: Nodes,
    virus: Virus,
    infections: u32,
}

impl Cluster {
    fn from_input(data: &str) -> Self {
        let mut nodes = Nodes::new();

        let rows = data.lines().count();
        let cols = data.lines().next().unwrap().len();

        let row_offset = (SIZE - rows) / 2;
        let col_offset = (SIZE - cols) / 2;

        for (row, row_data) in data.lines().enumerate() {
            for (col, node_data) in row_data.as_bytes().iter().enumerate() {
                match &node_data {
                    b'#' => nodes.set(Point::new(col + col_offset, row + row_offset), 2),
                    b'.' => {}
                    _ => unreachable!(),
                }
            }
        }

        Self {
            nodes,
            virus: Virus::new(),
            infections: 0,
        }
    }

    fn step_simple(&mut self) {
        if self.nodes.toggle(self.virus.location) == 0 {
            // Infected -> Clean
            self.virus.turn_right();
        } else {
            // Clean -> Infected
            self.infections += 1;
            self.virus.turn_left();
        }
        self.virus.advance();
    }

    fn step_complex(&mut self) {
        match self.nodes.rotate(self.virus.location) {
            1 => {
                // Clean -> Weakened
                self.virus.turn_left();
            }
            2 => {
                // Weakened -> Infected
                self.infections += 1;
            }
            3 => {
                // Infected-> Flagged
                self.virus.turn_right();
            }
            0 => {
                // Flagged-> Clean
                self.virus.reverse();
            }
            _ => unreachable!(),
        }
        self.virus.advance();
    }
}

impl Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const OFFSET: usize = 4;
        let y_start = (SIZE - 1) / 2 - OFFSET;
        let y_len = 2 * OFFSET + 1;
        let x_start = y_start / 4;
        let x_len = (y_len + 3) / 4;
        let virus_start = (self.virus.location.x - 1) / 4;
        let virus_start_offset = (self.virus.location.x - 1) % 4;
        let virus_end = (self.virus.location.x) / 4;
        let virus_end_offset = (self.virus.location.x) % 4;

        for (y, row) in self.nodes.data.iter().skip(y_start).take(y_len).enumerate() {
            for (x, ng) in row.iter().skip(x_start).take(x_len).enumerate() {
                let mut temp = *ng;
                for i in 0..4 {
                    match temp & 0b11 {
                        0 => write!(f, ".")?,
                        1 => write!(f, "W")?,
                        2 => write!(f, "#")?,
                        3 => write!(f, "F")?,
                        _ => unreachable!(),
                    }
                    if y + y_start == self.virus.location.y {
                        if x + x_start == virus_start && i == virus_start_offset {
                            write!(f, "[")?;
                        } else if x + x_start == virus_end && i == virus_end_offset {
                            write!(f, "]")?;
                        } else {
                            write!(f, " ")?;
                        }
                    } else {
                        write!(f, " ")?;
                    }
                    temp >>= 2;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Nodes {
    data: Vec<[NodeGroup; (SIZE + 1) / 4]>,
}

impl Nodes {
    fn new() -> Self {
        Self {
            data: vec![[0; (SIZE + 1) / 4]; SIZE],
        }
    }

    fn set(&mut self, p: Point, val: u8) {
        let x_group = p.x / 4;
        let x_offset = p.x % 4;

        // Clear the bits
        // Not actually needed as only ever using this to write over zero!
        // self.data[p.y][x_group] &= !(0b11 << (2 * x_offset));

        // Set the data
        self.data[p.y][x_group] |= (val & 0b11) << (2 * x_offset);
    }

    fn toggle(&mut self, p: Point) -> u8 {
        let x_group = p.x / 4;
        let x_offset = p.x % 4;
        self.data[p.y][x_group].toggle(x_offset)
    }

    fn rotate(&mut self, p: Point) -> u8 {
        let x_group = p.x / 4;
        let x_offset = p.x % 4;
        self.data[p.y][x_group].rotate(x_offset)
    }
}

#[derive(Debug)]
struct Virus {
    location: Point,
    direction: Direction,
}

impl Virus {
    const fn new() -> Self {
        let centre = (SIZE - 1) / 2;
        Self {
            location: Point::new(centre, centre),
            direction: Direction::Up,
        }
    }

    fn advance(&mut self) {
        self.location.advance(self.direction);
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    fn reverse(&mut self) {
        self.direction = self.direction.reverse();
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn advance(&mut self, dir: Direction) {
        *self = match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    const fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

type NodeGroup = u8;

trait NodeGroupStatus {
    fn toggle(&mut self, position: usize) -> u8;
    fn rotate(&mut self, position: usize) -> u8;
}

impl NodeGroupStatus for NodeGroup {
    fn toggle(&mut self, position: usize) -> u8 {
        // Toggle the 1 / 3 / 5 / 7 bit
        *self ^= 1 << (2 * position + 1);
        // Return the value of the bit pair after toggling
        *self >> (2 * position) & 0b11
    }

    fn rotate(&mut self, position: usize) -> u8 {
        let old = *self >> (2 * position) & 0b11;
        let new = (old + 1) % 4;

        *self &= !(0b11 << (2 * position));
        *self |= new << (2 * position);

        // Return the value of the bit pair after toggling
        new
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn node_group_status_toggle() {
        let mut g: NodeGroup = 0b0000_0000;
        let mut ret;

        ret = g.toggle(0);
        assert_eq!(2, ret);
        assert_eq!(0b0000_0010, g);
        ret = g.toggle(0);
        assert_eq!(0, ret);
        assert_eq!(0b0000_0000, g);

        ret = g.toggle(1);
        assert_eq!(2, ret);
        assert_eq!(0b0000_1000, g);
        ret = g.toggle(1);
        assert_eq!(0, ret);
        assert_eq!(0b0000_0000, g);

        ret = g.toggle(2);
        assert_eq!(2, ret);
        assert_eq!(0b0010_0000, g);
        ret = g.toggle(2);
        assert_eq!(0, ret);
        assert_eq!(0b0000_0000, g);

        ret = g.toggle(3);
        assert_eq!(2, ret);
        assert_eq!(0b1000_0000, g);
        ret = g.toggle(3);
        assert_eq!(0, ret);
        assert_eq!(0b0000_0000, g);

        ret = g.toggle(0);
        assert_eq!(2, ret);
        ret = g.toggle(1);
        assert_eq!(2, ret);
        ret = g.toggle(2);
        assert_eq!(2, ret);
        ret = g.toggle(3);
        assert_eq!(2, ret);
        assert_eq!(0b1010_1010, g);

        ret = g.toggle(2);
        assert_eq!(0, ret);
        assert_eq!(0b1000_1010, g);
    }

    #[test]
    fn one_examples() {
        let data = include_str!("test.txt");
        let mut cluster = Cluster::from_input(data);

        // First 7 iterations
        for _ in 0..7 {
            cluster.step_simple();
        }
        assert_eq!(5, cluster.infections);

        // Next 63 iterations (to 70 overall)
        for _ in 0..63 {
            cluster.step_simple();
        }
        assert_eq!(41, cluster.infections);
    }

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(5_587, part_one(data));
    }

    #[test]
    fn two_examples() {
        let data = include_str!("test.txt");
        let mut cluster = Cluster::from_input(data);

        // First 7 iterations
        for _ in 0..7 {
            cluster.step_complex();
        }
        assert_eq!(1, cluster.infections);

        // Next 93 iterations (to 100 overall)
        for _ in 0..93 {
            cluster.step_complex();
        }
        assert_eq!(26, cluster.infections);
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(2_511_944, part_two(data));
    }
}
