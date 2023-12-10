#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

pub fn main() {
    const R: usize = 140;
    const C: usize = 140;
    let data = include_str!("input.txt");
    let mut l = Landscape::<R, C>::new(data);
    l.run();
    println!("Part 1: {}", part_one::<R, C>(&l));
    println!("Part 2: {}", part_two::<R, C>(l));
}

fn part_one<const R: usize, const C: usize>(l: &Landscape<'_, R, C>) -> u32 {
    l.steps / 2
}

fn part_two<const R: usize, const C: usize>(mut l: Landscape<'_, R, C>) -> usize {
    l.count_inner()
}

struct Landscape<'a, const R: usize, const C: usize> {
    data: &'a [u8],
    mapped: [[Option<MapType>; C]; R],
    location: usize,
    tile: Tile,
    direction: Direction,
    steps: u32,
}

impl<'a, const R: usize, const C: usize> Landscape<'a, R, C> {
    fn new(data: &'a str) -> Self {
        let data = data.as_bytes();
        let mapped = [[None; C]; R];
        let (location, direction) = Self::get_start(data);
        Self {
            data,
            mapped,
            location,
            tile: Tile::Start,
            direction,
            steps: 0,
        }
    }

    fn get_start(data: &[u8]) -> (usize, Direction) {
        let location = data.iter().position(|&t| t == b'S').unwrap();
        if location > C {
            let tile = Tile::from_byte(data[location - C - 1]);
            if tile == Tile::VerticalPipe
                || tile == Tile::SouthWestPipe
                || tile == Tile::SouthEastPipe
            {
                return (location, Direction::North);
            }
        }
        if location + C + 1 < data.len() {
            let tile = Tile::from_byte(data[location + C + 1]);
            if tile == Tile::VerticalPipe
                || tile == Tile::NorthWestPipe
                || tile == Tile::NorthEastPipe
            {
                return (location, Direction::South);
            }
        }
        if location > 0 {
            let tile = Tile::from_byte(data[location - 1]);
            if tile == Tile::HorizontalPipe
                || tile == Tile::SouthEastPipe
                || tile == Tile::NorthEastPipe
            {
                return (location, Direction::West);
            }
        }
        (location, Direction::East)
    }

    fn run(&mut self) {
        self.step();
        while self.tile != Tile::Start {
            self.step();
        }
    }

    fn step(&mut self) {
        self.location = match self.direction {
            Direction::North => self.location - C - 1,
            Direction::South => self.location + C + 1,
            Direction::East => self.location + 1,
            Direction::West => self.location - 1,
        };
        self.tile = Tile::from_byte(self.data[self.location]);
        self.add_mappings();
        if self.tile != Tile::Start {
            self.direction = self.tile.next_direction(self.direction.invert());
        }
        self.steps += 1;
    }

    fn count_inner(&mut self) -> usize {
        self.flood_fill();
        match self.mapped[0][0] {
            Some(MapType::Left) => self
                .mapped
                .iter()
                .flatten()
                .filter(|&&m| m == Some(MapType::Right))
                .count(),
            Some(MapType::Right) => self
                .mapped
                .iter()
                .flatten()
                .filter(|&&m| m == Some(MapType::Left))
                .count(),
            _ => unreachable!(),
        }
    }

