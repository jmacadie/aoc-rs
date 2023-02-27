#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let parts = data.trim_end_matches('\n').split(", ");
    let mut santa = DirectionLocation::default();
    for instruction in parts {
        santa.travel(instruction);
    }
    santa.manhatten_distance()
}

fn part_two(data: &str) -> u32 {
    let parts = data.trim_end_matches('\n').split(", ");
    let mut santa = DirectionLocation::default();
    let mut visited = HashSet::new();
    visited.insert(santa.location);
    for instruction in parts {
        if santa.travel_recorded(instruction, &mut visited) {
            return santa.manhatten_distance();
        }
        visited.insert(santa.location);
    }
    0
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(self, instruction: char) -> Self {
        match (self, instruction) {
            (Self::North, 'L') | (Self::South, 'R') => Self::West,
            (Self::North, 'R') | (Self::South, 'L') => Self::East,
            (Self::East, 'L') | (Self::West, 'R') => Self::North,
            (Self::East, 'R') | (Self::West, 'L') => Self::South,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn travel(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::North => self.x += distance,
            Direction::East => self.y += distance,
            Direction::South => self.x -= distance,
            Direction::West => self.y -= distance,
        }
    }
}

#[derive(Debug)]
struct DirectionLocation {
    direction: Direction,
    location: Location,
}

impl DirectionLocation {
    fn travel(&mut self, instruction: &str) {
        let (turn, distance) = instruction.split_at(1);

        let turn = turn.chars().next().unwrap();
        self.direction = self.direction.turn(turn);

        let distance = distance.parse().unwrap();

        self.location.travel(self.direction, distance);
    }

    fn travel_recorded(&mut self, instruction: &str, visited: &mut HashSet<Location>) -> bool {
        let (turn, distance) = instruction.split_at(1);

        let turn = turn.chars().next().unwrap();
        self.direction = self.direction.turn(turn);

        let distance = distance.parse().unwrap();

        for _ in 0..distance {
            self.location.travel(self.direction, 1);
            if visited.contains(&self.location) {
                return true;
            }
            visited.insert(self.location);
        }
        false
    }

    fn manhatten_distance(&self) -> u32 {
        u32::try_from(self.location.x.abs()).unwrap()
            + u32::try_from(self.location.y.abs()).unwrap()
    }
}

impl Default for DirectionLocation {
    fn default() -> Self {
        Self {
            direction: Direction::North,
            location: Location::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(5, part_one("R2, L3"));
        assert_eq!(2, part_one("R2, R2, R2"));
        assert_eq!(12, part_one("R5, L5, R5, R3"));
    }

    #[test]
    fn two() {
        assert_eq!(4, part_two("R8, R4, R4, R8"));
        assert_eq!(4, part_two("R8, R4, R4, R8, L10, L12"));
        assert_eq!(4, part_two("R8, R4, R4, R8, R8, R4"));
    }
}
