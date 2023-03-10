#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<6, 50>(data));
    println!("Part 2: {}", part_two::<6, 50>(data));
}

fn part_one<const R: usize, const C: usize>(data: &str) -> u32 {
    let mut screen = Screen::<R, C>::new();
    for instruction in data.lines() {
        screen.step(instruction);
    }
    screen.count_on()
}

fn part_two<const R: usize, const C: usize>(data: &str) -> u32 {
    let mut screen = Screen::<R, C>::new();
    for instruction in data.lines() {
        screen.step(instruction);
    }
    println!("{screen}");
    0
}

struct Screen<const R: usize, const C: usize> {
    data: [u64; R],
}

impl<const R: usize, const C: usize> Screen<R, C> {
    const fn new() -> Self {
        Self { data: [0; R] }
    }

    fn step(&mut self, instruction: &str) {
        let mut parts = instruction.split(' ');
        let Some(instruction_type) = parts.next() else {
            unreachable!();
        };
        match instruction_type {
            "rect" => {
                let (cols, rows) = parts.next().unwrap().split_once('x').unwrap();
                let rows = rows.parse().unwrap();
                let cols = cols.parse().unwrap();
                self.rect(rows, cols);
            }
            "rotate" => {
                let Some(sub_type) = parts.next() else {
                    unreachable!();
                };
                match sub_type {
                    "row" => {
                        let (_, row) = parts.next().unwrap().split_at(2);
                        let row = row.parse().unwrap();

                        let by = parts.nth(1).unwrap().parse().unwrap();

                        self.rotate_row(row, by);
                    }
                    "column" => {
                        let (_, col) = parts.next().unwrap().split_at(2);
                        let col = col.parse().unwrap();

                        let by = parts.nth(1).unwrap().parse().unwrap();

                        self.rotate_col(col, by);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn rect(&mut self, rows: u8, cols: u8) {
        for row in self.data.iter_mut().take(rows.into()) {
            let mask = 2_u64.pow(cols.into()) - 1;
            *row |= mask;
        }
    }

    fn rotate_row(&mut self, row: u8, by: u8) {
        let full_width_mask = 2u64.pow(u32::try_from(C).unwrap()) - 1;
        let row = usize::try_from(row).unwrap();
        let orig = self.data[row];
        let overflow = orig >> (u8::try_from(C).unwrap() - by);
        let shifted = orig << by & full_width_mask;
        let new = shifted | overflow;
        self.data[row] = new;
    }

    fn rotate_col(&mut self, col: u8, by: u8) {
        let orig: [bool; R] = std::array::from_fn(|i| (self.data[i] >> col) & 1 == 1);
        let mask = 1_u64 << col;
        for (i, row) in self.data.iter_mut().enumerate() {
            let source_row = (R + i - usize::try_from(by).unwrap()) % R;
            if orig[source_row] {
                *row |= mask;
            } else {
                *row &= !mask;
            }
        }
    }

    fn count_on(&self) -> u32 {
        self.data.iter().map(|&row| row.count_ones()).sum()
    }
}

impl<const R: usize, const C: usize> Display for Screen<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            let mut temp = *row;
            for _ in 0..C {
                if temp & 1 == 1 {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
                temp >>= 1;
            }
            writeln!(f)?;
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
        assert_eq!(6, part_one::<3, 7>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two::<3, 7>(data));
    }
}
