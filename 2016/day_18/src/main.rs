#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data, 40));
    println!("Part 2: {}", part_two(data, 400_000));
}

fn part_one(data: &str, rows: u32) -> u32 {
    count_safe_rows(data, rows)
}

fn part_two(data: &str, rows: u32) -> u32 {
    count_safe_rows(data, rows)
}

fn count_safe_rows(data: &str, rows: u32) -> u32 {
    let mut row = Row::new(data);
    let mut safe = 0;

    for _ in 0..rows {
        safe += row.count_safe();
        row = row.next();
    }
    safe
}

struct Row {
    data: u128,
    mask: u128,
    len: u32,
}

impl Row {
    fn new(input: &str) -> Self {
        let input = input.trim();
        let mut data = 0;
        let mut mask = 0;
        for element in input.as_bytes().iter().rev() {
            data <<= 1;
            mask = (mask << 1) | 1;
            match element {
                b'^' => data |= 1,
                b'.' => (),
                _ => unreachable!(),
            }
        }
        let len = u32::try_from(input.len()).unwrap();
        Self { data, mask, len }
    }

    const fn next(&self) -> Self {
        let left = self.data >> 1;
        let right = self.data << 1;
        let data = (left ^ right) & self.mask;

        Self {
            data,
            mask: self.mask,
            len: self.len,
        }
    }

    const fn count_safe(&self) -> u32 {
        self.len - self.data.count_ones()
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.data;
        for _ in 0..self.len {
            if temp & 1 == 1 {
                write!(f, "^")?;
            } else {
                write!(f, ".")?;
            }
            temp >>= 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(6, part_one("..^^.", 3));
        assert_eq!(38, part_one(".^^.^.^^^^", 10));
    }
}
