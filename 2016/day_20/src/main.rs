#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let mut limit: IpAddress = 0.into();
    for range in data
        .lines()
        .map(BlockedIpRange::new)
        .sorted_unstable_by_key(|range| range.from)
    {
        if range.from.0 > limit.0.saturating_add(1) {
            return limit.0 + 1;
        }
        if range.to > limit {
            limit = range.to;
        }
    }
    0
}

fn part_two(data: &str) -> u32 {
    let mut limit: IpAddress = 0.into();
    let mut allowed_count = 0;
    for range in data
        .lines()
        .map(BlockedIpRange::new)
        .sorted_unstable_by_key(|range| range.from)
    {
        if range.from.0 > limit.0.saturating_add(1) {
            allowed_count += range.from.0 - limit.0 - 1;
            limit = range.to;
        } else if range.to > limit {
            limit = range.to;
        }
    }
    allowed_count
}

#[derive(Debug)]
struct BlockedIpRange {
    from: IpAddress,
    to: IpAddress,
}

impl BlockedIpRange {
    fn new(line: &str) -> Self {
        let (from, to) = line.split_once('-').unwrap();
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        Self { from, to }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct IpAddress(u32);

impl From<u32> for IpAddress {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl FromStr for IpAddress {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u32 = s.parse()?;
        Ok(value.into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
