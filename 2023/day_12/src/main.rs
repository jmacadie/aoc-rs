#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(ConditionRecord::new)
        .map(|mut r| r.matches())
        .sum()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .map(ConditionRecord::new_folded)
        .map(|mut r| r.matches())
        .sum()
}

#[derive(Debug)]
struct ConditionRecord {
    springs: Vec<Status>,
    groups: Vec<usize>,
    memo: HashMap<Key, usize>,
}

impl ConditionRecord {
    fn new(data: &str) -> Self {
        let mut springs = Vec::new();
        let mut groups = Vec::new();
        let (s, g) = data.split_once(' ').unwrap();

        springs.extend(
            s.chars()
                .map(Status::from_char)
                .chain(std::iter::once(Status::Unknown)),
        );
        groups.extend(g.split(',').map(|x| x.parse::<usize>().unwrap()));

        Self {
            springs,
            groups,
            memo: HashMap::new(),
        }
    }

    fn new_folded(data: &str) -> Self {
        let mut springs = Vec::new();
        let mut groups = Vec::new();
        let (s, g) = data.split_once(' ').unwrap();

        springs.extend(
            s.chars()
                .map(Status::from_char)
                .chain(std::iter::once(Status::Unknown))
                .cycle()
                .take(5 * (s.len() + 1)),
        );
        let g = g.split(',').map(|x| x.parse::<usize>().unwrap());
        let groups_len = 5 * g.clone().count();
        groups.extend(g.cycle().take(groups_len));

        Self {
            springs,
            groups,
            memo: HashMap::new(),
        }
    }

    fn matches(&mut self) -> usize {
        let springs = self.springs.clone();
        let groups = self.groups.clone();
        self.matches_inner(&springs, &groups)
    }

    fn matches_inner(&mut self, springs: &[Status], groups: &[usize]) -> usize {
        let key = Self::generate_key(springs, groups);
        if let Some(val) = self.memo.get(&key) {
            return *val;
        }
        if groups.is_empty() {
            if springs.iter().any(|&x| x == Status::Damaged) {
                return self.store(key, 0);
            }
            return self.store(key, 1);
        }
        let group = groups[0] + 1;
        let next = springs.windows(group).position(|test| {
            let (end, body) = test.split_last().unwrap();
            body.iter().all(|&st| st != Status::Operational) && (*end != Status::Damaged)
        });
        if next.is_none() {
            return self.store(key, 0);
        }
        let offset = next.unwrap();
        if springs.iter().take(offset).any(|&s| s == Status::Damaged) {
            return self.store(key, 0);
        }
        let val = self.matches_inner(&springs[offset + group..], &groups[1..])
            + if springs[offset] == Status::Unknown {
                self.matches_inner(&springs[offset + 1..], groups)
            } else {
                0
            };
        self.store(key, val)
    }

    fn generate_key(springs: &[Status], groups: &[usize]) -> Key {
        let mut key = Vec::with_capacity(springs.len() + groups.len() + 1);
        springs.iter().for_each(|&s| match s {
            Status::Unknown => key.push(1),
            Status::Operational => key.push(2),
            Status::Damaged => key.push(3),
        });
        key.push(0);
        groups
            .iter()
            .for_each(|&g| key.push(u8::try_from(g).unwrap()));
        key
    }

    fn store(&mut self, key: Key, val: usize) -> usize {
        self.memo.insert(key, val);
        val
    }
}

type Key = Vec<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Status {
    fn from_char(s: char) -> Self {
        match s {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(21, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(525_152, part_two(data));
    }
}