    fn add_mappings(&mut self) {
        self.add_mapping((0, Seek::Forwards), MapType::Pipe);
        match (self.tile, self.direction) {
            (Tile::Start, _) => {} // don't know how to map this
            (Tile::VerticalPipe, Direction::North) => {
                self.add_mapping((1, Seek::Backwards), MapType::Left);
                self.add_mapping((1, Seek::Forwards), MapType::Right);
            }
            (Tile::VerticalPipe, Direction::South) => {
                self.add_mapping((1, Seek::Backwards), MapType::Right);
                self.add_mapping((1, Seek::Forwards), MapType::Left);
            }
            (Tile::HorizontalPipe, Direction::East) => {
                self.add_mapping((C + 1, Seek::Backwards), MapType::Left);
                self.add_mapping((C + 1, Seek::Forwards), MapType::Right);
            }
            (Tile::HorizontalPipe, Direction::West) => {
                self.add_mapping((C + 1, Seek::Backwards), MapType::Right);
                self.add_mapping((C + 1, Seek::Forwards), MapType::Left);
            }
            (Tile::NorthEastPipe, Direction::South) => {
                self.add_mapping((1, Seek::Backwards), MapType::Right);
                self.add_mapping((C + 1, Seek::Forwards), MapType::Right);
            }
            (Tile::NorthEastPipe, Direction::West) => {
                self.add_mapping((1, Seek::Backwards), MapType::Left);
                self.add_mapping((C + 1, Seek::Forwards), MapType::Left);
            }
            (Tile::NorthWestPipe, Direction::South) => {
                self.add_mapping((1, Seek::Forwards), MapType::Left);
                self.add_mapping((C + 1, Seek::Forwards), MapType::Left);
            }
            (Tile::NorthWestPipe, Direction::East) => {
                self.add_mapping((1, Seek::Forwards), MapType::Right);
                self.add_mapping((C + 1, Seek::Forwards), MapType::Right);
            }
            (Tile::SouthEastPipe, Direction::North) => {
                self.add_mapping((1, Seek::Backwards), MapType::Left);
                self.add_mapping((C + 1, Seek::Backwards), MapType::Left);
            }
            (Tile::SouthEastPipe, Direction::West) => {
                self.add_mapping((1, Seek::Backwards), MapType::Right);
                self.add_mapping((C + 1, Seek::Backwards), MapType::Right);
            }
            (Tile::SouthWestPipe, Direction::North) => {
                self.add_mapping((1, Seek::Forwards), MapType::Right);
                self.add_mapping((C + 1, Seek::Backwards), MapType::Right);
            }
            (Tile::SouthWestPipe, Direction::East) => {
                self.add_mapping((1, Seek::Forwards), MapType::Left);
                self.add_mapping((C + 1, Seek::Backwards), MapType::Left);
            }
            _ => unreachable!(),
        }
    }

    fn add_mapping(&mut self, offset: (usize, Seek), map_type: MapType) {
        if (offset.1 == Seek::Backwards && self.location < offset.0)
            || (offset.1 == Seek::Forwards && self.location + offset.0 >= R * (C + 1))
        {
            return;
        }
        let new = if offset.1 == Seek::Forwards {
            self.location + offset.0
        } else {
            self.location - offset.0
        };
        let col = new % (C + 1);
        if col == C {
            return;
        }
        let row = new / (C + 1);
        match (self.mapped[row][col], map_type) {
            (Some(MapType::Pipe), MapType::Pipe) => {
                unreachable!();
            }
            (Some(MapType::Left), MapType::Right) | (Some(MapType::Right), MapType::Left) => {
                self.mapped[row][col] = None;
            }
            (Some(MapType::Pipe), _)
            | (Some(MapType::Left), MapType::Left)
            | (Some(MapType::Right), MapType::Right) => {}
            (Some(MapType::Left | MapType::Right), MapType::Pipe) | (None, _) => {
                self.mapped[row][col] = Some(map_type);
            }
        }
    }

    fn flood_fill(&mut self) {
        for row in 0..R {
            for col in 0..C {
                if self.mapped[row][col].is_none() {
                    if let Some(value) = self.get_neighbours(row, col) {
                        self.flood_fill_point(row, col, value);
                    }
                }
            }
        }
    }

