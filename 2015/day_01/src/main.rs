#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.trim_end_matches('\n')
        .as_bytes()
        .iter()
        .fold(0, |acc, b| match b {
            b'(' => acc + 1,
            b')' => acc - 1,
            _ => unreachable!(),
        })
}

fn part_two(data: &str) -> usize {
    let mut acc = 0;
    for (position, character) in data.trim_end_matches('\n').as_bytes().iter().enumerate() {
        match character {
            b'(' => acc += 1,
            b')' => acc -= 1,
            _ => unreachable!(),
        }
        if acc == -1 {
            return position + 1;
        }
    }
    0
}
