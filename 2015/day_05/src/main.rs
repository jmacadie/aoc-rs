#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines().filter(|&l| is_nice_1(l.as_bytes())).count()
}

fn part_two(data: &str) -> usize {
    data.lines().filter(|&l| is_nice_2(l.as_bytes())).count()
}

fn is_nice_1(input: &[u8]) -> bool {
    three_vowels(input) && double_letter(input) && no_bad_sustrings(input)
}

fn is_nice_2(input: &[u8]) -> bool {
    non_overlapping_pair(input) && seperated_repeated_letter(input)
}

fn three_vowels(input: &[u8]) -> bool {
    let mut count: u8 = 0;
    for charachter in input {
        match charachter {
            b'a' | b'e' | b'i' | b'o' | b'u' => {
                if count == 2 {
                    return true;
                }
                count += 1;
            }
            _ => (),
        }
    }
    false
}

fn double_letter(input: &[u8]) -> bool {
    let mut last: u8 = 0;
    for charachter in input {
        if charachter == &last {
            return true;
        }
        last = *charachter;
    }
    false
}

const fn no_bad_sustrings(mut input: &[u8]) -> bool {
    while let Some((charachter, rest)) = input.split_first() {
        if rest.is_empty() {
            return true;
        }
        match charachter {
            b'a' => {
                if rest[0] == b'b' {
                    return false;
                }
            }
            b'c' => {
                if rest[0] == b'd' {
                    return false;
                }
            }
            b'p' => {
                if rest[0] == b'q' {
                    return false;
                }
            }
            b'x' => {
                if rest[0] == b'y' {
                    return false;
                }
            }
            _ => (),
        }
        input = rest;
    }
    true
}

fn non_overlapping_pair(mut input: &[u8]) -> bool {
    /*input
    .windows(2)
    .dedup()
    .sorted_unstable()
    .dedup_with_count()
    .any(|(count, _)| count > 1)*/
    if input.len() < 4 {
        return false;
    }
    while let Some((head, rest)) = input.split_first() {
        if rest.is_empty() {
            return false;
        }
        if matching_pair(*head, rest[0], &rest[1..]) {
            return true;
        }
        input = rest;
    }
    false
}

const fn matching_pair(char_1: u8, char_2: u8, mut search: &[u8]) -> bool {
    while let Some((charachter, rest)) = search.split_first() {
        if rest.is_empty() {
            return false;
        }
        if *charachter == char_1 && rest[0] == char_2 {
            return true;
        }
        search = rest;
    }
    false
}

fn seperated_repeated_letter(input: &[u8]) -> bool {
    input.windows(3).any(|slice| slice[0] == slice[2])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_one(data));
    }

    #[test]
    fn overlap_test() {
        assert!(non_overlapping_pair(b"xyxy"));
        assert!(non_overlapping_pair(b"aabcdefgaa"));
        assert!(!non_overlapping_pair(b"aaa"));
        assert!(non_overlapping_pair(b"aaaa"));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
