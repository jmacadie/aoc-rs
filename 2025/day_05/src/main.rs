#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn main() {
    let data = include_str!("input.txt");
    let (ranges, ingredients) = data.split_once("\n\n").unwrap();
    let r = Ranges::new(ranges);

    println!("Part 1: {}", part_one(&r, ingredients));
    println!("Part 2: {}", part_two(&r));
}

fn part_one(ranges: &Ranges, ingredients: &str) -> usize {
    ingredients
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(|val| ranges.depth(val))
        .filter(|&d| d > 0)
        .count()
}

fn part_two(ranges: &Ranges) -> usize {
    ranges.good_values()
}

struct Ranges {
    data: Vec<(usize, usize)>,
}

impl Ranges {
    fn new(input: &str) -> Self {
        let mut ranges = Self::input_to_tuples(input).into_iter().peekable();
        let mut ends = BinaryHeap::<Reverse<usize>>::new();
        let mut data = vec![(0, 0)];
        let mut depth = 0usize;

        loop {
            match (ranges.peek(), ends.peek()) {
                (Some(&(start, end)), None) => {
                    Self::open_range(&mut data, &mut ends, &mut depth, start, end);
                    ranges.next();
                }
                (Some(&(start, end)), Some(&Reverse(min_end))) if start < min_end => {
                    Self::open_range(&mut data, &mut ends, &mut depth, start, end);
                    ranges.next();
                }
                (None, Some(&Reverse(min_end))) => {
                    Self::close_ranges(&mut data, &mut ends, &mut depth, min_end);
                }
                (Some(&(start, _)), Some(&Reverse(min_end))) if min_end < start => {
                    Self::close_ranges(&mut data, &mut ends, &mut depth, min_end);
                }
                (Some(&(start, end)), Some(&Reverse(min_end))) => {
                    debug_assert_eq!(start, min_end);
                    Self::replace_close_with_open(&mut ends, &mut depth, min_end, end);
                    ranges.next();
                }
                (None, None) => break,
            }
        }

        Self { data }
    }

    fn good_values(&self) -> usize {
        self.data
            .iter()
            .fold((0, 0), |(count, last), &(val, depth)| {
                if depth == 0 {
                    (count + val - last, 0)
                } else if last == 0 {
                    (count, val)
                } else {
                    (count, last)
                }
            })
            .0
    }
    fn depth(&self, val: usize) -> usize {
        let posn = self
            .data
            .binary_search_by_key(&val, |&(s, _)| s)
            .map_err(|pos| pos - 1)
            .unwrap_or_else(|e| e);
        self.data[posn].1
    }

    fn open_range(
        data: &mut Vec<(usize, usize)>,
        ends: &mut BinaryHeap<Reverse<usize>>,
        depth: &mut usize,
        start: usize,
        end: usize,
    ) {
        *depth += 1;
        Self::add_depth_change(data, start, *depth);
        ends.push(Reverse(end + 1));
    }

    fn close_ranges(
        data: &mut Vec<(usize, usize)>,
        ends: &mut BinaryHeap<Reverse<usize>>,
        depth: &mut usize,
        end: usize,
    ) {
        let removed = Self::pop_matching_ends(ends, end);
        *depth -= removed;
        Self::add_depth_change(data, end, *depth);
    }

    fn replace_close_with_open(
        ends: &mut BinaryHeap<Reverse<usize>>,
        depth: &mut usize,
        old_end: usize,
        new_end: usize,
    ) {
        let removed = Self::pop_matching_ends(ends, old_end);
        *depth += 1;
        *depth -= removed;

        ends.push(Reverse(new_end + 1));
    }

    fn pop_matching_ends(ends: &mut BinaryHeap<Reverse<usize>>, end_pos: usize) -> usize {
        let mut removed = 0;

        while let Some(&Reverse(pos)) = ends.peek() {
            if pos != end_pos {
                break;
            }
            ends.pop();
            removed += 1;
        }

        removed
    }

    fn add_depth_change(data: &mut Vec<(usize, usize)>, val: usize, depth: usize) {
        while data.last().is_some_and(|&(p, _)| p == val) {
            data.pop();
        }
        data.push((val, depth));
    }

    fn input_to_tuples(input: &str) -> Vec<(usize, usize)> {
        let mut data: Vec<_> = input
            .lines()
            .map(|line| {
                let Some((start, end)) = line.split_once('-') else {
                    panic!("Cannot split {line} into two numbers");
                };
                let start = start.parse::<usize>().expect("Can parse into a number");
                let end = end.parse::<usize>().expect("Can parse into a number");
                (start, end)
            })
            .collect();

        data.sort_unstable_by_key(|&(start, _)| start);
        data
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (ranges, ingredients) = data.split_once("\n\n").unwrap();
        let r = Ranges::new(ranges);
        assert_eq!(3, part_one(&r, ingredients));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (ranges, _) = data.split_once("\n\n").unwrap();
        let r = Ranges::new(ranges);
        assert_eq!(14, part_two(&r));
    }
}
