#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(|l| l.parse::<Claim>().unwrap())
        .fold(Fabric::new(), |mut fabric, claim| {
            fabric.add_claim(&claim);
            fabric
        })
        .count_overclaims()
}

fn part_two(data: &str) -> u16 {
    let mut claims = Vec::with_capacity(1_327);
    claims.extend(data.lines().map(|l| l.parse::<Claim>().unwrap()));

    for (i, claim) in claims.iter().enumerate() {
        if !claims
            .iter()
            .take(i)
            .chain(claims.iter().skip(i + 1))
            .any(|other| claim.overlaps(other))
        {
            return claim.number;
        }
    }
    0
}

struct Fabric {
    data: Box<[[u8; 1000]]>,
}

impl Fabric {
    fn new() -> Self {
        Self {
            data: vec![[0; 1000]; 1000].into_boxed_slice(),
        }
    }

    fn add_claim(&mut self, claim: &Claim) {
        for row in 0..claim.height {
            let full_row = usize::from(claim.location.y + row);
            for col in 0..claim.width {
                let full_col = usize::from(claim.location.x + col);
                self.data[full_row][full_col] += 1;
            }
        }
    }

    fn count_overclaims(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&&v| v > 1).count())
            .sum()
    }
}

#[derive(Debug)]
struct Claim {
    number: u16,
    location: Point,
    width: u16,
    height: u16,
}

impl Claim {
    fn end(&self) -> Point {
        (
            self.location.x + self.width - 1,
            self.location.y + self.height - 1,
        )
            .into()
    }

    fn overlaps(&self, other: &Self) -> bool {
        let end = self.end();
        let other_end = other.end();

        !(self.location.x > other_end.x
            || self.location.y > other_end.y
            || other.location.x > end.x
            || other.location.y > end.y)
    }
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (number, rest) = s
            .split_once(' ')
            .ok_or_else(|| format!("Cannot split the number off the start of the claim: {s}"))?;
        let number = number
            .trim_start_matches('#')
            .parse::<u16>()
            .map_err(|_| format!("Cannot convert {number} into a number"))?;
        let (location, dimensions) = rest
            .trim_start_matches("@ ")
            .split_once(": ")
            .ok_or_else(|| format!("Cannot split the location from the dimensions: {rest}"))?;
        let (x, y) = location.split_once(',').ok_or_else(|| {
            format!("Cannot split the location into x and y co-ordinates: {location}")
        })?;
        let x = x
            .parse()
            .map_err(|_| format!("Cannot convert x co-ordinate into a number: {x}"))?;
        let y = y
            .parse()
            .map_err(|_| format!("Cannot convert y co-ordinate into a number: {y}"))?;
        let (width, height) = dimensions.split_once('x').ok_or_else(|| {
            format!("Cannot split the dimensions into height and width: {dimensions}")
        })?;
        let width = width
            .parse()
            .map_err(|_| format!("Cannot convert width into a number: {width}"))?;
        let height = height
            .parse()
            .map_err(|_| format!("Cannot convert height into a number: {height}"))?;
        Ok(Self {
            number,
            location: (x, y).into(),
            width,
            height,
        })
    }
}

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(4, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(3, part_two(data));
    }
}
