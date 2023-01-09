#![warn(clippy::all, clippy::pedantic)]

use std::ops::Range;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> i32 {
    count_numbers(data)
}

fn part_two(data: &str) -> i32 {
    let adj = strip_red(data);
    count_numbers(&adj)
}

fn count_numbers(data: &str) -> i32 {
    let mut out = 0;
    let mut num = 0;
    let mut sub = false;
    for ch in data.as_bytes() {
        match (num, ch) {
            (0, b'-') => sub = true,
            (0, b'1'..=b'9') => num = i32::from(ch - b'0'),
            (0, _) => sub = false,
            (_, b'0'..=b'9') => {
                num *= 10;
                num += i32::from(ch - b'0');
            }
            (_, _) => {
                if sub {
                    out -= num;
                    sub = false;
                } else {
                    out += num;
                }
                num = 0;
            }
        }
    }
    out
}

fn strip_red(data: &str) -> String {
    let mut out = data.to_owned();
    let mut idx = 0;

    while let Some(mut posn) = find_red(out.as_bytes(), idx) {
        if let Some(range) = object_range(out.as_bytes(), posn) {
            posn = range.start;
            out.replace_range(range, "");
        }
        idx = posn + 1;
    }

    out
}

fn find_red(data: &[u8], from: usize) -> Option<usize> {
    if data.len() < 4 || from > data.len() - 4 {
        return None;
    }
    let (_, read) = data.split_at(from);
    for i in 0..read.len() - 4 {
        if read[i..i + 5] == [b'"', b'r', b'e', b'd', b'"'] {
            return Some(from + i);
        }
    }
    None
}

fn object_range(data: &[u8], from: usize) -> Option<Range<usize>> {
    let (before, after) = data.split_at(from);
    if let Some(start) = look_back(before) {
        if let Some(end) = look_forwards(after) {
            return Some(start..from + end);
        }
    }
    None
}

fn look_back(data: &[u8]) -> Option<usize> {
    let mut arr_count = 0;
    let mut obj_count = 0;
    let mut temp = data;
    let mut ch;
    let mut idx = data.len();

    while !temp.is_empty() {
        (ch, temp) = temp.split_last().unwrap();
        idx -= 1;
        match (ch, arr_count, obj_count) {
            (b']', _, _) => arr_count += 1,
            (b'}', _, _) => obj_count += 1,
            (b'[', 0, _) => return None,
            (b'{', _, 0) => return Some(idx),
            (b'[', _, _) => arr_count -= 1,
            (b'{', _, _) => obj_count -= 1,
            (_, _, _) => (),
        }
    }
    None
}

fn look_forwards(data: &[u8]) -> Option<usize> {
    let mut obj_count = 0;
    let mut temp = data;
    let mut ch;
    let mut idx = 1;

    while !temp.is_empty() {
        (ch, temp) = temp.split_first().unwrap();
        match (ch, obj_count) {
            (b'{', _) => obj_count += 1,
            (b'}', 0) => return Some(idx),
            (b'}', _) => obj_count -= 1,
            (_, _) => (),
        }
        idx += 1;
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(6, part_one("[1,2,3]"));
        assert_eq!(6, part_one("{{\"a\":2,\"b\":4}}"));
        assert_eq!(3, part_one("[[[3]]]"));
        assert_eq!(3, part_one("{{\"a\":{{\"b\":4}},\"c\":-1}}"));
        assert_eq!(0, part_one("{{\"a\":[-1,1]}}"));
        assert_eq!(0, part_one("[-1,{{\"a\":1}}]"));
        assert_eq!(0, part_one("[]"));
        assert_eq!(0, part_one("{{}}"));
    }

    #[test]
    fn two() {
        assert_eq!(6, part_two("[1,2,3]"));
        assert_eq!(4, part_two("[1,{{\"c\":\"red\",\"b\":2}},3]"));
        assert_eq!(0, part_two("{{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}}"));
        assert_eq!(6, part_two("[1,\"red\",5]"));
        assert_eq!(0, part_two("[]"));
        assert_eq!(0, part_two("{{}}"));
    }
}
