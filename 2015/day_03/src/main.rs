#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let mut visited = HashSet::new();
    let mut current: Location = (0, 0);
    visited.insert(current);

    for instruction in data.trim_end_matches('\n').as_bytes() {
        current = step(&mut visited, current, *instruction);
    }
    visited.len()
}

fn part_two(data: &str) -> usize {
    let mut visited = HashSet::new();
    let mut santa: Location = (0, 0);
    let mut robo: Location = (0, 0);
    visited.insert(santa);

    for instructions in data.trim_end_matches('\n').as_bytes().chunks(2) {
        santa = step(&mut visited, santa, instructions[0]);
        robo = step(&mut visited, robo, instructions[1]);
    }
    visited.len()
}

fn step(visited: &mut HashSet<Location>, start: Location, instruction: u8) -> Location {
    let destination: Location;
    match Direction::new(instruction) {
        Direction::North => destination = (start.0 + 1, start.1),
        Direction::South => destination = (start.0 - 1, start.1),
        Direction::East => destination = (start.0, start.1 + 1),
        Direction::West => destination = (start.0, start.1 - 1),
    }
    visited.insert(destination);
    destination
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn new(instruction: u8) -> Self {
        match instruction {
            b'^' => Self::North,
            b'v' => Self::South,
            b'>' => Self::East,
            b'<' => Self::West,
            _ => unreachable!(),
        }
    }
}

type Location = (i32, i32);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(2, part_one(">"));
        assert_eq!(4, part_one("^>v<"));
        assert_eq!(2, part_one("^v^v^v^v^v"));
    }

    #[test]
    fn two() {
        assert_eq!(3, part_two("^v"));
        assert_eq!(3, part_two("^>v<"));
        assert_eq!(11, part_two("^v^v^v^v^v"));
    }
}
