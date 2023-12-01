#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    data.lines()
        .map(|l| calibration(l.as_bytes(), 1).unwrap())
        .sum()
}

fn part_two(data: &str) -> u32 {
    data.lines()
        .map(|l| calibration(l.as_bytes(), 2).unwrap())
        .sum()
}

fn calibration(line: &[u8], part: u8) -> Option<u32> {
    Some(first_num(line, part)? * 10 + last_num(line, part)?)
}

fn first_num(line: &[u8], part: u8) -> Option<u32> {
    if part == 1 {
        find_num(line.iter())
    } else {
        find(line)
    }
}

fn last_num(line: &[u8], part: u8) -> Option<u32> {
    if part == 1 {
        find_num(line.iter().rev())
    } else {
        find_rev(line)
    }
}

fn find_num<'a>(mut line: impl Iterator<Item = &'a u8>) -> Option<u32> {
    line.find(|&&ch| ch.is_ascii_digit())
        .map(|&ch| (ch - b'0').into())
}

const NUM_AS_BYTES: [&[u8]; 10] = [
    &[b'z', b'e', b'r', b'o'],
    &[b'o', b'n', b'e'],
    &[b't', b'w', b'o'],
    &[b't', b'h', b'r', b'e', b'e'],
    &[b'f', b'o', b'u', b'r'],
    &[b'f', b'i', b'v', b'e'],
    &[b's', b'i', b'x'],
    &[b's', b'e', b'v', b'e', b'n'],
    &[b'e', b'i', b'g', b'h', b't'],
    &[b'n', b'i', b'n', b'e'],
];

fn find(line: &[u8]) -> Option<u32> {
    for idx in 0..line.len() {
        let byte = line[idx];
        if byte.is_ascii_digit() {
            return Some((byte - b'0').into());
        }
        for (val, num_array) in NUM_AS_BYTES.iter().enumerate() {
            if line.len() - idx >= num_array.len()
                && line[idx..idx + num_array.len()] == num_array[..]
            {
                return Some(u32::try_from(val).unwrap());
            }
        }
    }
    None
}

// TODO: should really dedup this with the function above
fn find_rev(line: &[u8]) -> Option<u32> {
    for idx in (0..line.len()).rev().map(|idx| idx + 1) {
        let byte = line[idx - 1];
        if byte.is_ascii_digit() {
            return Some((byte - b'0').into());
        }
        for (val, num_array) in NUM_AS_BYTES.iter().enumerate() {
            if idx >= num_array.len() && line[(idx - num_array.len())..idx] == num_array[..] {
                return Some(u32::try_from(val).unwrap());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(142, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test2.txt");
        assert_eq!(281, part_two(data));
    }
}