    fn flood_fill_point(&mut self, row: usize, col: usize, value: MapType) {
        self.mapped[row][col] = Some(value);
        if row > 0 && self.mapped[row - 1][col].is_none() {
            self.flood_fill_point(row - 1, col, value);
        }
        if row < (R - 1) && self.mapped[row + 1][col].is_none() {
            self.flood_fill_point(row + 1, col, value);
        }
        if col > 0 && self.mapped[row][col - 1].is_none() {
            self.flood_fill_point(row, col - 1, value);
        }
        if col < (C - 1) && self.mapped[row][col + 1].is_none() {
            self.flood_fill_point(row, col + 1, value);
        }
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Option<MapType> {
        let mut neighbour_type = None;
        if row > 0 {
            neighbour_type = Self::update_neighbour(neighbour_type, self.mapped[row - 1][col]);
        }
        if row < (R - 1) {
            neighbour_type = Self::update_neighbour(neighbour_type, self.mapped[row + 1][col]);
        }
        if col > 0 {
            neighbour_type = Self::update_neighbour(neighbour_type, self.mapped[row][col - 1]);
        }
        if col < (C - 1) {
            neighbour_type = Self::update_neighbour(neighbour_type, self.mapped[row][col + 1]);
        }
        neighbour_type
    }

    fn update_neighbour(current: Option<MapType>, next: Option<MapType>) -> Option<MapType> {
        if next.is_some() && current.is_some() && next != current {
            unreachable!();
        }
        if next == Some(MapType::Pipe) {
            return current;
        }
        current.or(next)
    }
}

impl<'a, const R: usize, const C: usize> Display for Landscape<'a, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.mapped {
            for cell in row {
                match cell {
                    None => write!(f, ".")?,
                    Some(MapType::Pipe) => write!(f, "*")?,
                    Some(MapType::Left) => write!(f, "L")?,
                    Some(MapType::Right) => write!(f, "R")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seek {
    Forwards,
    Backwards,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapType {
    Pipe,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    Start,
}

impl Tile {
    fn from_byte(tile: u8) -> Self {
        match tile {
            b'|' => Self::VerticalPipe,
            b'-' => Self::HorizontalPipe,
            b'L' => Self::NorthEastPipe,
            b'J' => Self::NorthWestPipe,
            b'7' => Self::SouthWestPipe,
            b'F' => Self::SouthEastPipe,
            b'.' => Self::Ground,
            b'S' => Self::Start,
            _ => unreachable!(),
        }
    }

    fn next_direction(self, in_direction: Direction) -> Direction {
        match (self, in_direction) {
            (Self::VerticalPipe, Direction::South)
            | (Self::NorthWestPipe, Direction::West)
            | (Self::NorthEastPipe, Direction::East) => Direction::North,
            (Self::VerticalPipe, Direction::North)
            | (Self::SouthWestPipe, Direction::West)
            | (Self::SouthEastPipe, Direction::East) => Direction::South,
            (Self::HorizontalPipe, Direction::West)
            | (Self::NorthEastPipe, Direction::North)
            | (Self::SouthEastPipe, Direction::South) => Direction::East,
            (Self::HorizontalPipe, Direction::East)
            | (Self::NorthWestPipe, Direction::North)
            | (Self::SouthWestPipe, Direction::South) => Direction::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn invert(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_a() {
        const R: usize = 5;
        const C: usize = 5;
        let data = include_str!("test_a.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(4, part_one::<R, C>(&l));
    }

    #[test]
    fn one_b() {
        const R: usize = 5;
        const C: usize = 5;
        let data = include_str!("test_b.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(8, part_one::<R, C>(&l));
    }

    #[test]
    fn two_a() {
        const R: usize = 5;
        const C: usize = 5;
        let data = include_str!("test_a.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(1, part_two::<R, C>(l));
    }

    #[test]
    fn two_b() {
        const R: usize = 5;
        const C: usize = 5;
        let data = include_str!("test_b.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(1, part_two::<R, C>(l));
    }

    #[test]
    fn two_c() {
        const R: usize = 9;
        const C: usize = 11;
        let data = include_str!("test_c.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(4, part_two::<R, C>(l));
    }

    #[test]
    fn two_d() {
        const R: usize = 9;
        const C: usize = 10;
        let data = include_str!("test_d.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(4, part_two::<R, C>(l));
    }

    #[test]
    fn two_e() {
        const R: usize = 10;
        const C: usize = 20;
        let data = include_str!("test_e.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(8, part_two::<R, C>(l));
    }

    #[test]
    fn two_f() {
        const R: usize = 10;
        const C: usize = 20;
        let data = include_str!("test_f.txt");
        let mut l = Landscape::<R, C>::new(data);
        l.run();
        assert_eq!(10, part_two::<R, C>(l));
    }
}
