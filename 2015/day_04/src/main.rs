#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use md5::Digest;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let prefix = data.trim_end_matches('\n');
    for i in 0.. {
        let input = format!("{prefix}{i}");
        if five_leading_zeros(md5::Md5::digest(input)) {
            return i;
        }
    }
    0
}

fn part_two(data: &str) -> usize {
    let prefix = data.trim_end_matches('\n');
    for i in 0.. {
        let input = format!("{prefix}{i}");
        if six_leading_zeros(md5::Md5::digest(input)) {
            return i;
        }
    }
    0
}

fn five_leading_zeros(digest: impl AsRef<[u8]>) -> bool {
    digest.as_ref()[..2] == [0, 0] && digest.as_ref()[2] < 16
}

fn six_leading_zeros(digest: impl AsRef<[u8]>) -> bool {
    digest.as_ref()[..3] == [0, 0, 0]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(609_043, part_one("abcdef"));
        assert_eq!(1_048_970, part_one("pqrstuv"));
    }
}
