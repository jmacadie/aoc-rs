#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    data.lines().map(|l| Bank::new(l).max_joltage::<2>()).sum()
}

fn part_two(data: &'static str) -> usize {
    data.lines().map(|l| Bank::new(l).max_joltage::<12>()).sum()
}

struct Bank {
    data: &'static [u8],
}

impl Bank {
    const fn new(s: &'static str) -> Self {
        Self { data: s.as_bytes() }
    }

    fn max_joltage<const N: usize>(self) -> usize {
        let mut out = [0u8; N];
        for (i, d) in self.data.iter().map(|d| d - b'0').rev().enumerate().rev() {
            let start = N - (i + 1).min(N);
            let window = &mut out[start..];
            if let Some(pos) = window.iter().position(|&x| d > x) {
                window[pos] = d;
                window[pos + 1..].fill(0);
            }
        }
        out.iter().fold(0, |acc, &x| acc * 10 + x as usize)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(357, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(3_121_910_778_619, part_two(data));
    }
}
