#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    data.lines().map(row_checksum).sum()
}

fn part_two(data: &str) -> u32 {
    data.lines().map(row_divisible_result).sum()
}

fn row_checksum(line: &str) -> u32 {
    let (min, max) = line.split_whitespace().map(|v| v.parse().unwrap()).fold(
        (u32::MAX, 0_u32),
        |(min, max), v| {
            let min = std::cmp::min(min, v);
            let max = std::cmp::max(max, v);
            (min, max)
        },
    );
    max - min
}

fn row_divisible_result(line: &str) -> u32 {
    let mut sorted = line
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .sorted_unstable();
    while let Some(v1) = sorted.next() {
        if let Some(v2) = sorted.clone().find(|v: &u32| *v % v1 == 0) {
            return v2 / v1;
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_row_checksum() {
        assert_eq!(8, row_checksum("5 1 9 5"));
        assert_eq!(4, row_checksum("7 5 3"));
        assert_eq!(6, row_checksum("2 4 6 8"));
    }

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(18, part_one(data));
    }

    #[test]
    fn test_row_divisible() {
        assert_eq!(4, row_divisible_result("5 9 2 8"));
        assert_eq!(3, row_divisible_result("9 4 7 3"));
        assert_eq!(2, row_divisible_result("3 8 6 5"));
    }
}
