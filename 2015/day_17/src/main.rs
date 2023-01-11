#![warn(clippy::all, clippy::pedantic)]
use itertools::Itertools;
use std::cmp::Ordering;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<20>(data, 150));
    println!("Part 2: {}", part_two::<20>(data, 150));
}

fn part_one<const N: usize>(data: &str, target: u8) -> u32 {
    let mut values = [0_u8; N];
    for (val, inp) in values.iter_mut().zip(data.lines()) {
        *val = inp.parse().unwrap();
    }
    count_combs(&values, target)
}

fn part_two<const N: usize>(data: &str, target: u8) -> u32 {
    let mut values = [0_u8; N];
    for (val, inp) in values.iter_mut().zip(data.lines()) {
        *val = inp.parse().unwrap();
    }
    let (count, _) = count_least_depth_combs(&values, target, 0);
    count
}

fn count_combs(data: &[u8], target: u8) -> u32 {
    data.iter()
        .enumerate()
        .map(|(i, elem)| match elem.cmp(&target) {
            Ordering::Greater => 0,
            Ordering::Equal => 1,
            Ordering::Less => count_combs(&data[i + 1..], target - elem),
        })
        .sum()
}

fn count_least_depth_combs(data: &[u8], target: u8, depth: u8) -> (u32, u8) {
    data.iter()
        .enumerate()
        .map(|(i, elem)| match elem.cmp(&target) {
            Ordering::Greater => (0, u8::MAX),
            Ordering::Equal => (1, depth + 1),
            Ordering::Less => count_least_depth_combs(&data[i + 1..], target - elem, depth + 1),
        })
        .min_set_by_key(|&(_, d)| d)
        .iter()
        .fold((0, u8::MAX), |(acc, _), &(c, d)| (acc + c, d))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(4, part_one::<5>(data, 25));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(3, part_two::<5>(data, 25));
    }
}
