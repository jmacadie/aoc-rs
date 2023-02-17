#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    //println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u64 {
    const MUL: u64 = 252_533;
    const MOD: u64 = 33_554_393;
    const BASE: u64 = 20_151_125;
    let (row, col) = read(data);
    let n = index(row, col);
    let mut out = square_and_multiply(MUL, n, MOD);
    out *= BASE;
    out %= MOD;
    out
}

const fn index(row: u32, col: u32) -> u32 {
    (row + col - 1) * (row + col) / 2 - row
}

fn square_and_multiply(multiplier: u64, n: u32, modulo: u64) -> u64 {
    let mut carry = 1;
    for bit in BitIterator::new(n) {
        carry *= carry;
        carry %= modulo;
        if bit {
            carry *= multiplier;
            carry %= modulo;
        }
    }
    carry
}

struct BitIterator {
    num: u32,
    index: u32,
}

impl BitIterator {
    const fn new(num: u32) -> Self {
        Self { num, index: 0 }
    }
}

impl Iterator for BitIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 32 {
            return None;
        }
        let bit = (self.num >> (31 - self.index)) & 1 == 1;
        self.index += 1;
        Some(bit)
    }
}

fn read(data: &str) -> (u32, u32) {
    let (_, rest) = data.split_once("row ").unwrap();
    let (row, rest) = rest.split_once(", column ").unwrap();
    // TODO: Assume linux line endings, would break on windows or mac
    let col = rest.strip_suffix(".\n").unwrap();
    (row.parse().unwrap(), col.parse().unwrap())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = "To continue, please consult the code grid in the manual.  Enter the code at row 3, column 3.";
        assert_eq!(1_601_130, part_one(data));
        let data = "To continue, please consult the code grid in the manual.  Enter the code at row 1, column 6.";
        assert_eq!(33_511_524, part_one(data));
        let data = "To continue, please consult the code grid in the manual.  Enter the code at row 5, column 2.";
        assert_eq!(17_552_253, part_one(data));
    }

    #[test]
    fn s_and_m() {
        assert_eq!(8, square_and_multiply(3, 10, 17));
    }
}
