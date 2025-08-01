#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code, clippy::missing_panics_doc)]

use std::{cmp, num::ParseIntError, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    let (rules, pages) = data.split_once("\n\n").unwrap();
    let r = Rules::new(rules);

    println!("Part 1: {}", part_one(&r, pages));
    println!("Part 2: {}", part_two(&r, pages));
}

fn part_one(r: &Rules, pages: &str) -> usize {
    pages
        .lines()
        .map(|l| l.parse::<Layout>().unwrap())
        .filter(|l| r.is_correct_order(l))
        .map(|l| l.middle())
        .sum()
}

fn part_two(r: &Rules, pages: &str) -> usize {
    pages
        .lines()
        .map(|l| l.parse::<Layout>().unwrap())
        .filter(|l| !r.is_correct_order(l))
        .map(|l| r.ordered_middle(&l))
        .sum()
}

struct Layout {
    pages: Vec<usize>,
}

impl FromStr for Layout {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pages = Vec::new();
        for p in s.split(',') {
            let x = p.parse::<usize>()?;
            pages.push(x);
        }
        Ok(Self { pages })
    }
}

impl Layout {
    fn middle(&self) -> usize {
        if self.pages.is_empty() {
            return 0;
        }
        self.pages[self.pages.len() / 2]
    }

    fn iter(&self) -> impl Iterator<Item = &usize> {
        self.pages.iter()
    }
}

#[derive(Debug)]
struct Rules {
    order: Vec<usize>,
}

impl Rules {
    fn new(input: &'static str) -> Self {
        let pages = Self::create_pages_rules(input);
        let order = Self::find_sort_order(&pages);
        Self { order }
    }

    fn create_pages_rules(input: &'static str) -> [Page; 100] {
        let mut pages = std::array::from_fn(|_| Page::new());

        // Process each line
        input.lines().for_each(|l| {
            let (b, a) = l.split_once('|').unwrap();
            let before = b.parse::<usize>().unwrap();
            let after = a.parse::<u8>().unwrap();
            let page: &mut Page = &mut pages[before];
            page.add_after(after);
        });

        // Sort the array, for faster searching later
        for page in pages.iter_mut().filter(|p| p.has_data()) {
            page.after[..page.count].sort_unstable();
        }

        pages
    }

    fn find_sort_order(pages: &[Page]) -> Vec<usize> {
        let mut first_half = Vec::with_capacity(25);
        let mut second_half = Vec::with_capacity(24);
        let start_page = pages.iter().position(Page::has_data).unwrap();
        let mut current = start_page;
        loop {
            first_half.push(current);
            let (first, second) = Self::find_next_page(current, pages);
            if second == start_page {
                break;
            }
            second_half.push(second);
            current = first;
        }

        first_half.extend_from_slice(&second_half[..]);
        first_half
    }

    fn find_next_page(start: usize, pages: &[Page]) -> (usize, usize) {
        let this_page = &pages[start];
        this_page
            .after
            .iter()
            .map(|&idx| {
                let idxu = usize::from(idx);
                (idxu, this_page.one_diff(&pages[idxu]))
            })
            .find(|(_, x)| *x > 0)
            .unwrap()
    }

    fn is_correct_order(&self, layout: &Layout) -> bool {
        let mut test_pages = layout.iter();
        let first_page = test_pages.next().unwrap();
        let mut search_pages = self
            .order
            .iter()
            .cycle()
            .skip_while(|p| *p != first_page)
            .take(25);

        for test in test_pages {
            if !search_pages.any(|&p| p == *test) {
                return false;
            }
        }

        true
    }

    fn ordered_middle(&self, layout: &Layout) -> usize {
        let mut order: Vec<usize> = layout
            .iter()
            .map(|p| self.order.iter().position(|o| *o == *p).unwrap())
            .collect();
        order.sort_unstable();
        let middle = order.len() / 2;
        for (idx, x) in order.windows(2).enumerate() {
            // If gap is bigger than 24 then we will need to rotate
            if x[1] - x[0] > 24 {
                let mod_middle = (middle + idx + 1) % order.len();
                return self.order[order[mod_middle]];
            }
        }
        self.order[order[middle]]
    }
}

#[derive(Debug)]
struct Page {
    after: [u8; 24],
    count: usize,
}

impl Page {
    const fn new() -> Self {
        Self {
            after: [0; 24],
            count: 0,
        }
    }

    const fn add_after(&mut self, page: u8) {
        self.after[self.count] = page;
        self.count += 1;
    }

    const fn has_data(&self) -> bool {
        self.count > 0
    }

    fn one_diff(&self, other: &Self) -> usize {
        let mut diffs = (0, 0);
        let mut idx_self = 0;
        let mut idx_other = 0;
        let mut result = 0;
        while idx_self < self.count && idx_other < other.count {
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
