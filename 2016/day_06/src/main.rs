#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<8>(data));
    println!("Part 2: {}", part_two::<8>(data));
}

fn part_one<const N: usize>(data: &str) -> String {
    (0..N)
        .into_iter()
        .map(|i| most_common::<N>(data, i))
        .collect::<String>()
}

fn part_two<const N: usize>(data: &str) -> String {
    (0..N)
        .into_iter()
        .map(|i| least_common::<N>(data, i))
        .collect::<String>()
}

// TODO: Lot of duplication, can we pass in a closure and only use one copy of this function?
fn most_common<const N: usize>(data: &str, offset: usize) -> char {
    data.chars()
        .skip(offset)
        .step_by(N + 1)
        .sorted_unstable()
        .dedup_with_count()
        .sorted_unstable_by_key(|&(c, _)| std::cmp::Reverse(c))
        .map(|(_, s)| s)
        .next()
        .unwrap()
}

fn least_common<const N: usize>(data: &str, offset: usize) -> char {
    data.chars()
        .skip(offset)
        .step_by(N + 1)
        .sorted_unstable()
        .dedup_with_count()
        .sorted_unstable_by_key(|&(c, _)| c)
        .map(|(_, s)| s)
        .next()
        .unwrap()
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!("easter", part_one::<6>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!("advent", part_two::<6>(data));
    }
}
