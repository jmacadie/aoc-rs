#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use std::{cmp, fmt::Display};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    let (rules, _pages) = data.split_once("\n\n").unwrap();
    let mut r = Rules::new();
    r.add_rules(rules);
    println!("{r}");
    let x = r.find_sort_order();
    println!("{x:?}");
    0
}

const fn part_two(_data: &str) -> usize {
    0
}

#[derive(Debug)]
struct Rules {
    pages: [Page; 100],
}

impl Rules {
    fn new() -> Self {
        let pages = std::array::from_fn(|_| Page::new());
        Self { pages }
    }

    fn add_rules(&mut self, input: &'static str) {
        // Process each line
        input.lines().for_each(|l| {
            let (b, a) = l.split_once('|').unwrap();
            let before = b.parse::<u8>().unwrap();
            let after = a.parse::<u8>().unwrap();
            let b_page: &mut Page = &mut self.pages[usize::from(before)];
            b_page.add_after(after);
            let a_page: &mut Page = &mut self.pages[usize::from(after)];
            a_page.add_before(before);
        });

        // Sort the arrays, for faster searching later
        for p in self
            .pages
            .iter_mut()
            .filter(|p| p.before_count > 0 || p.after_count > 0)
        {
            p.before[..p.before_count].sort_unstable();
            p.after[..p.after_count].sort_unstable();
        }
    }

    fn find_sort_order(&self) -> Vec<usize> {
        let mut first_half = Vec::with_capacity(25);
        let mut second_half = Vec::with_capacity(24);
        let start_page = self
            .pages
            .iter()
            .position(|p| p.before_count > 0 || p.after_count > 0)
            .unwrap();
        let mut current = start_page;
        loop {
            first_half.push(current);
            let (first, second) = self.find_next_page(current);
            if second == start_page {
                break;
            }
            second_half.push(second);
            current = first;
        }

        first_half.extend_from_slice(&second_half[..]);
        first_half
    }

    fn find_next_page(&self, start: usize) -> (usize, usize) {
        let this_page = &self.pages[start];
        this_page
            .after
            .iter()
            .map(|&idx| {
                let idxu = usize::from(idx);
                (idxu, this_page.one_diff(&self.pages[idxu]))
            })
            .find(|(_, x)| *x > 0)
            .unwrap()
    }
}

impl Display for Rules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, p) in self
            .pages
            .iter()
            .enumerate()
            .filter(|(_, p)| p.before_count > 0 || p.after_count > 0)
        {
            writeln!(f, "{i}: {p}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Page {
    before: [u8; 24],
    after: [u8; 24],
    before_count: usize,
    after_count: usize,
}

impl Page {
    const fn new() -> Self {
        Self {
            before: [0; 24],
            after: [0; 24],
            before_count: 0,
            after_count: 0,
        }
    }

    const fn add_before(&mut self, page: u8) {
        self.before[self.before_count] = page;
        self.before_count += 1;
    }

    const fn add_after(&mut self, page: u8) {
        self.after[self.after_count] = page;
        self.after_count += 1;
    }

    fn one_diff(&self, other: &Self) -> usize {
        let mut diffs = (0, 0);
        let mut idx_self = 0;
        let mut idx_other = 0;
        let mut result = 0;
        while idx_self < self.after_count && idx_other < other.after_count {
            match (self.after[idx_self].cmp(&other.after[idx_other]), diffs) {
                (cmp::Ordering::Equal, _) => {
                    idx_self += 1;
                    idx_other += 1;
                }
                (cmp::Ordering::Less, (0, 0 | 1)) => {
                    idx_self += 1;
                    diffs.0 += 1;
                }
                (cmp::Ordering::Greater, (0 | 1, 0)) => {
                    result = other.after[idx_other].into();
                    idx_other += 1;
                    diffs.1 += 1;
                }
                _ => {
                    return 0;
                }
            }
        }
        result
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "before: ")?;
        for p in self.before.iter().filter(|p| **p > 0) {
            write!(f, "{p}, ")?;
        }
        write!(f, "after: ")?;
        for p in self.after.iter().filter(|p| **p > 0) {
            write!(f, "{p}, ")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_ne!(0, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
