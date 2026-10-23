#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .skip(30)
        .map(Case::new)
        .filter(Case::fits)
        .count()
}

struct Case {
    height: u32,
    width: u32,
    tiles: u32,
}

impl Case {
    fn new(input: &str) -> Self {
        let Some((dims, tiles)) = input.split_once(": ") else {
            unreachable!();
        };
        let Some((height, width)) = dims.split_once('x') else {
            unreachable!();
        };
        let tiles = tiles
            .split_whitespace()
            .map(|d| d.parse::<u32>().unwrap())
            .sum();
        let height = height.parse::<u32>().unwrap();
        let width = width.parse::<u32>().unwrap();
        Self {
            height,
            width,
            tiles,
        }
    }

    const fn fits(&self) -> bool {
        (self.height / 3) * (self.width / 3) >= self.tiles
    }
}
