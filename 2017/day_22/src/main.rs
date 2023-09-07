#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

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

const SIZE: usize = 10_001;

// TODO: NodeStatus only needs to be 2 bits, as it only has 4 states
//       Would it be better to pack the row and have u8s covering 4 adjacent nodes?
//       We could also have a node represented by 4 bits, and so pack two to a u8
//       This would make it easier to use bit shifting to move between the states
struct Cluster {
    nodes: Vec<[NodeStatus; SIZE]>,
    virus: Virus,
    infections: u32,
}

impl Cluster {
    fn from_input(data: &str) -> Self {
        let mut nodes = vec![[NodeStatus::Clean; SIZE]; SIZE];

        let rows = data.lines().count();
        let cols = data.lines().next().unwrap().len();

        let row_offset = (SIZE - rows) / 2;
        let col_offset = (SIZE - cols) / 2;

        for (row, row_data) in nodes.iter_mut().skip(row_offset).zip(data.lines()) {
            for (node, node_data) in row.iter_mut().skip(col_offset).zip(row_data.as_bytes()) {
                match &node_data {
                    b'#' => *node = NodeStatus::Infected,
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
        if self.nodes[self.virus.location.y][self.virus.location.x].toggle() {
            // Clean -> Infected
            self.infections += 1;
            self.virus.turn_left();
        } else {
            // Infected -> Clean
            self.virus.turn_right();
        }
        self.virus.advance();
    }

    fn step_complex(&mut self) {
        match self.nodes[self.virus.location.y][self.virus.location.x].rotate() {
            NodeStatus::Clean => {
                // Clean -> Weakened
                self.virus.turn_left();
            }
            NodeStatus::Weakened => {
                // Weakened -> Infected
                self.infections += 1;
            }
            NodeStatus::Infected => {
                // Infected-> Flagged
                self.virus.turn_right();
            }
            NodeStatus::Flagged => {
                // Flagged-> Clean
                self.virus.reverse();
            }
        }
        self.virus.advance();
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeStatus {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeStatus {
    fn toggle(&mut self) -> bool {
        match self {
            Self::Clean => {
                *self = Self::Infected;
                true
            }
            Self::Infected => {
                *self = Self::Clean;
                false
            }
            _ => unreachable!(),
        }
    }

    fn rotate(&mut self) -> Self {
        let current = *self;
        *self = match self {
            Self::Clean => Self::Weakened,
            Self::Weakened => Self::Infected,
            Self::Infected => Self::Flagged,
            Self::Flagged => Self::Clean,
        };
        current
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(5_587, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(2_511_944, part_two(data));
    }
}
