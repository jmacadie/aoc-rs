#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

use day_2017_10::KnotHash;

pub fn main() {
    let data = include_str!("input.txt");
    let used = gen_used_hash(data);
    println!("Part 1: {}", part_one(&used));
    println!("Part 2: {}", part_two(used));
}

fn part_one(used: &HashSet<(u8, u8)>) -> usize {
    /* let data = data.trim();
    (0..128)
        .map(|i| KnotHash::new(&format!("{data}-{i}")))
        .map(|k| k.to_u128().count_ones())
        .sum() */
    used.len()
}

fn part_two(mut used: HashSet<(u8, u8)>) -> usize {
    let mut count = 0;
    while !used.is_empty() {
        if let Some(cell) = used.iter().next().cloned() {
            used.remove(&cell);
            find_all_neighbours(cell, &mut used);
            count += 1;
        }
    }
    count
}

fn gen_used_hash(data: &str) -> HashSet<(u8, u8)> {
    let data = data.trim();
    let mut used = HashSet::with_capacity(10_000);
    for (row, mut val) in (0..128)
        .map(|i| (i, KnotHash::new(&format!("{data}-{i}"))))
        .map(|(i, k)| (i, k.to_u128()))
    {
        for col in 0..128 {
            if val & 1 == 1 {
                used.insert((row, col));
            }
            val >>= 1;
        }
    }
    used
}

fn find_all_neighbours(from: (u8, u8), list: &mut HashSet<(u8, u8)>) {
    for n in neighbours(from)
        .into_iter()
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
    {
        if list.remove(&n) {
            find_all_neighbours(n, list);
        }
    }
}

fn neighbours(from: (u8, u8)) -> [Option<(u8, u8)>; 4] {
    match from {
        (0, 0) => [Some((1, 0)), Some((0, 1)), None, None],
        (_, 0) => [
            Some((from.0 + 1, 0)),
            Some((from.0 - 1, 0)),
            Some((from.0, 1)),
            None,
        ],
        (0, _) => [
            Some((0, from.1 + 1)),
            Some((0, from.1 - 1)),
            Some((1, from.1)),
            None,
        ],
        (_, _) => [
            Some((from.0 + 1, from.1)),
            Some((from.0 - 1, from.1)),
            Some((from.0, from.1 + 1)),
            Some((from.0, from.1 - 1)),
        ],
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let used = gen_used_hash(data);
        assert_eq!(8108, part_one(&used));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let used = gen_used_hash(data);
        assert_eq!(1242, part_two(used));
    }
}
