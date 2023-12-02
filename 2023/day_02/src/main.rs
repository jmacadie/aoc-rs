#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    data.lines()
        .map(|l| l.parse::<Game>().unwrap())
        .filter(Game::is_possible)
        .map(|g| g.number)
        .sum()
}

fn part_two(data: &str) -> u32 {
    data.lines()
        .map(|l| l.parse::<Game>().unwrap())
        .map(|g| g.min_power())
        .sum()
}

#[derive(Debug)]
struct Game {
    number: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rounds = Vec::with_capacity(20);
        let (number, details) = s.split_once(": ").ok_or_else(|| {
            format!("Cannot split the game into the number and the details parts: {s}")
        })?;
        let number = number
            .trim_start_matches("Game ")
            .parse()
            .map_err(|_| format!("Cannot find the game number in {number}"))?;
        for round in details.split("; ") {
            let round = round.parse()?;
            rounds.push(round);
        }
        Ok(Self { number, rounds })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|r| r.is_possible())
    }

    fn min_power(&self) -> u32 {
        let mins = self.rounds.iter().fold((0, 0, 0), |acc, r| {
            let red = std::cmp::max(acc.0, r.red);
            let blue = std::cmp::max(acc.1, r.blue);
            let green = std::cmp::max(acc.2, r.green);
            (red, blue, green)
        });
        mins.0 * mins.1 * mins.2
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for pick in s.split(", ") {
            let (count, cube) = pick
                .split_once(' ')
                .ok_or_else(|| format!("{pick} cannot be split into a number and a colour"))?;
            let count = count
                .parse()
                .map_err(|_| format!("{count} cannot be converted into a number"))?;
            let cube = cube.parse()?;
            match cube {
                Cube::Red => red = count,
                Cube::Blue => blue = count,
                Cube::Green => green = count,
            }
        }
        Ok(Self { red, blue, green })
    }
}

impl Round {
    const fn is_possible(self) -> bool {
        const MAX_RED: u32 = 12;
        const MAX_BLUE: u32 = 14;
        const MAX_GREEN: u32 = 13;

        self.red <= MAX_RED && self.blue <= MAX_BLUE && self.green <= MAX_GREEN
    }
}

#[derive(Debug, Clone, Copy)]
enum Cube {
    Red,
    Blue,
    Green,
}

impl FromStr for Cube {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err(format!("{s} is not a well formed cube")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(8, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(2286, part_two(data));
    }
}
