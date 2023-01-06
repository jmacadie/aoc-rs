#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    iterate_length(data, 40)
}

fn part_two(data: &str) -> usize {
    iterate_length(data, 50)
}

fn iterate_length(data: &str, times: usize) -> usize {
    let mut ls = read_line(data);
    for _ in 0..times {
        ls = iterate(&ls);
    }
    ls.len()
}

type LookSay = Vec<u8>;

fn read_line(line: &str) -> LookSay {
    let mut num = Vec::with_capacity(10);
    for elem in line.trim().as_bytes().iter() {
        num.push(elem - b'0');
    }
    num
}

fn iterate(mut num: &[u8]) -> LookSay {
    let mut next = Vec::with_capacity(num.len() * 2);
    while !num.is_empty() {
        let (digit, mut rest) = num.split_first().unwrap();
        let mut count = 1;
        while !rest.is_empty() && rest[0] == *digit {
            (_, rest) = rest.split_first().unwrap();
            count += 1;
        }
        next.push(count);
        next.push(*digit);
        num = rest;
    }
    next
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(6, iterate_length(data, 5));
    }
}
