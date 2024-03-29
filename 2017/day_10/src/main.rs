#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<256>(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one<const N: usize>(data: &str) -> usize {
    let folds = data
        .trim_end()
        .split(',')
        .map(|d| str::parse::<usize>(d).unwrap())
        .collect::<Vec<usize>>();

    let folds_count = folds.len();
    let mut posn: usize = folds.iter().sum();
    posn += (folds_count * (folds_count - 1)) / 2;
    posn %= N;

    let mut first = 0;
    let mut second = 1;
    let mut skip = folds_count - 1;
    for jump in folds.iter().rev() {
        (posn, skip) = move_position::<N>(*jump, posn, skip);
        first = fold_array::<N>(first, posn, *jump);
        second = fold_array::<N>(second, posn, *jump);
    }
    first * second
}

fn part_two(data: &str) -> String {
    let knot = KnotHash::new(data);
    format!("{knot}")
}

fn fold_array<const N: usize>(mut x: usize, start: usize, length: usize) -> usize {
    if x < start {
        x += N;
    }
    let end = start + length;
    if x >= end {
        return x % N;
    }
    (start + end - x - 1) % N
}

fn move_position<const N: usize>(
    jump: usize,
    mut position: usize,
    mut skip: usize,
) -> (usize, usize) {
    position += 2 * N;
    position -= jump + skip;
    position %= N;
    skip = skip.saturating_sub(1);
    (position, skip)
}

const KNOT_SIZE: usize = 256;

#[derive(Debug)]
pub struct KnotHash {
    data: [u8; KNOT_SIZE],
    position: usize,
    skip: usize,
}

impl KnotHash {
    #[must_use]
    pub fn new(input: &str) -> Self {
        let ascii = input.trim().as_bytes();
        let data = std::array::from_fn(|i| u8::try_from(i).unwrap());
        let mut knot = Self {
            data,
            position: 0,
            skip: 0,
        };

        for _ in 0..64 {
            for ch in ascii {
                knot.fold(usize::from(*ch));
            }
            for ch in [17, 31, 73, 47, 23] {
                knot.fold(ch);
            }
        }

        knot
    }

    fn fold(&mut self, length: usize) {
        let end = self.position + length - 1;
        let middle_end = (self.position + end - 1) / 2; // TODO: will break if self.position is 0 and
                                                        // length is 1
        let middle_start = middle_end + if length & 1 == 0 { 1 } else { 2 };

        for (i, j) in (self.position..=middle_end).zip((middle_start..=end).rev()) {
            self.swap(i, j);
        }

        self.position += length + self.skip;
        self.position %= KNOT_SIZE;
        self.skip += 1;
    }

    fn swap(&mut self, mut a: usize, mut b: usize) {
        if a >= KNOT_SIZE {
            a %= KNOT_SIZE;
        }
        if b >= KNOT_SIZE {
            b %= KNOT_SIZE;
        }
        self.data.swap(a, b);
    }

    #[allow(dead_code)]
    fn first_two(&self) -> usize {
        usize::from(self.data[0]) * usize::from(self.data[1])
    }

    fn to_dense_hash(&self) -> [u8; 16] {
        std::array::from_fn(|i| {
            self.data[i * 16..(i + 1) * 16]
                .iter()
                .copied()
                .reduce(|acc, e| acc ^ e)
                .unwrap()
        })
    }

    #[must_use]
    pub fn to_u128(&self) -> u128 {
        let mut out: u128 = 0;
        for val in self.to_dense_hash() {
            out <<= 8;
            out |= u128::from(val);
        }
        out
    }
}

impl Display for KnotHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for num in self.to_dense_hash() {
            write!(f, "{num:02x}")?;
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
        assert_eq!(12, part_one::<5>(data));
    }

    #[test]
    fn two() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", part_two(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", part_two("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", part_two("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", part_two("1,2,4"));
    }
}
