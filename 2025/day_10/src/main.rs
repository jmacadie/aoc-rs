#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(int_roundings)]

pub mod light;
pub mod matrix;
pub mod utils;

use crate::light::Light;
use crate::matrix::Matrix;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(|l| l.parse::<Light>().unwrap())
        .map(|l| l.find_min_presses())
        .sum()
}

fn part_two(data: &str) -> i32 {
    data.lines()
        .map(|l| l.parse::<Matrix>().unwrap())
        .map(|m| m.min_sum())
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(7, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(33, part_two(data));
    }
}
