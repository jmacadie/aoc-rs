#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> i32 {
    data.trim()
        .split(',')
        .map(|s| HexMove::from_str(s).unwrap())
        .fold(HexPoint::new(), |p, m| p.step(m))
        .dist_to_origin()
}

fn part_two(data: &str) -> i32 {
    data.trim()
        .split(',')
        .map(|s| HexMove::from_str(s).unwrap())
        .fold((HexPoint::new(), 0), |(p, d), m| {
            let next = p.step(m);
            let next_distance = next.dist_to_origin();
            (next, std::cmp::max(d, next_distance))
        })
        .1
}

#[derive(Debug)]
struct HexPoint {
    x: i32,
    y: i32,
}

impl HexPoint {
    const fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    const fn step(&self, dir: HexMove) -> Self {
        match dir {
            HexMove::NorthWest => Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            HexMove::North => Self {
                x: self.x,
                y: self.y + 2,
            },
            HexMove::NorthEast => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            HexMove::SouthWest => Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            HexMove::South => Self {
                x: self.x,
                y: self.y - 2,
            },
            HexMove::SouthEast => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
        }
    }

    const fn dist_to_origin(&self) -> i32 {
        let x_bar = self.x.abs();
        let y_bar = self.y.abs();
        if x_bar > y_bar {
            x_bar
        } else {
            x_bar + (y_bar - x_bar) / 2
        }
    }
}

#[derive(Clone, Copy)]
enum HexMove {
    NorthWest,
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
}

impl FromStr for HexMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nw" | "NW" => Ok(Self::NorthWest),
            "n" | "N" => Ok(Self::North),
            "ne" | "NE" => Ok(Self::NorthEast),
            "sw" | "SW" => Ok(Self::SouthWest),
            "s" | "S" => Ok(Self::South),
            "se" | "SE" => Ok(Self::SouthEast),
            _ => Err(format!("{s} is not correctly formatted")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(3, part_one("ne,ne,ne"));
        assert_eq!(0, part_one("ne,ne,sw,sw"));
        assert_eq!(2, part_one("ne,ne,s,s"));
        assert_eq!(3, part_one("se,sw,se,sw,sw"));
    }
}
