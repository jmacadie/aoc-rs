#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let mut grid = [[0_u128; 10]; 1000];
    for line in data.lines() {
        part_1_line(&mut grid, line);
    }
    count_bin_all(&grid)
}

fn part_two(data: &str) -> u32 {
    let mut grid = [[0_u8; 1000]; 1000];
    for line in data.lines() {
        part_2_line(&mut grid, line);
    }
    count_all(&grid)
}

type Point = [usize; 2];
type Grid = [[u128; 10]; 1000];
type BrightGrid = [[u8; 1_000]; 1_000];
type Square = [Point; 2];

const MAX_MASK: u128 = 2_u128.pow(100) - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SwitchType {
    On,
    Toggle,
    Off,
}

fn count_all(grid: &BrightGrid) -> u32 {
    grid.iter().flatten().map(|&v| Into::<u32>::into(v)).sum()
}

fn count_bin_all(grid: &Grid) -> u32 {
    grid.iter().flatten().map(|&v| count_bin_one(v)).sum()
}

fn count_bin_one(mut num: u128) -> u32 {
    let mut count = 0;
    for _ in 0..100 {
        count += (num & 1) as u32;
        num >>= 1;
    }
    count
}

fn part_2_line(grid: &mut BrightGrid, line: &str) {
    let (square, st) = read_line(line);
    part_2_switch(grid, square, st);
}

fn part_1_line(grid: &mut Grid, line: &str) {
    let (square, st) = read_line(line);
    switch(grid, square, st);
}

fn read_line(line: &str) -> (Square, SwitchType) {
    let (coords, st) = match (line.as_bytes()[1], line.as_bytes()[6]) {
        (b'u', b'n') => (line.trim_start_matches("turn on "), SwitchType::On),
        (b'u', b'f') => (line.trim_start_matches("turn off "), SwitchType::Off),
        (b'o', _) => (line.trim_start_matches("toggle "), SwitchType::Toggle),
        _ => unreachable!(),
    };
    (read_coords(coords), st)
}

fn read_coords(line: &str) -> Square {
    let (right, left) = line.trim().split_once(" through ").unwrap();
    [read_point(right), read_point(left)]
}

fn read_point(line: &str) -> Point {
    let (x, y) = line.trim().split_once(',').unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    [x, y]
}

fn part_2_switch(grid: &mut BrightGrid, square: Square, st: SwitchType) {
    for row in grid.iter_mut().take(square[1][1] + 1).skip(square[0][1]) {
        for elem in row.iter_mut().take(square[1][0] + 1).skip(square[0][0]) {
            match st {
                SwitchType::On => *elem += 1,
                SwitchType::Off => *elem = elem.saturating_sub(1),
                SwitchType::Toggle => *elem += 2,
            }
        }
    }
}

fn switch(grid: &mut Grid, square: Square, st: SwitchType) {
    let start_chunk = square[0][0] / 100;
    let end_chunk = square[1][0] / 100;
    let start = (square[0][0] - start_chunk * 100).try_into().unwrap();
    let end = (square[1][0] - end_chunk * 100).try_into().unwrap();
    let lm = left_mask(start);
    let rm = right_mask(end);
    let pm = part_mask(start, end);

    for item in grid.iter_mut().take(square[1][1] + 1).skip(square[0][1]) {
        for chunk in (start_chunk..=end_chunk).into_iter().with_position() {
            let (mask, elem) = match chunk {
                itertools::Position::First(v) => (lm, v),
                itertools::Position::Middle(v) => (MAX_MASK, v),
                itertools::Position::Last(v) => (rm, v),
                itertools::Position::Only(v) => (pm, v),
            };
            switch_elem(&mut item[elem], mask, st);
        }
    }
}

fn switch_elem(elem: &mut u128, mask: u128, st: SwitchType) {
    match st {
        SwitchType::On => *elem |= mask,
        SwitchType::Toggle => *elem ^= mask,
        SwitchType::Off => *elem &= !mask,
    }
}

fn right_mask(mut bits: u8) -> u128 {
    bits += 1;
    2_u128.pow(bits.into()) - 1
}

fn left_mask(bits: u8) -> u128 {
    !(right_mask(bits) >> 1) & MAX_MASK
}

fn part_mask(left: u8, right: u8) -> u128 {
    left_mask(left) & right_mask(right)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(998_996, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(1_001_996, part_two(data));
    }

    #[test]
    fn test_right_mask() {
        assert_eq!(right_mask(0), 1);
        assert_eq!(right_mask(1), 0b11);
        assert_eq!(right_mask(10), 0b111_1111_1111);
    }

    #[test]
    fn test_left_mask() {
        assert_eq!(left_mask(0), 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
        assert_eq!(left_mask(1), 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110);
        assert_eq!(left_mask(10), 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1100_0000_0000);
    }

    #[test]
    fn test_part_mask() {
        assert_eq!(part_mask(0, 1), 0b11);
        assert_eq!(part_mask(1, 10), 0b111_1111_1110);
        assert_eq!(part_mask(10, 20), 0b1_1111_1111_1100_0000_0000);
    }
}
