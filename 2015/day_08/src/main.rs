#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines().map(read_line).map(|(m, c)| m - c).sum()
}

fn part_two(data: &str) -> usize {
    data.lines().map(encode_line).map(|(m, e)| e - m).sum()
}

fn read_line(line: &str) -> (usize, usize) {
    let mem_len = line.as_bytes().len();

    let mut char_len = 0;
    let mut string = &line.as_bytes()[1..mem_len - 1];
    while !string.is_empty() {
        let (ch, rest) = string.split_first().unwrap();
        if let b'\\' = ch {
            match rest.split_first().unwrap() {
                (b'"' | b'\\', _) => {
                    string = &rest[1..];
                }
                (b'x', _) => {
                    string = &rest[3..];
                }
                _ => unreachable!(),
            }
        } else {
            string = rest;
        }
        char_len += 1;
    }
    (mem_len, char_len)
}

fn encode_line(line: &str) -> (usize, usize) {
    let mem_len = line.as_bytes().len();

    let enc_len = mem_len
        + 2
        + line
            .as_bytes()
            .iter()
            .filter(|&ch| ch == &b'"' || ch == &b'\\')
            .count();

    (mem_len, enc_len)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(12, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(19, part_two(data));
    }
}
