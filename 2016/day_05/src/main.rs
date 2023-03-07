#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use md5::{Digest, Md5};

pub fn main() {
    let data = include_str!("input.txt")
        .trim_end_matches('\n')
        .trim_end_matches('\r');
    let hashes = find_hashes(data);
    println!("Part 1: {}", part_one(hashes));
    println!("Part 2: {}", part_two(hashes));
}

fn part_one(data: Hashes) -> String {
    data.into_iter()
        .take(8)
        .map(|(a, _)| as_hex_char(a))
        .collect::<String>()
}

fn part_two(data: Hashes) -> String {
    data.into_iter()
        .sorted_by_key(|&(a, _)| a)
        .dedup_by(|x, y| x.0 == y.0)
        .take(8)
        .map(|(_, b)| as_hex_char(b))
        .collect::<String>()
}

type Hashes = [(u8, u8); 64];

fn find_hashes(prefix: &str) -> Hashes {
    let mut out = [(0, 0); 64];
    let mut hasher = Md5::new();
    let mut index = 0;
    let mut found = [false; 8];

    for i in 0.. {
        hasher.update(format!("{prefix}{i}"));
        let digest = hasher.finalize_reset();
        if five_leading_zeros(&digest) {
            let sixth = get_hex_num(&digest, 5);
            let seventh = get_hex_num(&digest, 6);
            if sixth < 8 {
                found[usize::try_from(sixth).unwrap()] = true;
            }
            out[index] = (sixth, seventh);
            if !found.iter().any(|&x| !x) {
                return out;
            }
            index += 1;
        }
    }
    out
}

fn five_leading_zeros(digest: &impl AsRef<[u8]>) -> bool {
    digest.as_ref()[..2] == [0, 0] && digest.as_ref()[2] < 16
}

fn get_hex_num(digest: &impl AsRef<[u8]>, position: usize) -> u8 {
    let byte = digest.as_ref()[position / 2];
    if position & 1 == 1 {
        byte & 0b0000_1111
    } else {
        byte >> 4
    }
}

fn as_hex_char(number: u8) -> char {
    let ascii = match number {
        x if x > 15 => unreachable!(),
        x if x < 10 => b'0' + x,
        x => b'a' + x - 10,
    };
    char::from_u32(u32::from(ascii)).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let hashes = find_hashes("abc");
        assert_eq!("18f47a30", part_one(hashes));
    }

    #[test]
    fn two() {
        let hashes = find_hashes("abc");
        assert_eq!("05ace8e3", part_two(hashes));
    }
}
