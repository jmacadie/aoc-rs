#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::SplitWhitespace;

use itertools::{Chunk, Itertools};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines().map(possible_line).filter(|&v| v).count()
}

fn part_two(data: &str) -> usize {
    data.split_whitespace()
        .chunks(9)
        .into_iter()
        .map(possible_chunk_count)
        .sum()
}

fn possible_chunk_count(c: Chunk<SplitWhitespace>) -> usize {
    c.tuple_windows()
        .map(|(a, _, _, b, _, _, c)| (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()))
        .map(possible)
        .filter(|&v| v)
        .count()
}

fn possible_line(line: &str) -> bool {
    let mut nums = line.split_whitespace().map(|v| v.parse().unwrap());
    let (Some(a), Some(b), Some(c), None) = (nums.next(), nums.next(), nums.next(), nums.next()) else {
        unreachable!();
    };
    possible((a, b, c))
}

fn possible(dims: (u16, u16, u16)) -> bool {
    let mut max = std::cmp::max(dims.0, dims.1);
    max = std::cmp::max(max, dims.2);
    2 * max < dims.0 + dims.1 + dims.2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data));
        assert_eq!(0, part_one("1 1 2"));
        assert_eq!(0, part_one("1 1 3"));
        assert_eq!(1, part_one("1 1 1"));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_two(data));
    }
}
