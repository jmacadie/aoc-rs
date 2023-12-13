#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::usize;

use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    let s = Sky::new(data);
    println!("Part 1: {}", part_one(&s));
    println!("Part 2: {}", part_two(&s, 999_999));
}

fn part_one(s: &Sky) -> usize {
    s.distances(1)
}

fn part_two(s: &Sky, expand_by: usize) -> usize {
    s.distances(expand_by)
}

#[derive(Debug)]
struct Sky {
    galaxies: Vec<Point>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Sky {
    fn new(data: &str) -> Self {
        let mut galaxies = Vec::new();
        let mut empty_rows = Vec::new();
        let mut empty_cols = Vec::new();

        let row_length = data.lines().nth(0).unwrap().len();
        data.lines()
            .enumerate()
            .filter(|&(_, row)| !row.contains('#'))
            .for_each(|(i, _)| empty_rows.push(i));
        (0..row_length)
            .filter(|&col| {
                !data
                    .as_bytes()
                    .iter()
                    .skip(col)
                    .step_by(row_length + 1)
                    .any(|&x| x == b'#')
            })
            .for_each(|col| empty_cols.push(col));
        data.lines().enumerate().for_each(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == b'#')
                .for_each(|(col, _)| {
                    galaxies.push((row, col));
                });
        });
        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    fn distances(&self, expand_by: usize) -> usize {
        let expanded = self
            .galaxies
            .iter()
            .map(|&(row, col)| {
                let shifted_row = row
                    + expand_by
                        * self
                            .empty_rows
                            .iter()
                            .position(|&x| x > row)
                            .unwrap_or(self.empty_rows.len());
                let shifted_col = col
                    + expand_by
                        * self
                            .empty_cols
                            .iter()
                            .position(|&x| x > col)
                            .unwrap_or(self.empty_cols.len());
                (shifted_row, shifted_col)
            })
            .collect_vec();
        expanded
            .iter()
            .tuple_combinations()
            .map(|(&a, &b)| manhatten_distance(a, b))
            .sum()
    }
}

const fn manhatten_distance(a: Point, b: Point) -> usize {
    let dist = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    if a.1 > b.1 {
        dist + a.1 - b.1
    } else {
        dist + b.1 - a.1
    }
}

type Point = (usize, usize);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let s = Sky::new(data);
        assert_eq!(374, part_one(&s));
    }

    #[test]
    fn two_a() {
        let data = include_str!("test.txt");
        let s = Sky::new(data);
        assert_eq!(1030, part_two(&s, 9));
    }

    #[test]
    fn two_b() {
        let data = include_str!("test.txt");
        let s = Sky::new(data);
        assert_eq!(8410, part_two(&s, 99));
    }
}
