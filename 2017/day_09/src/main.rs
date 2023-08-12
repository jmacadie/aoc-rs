#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let mut data = data.as_bytes();
    let mut ch;
    let mut level = 0_u32;
    let mut score = 0_u32;

    while !data.is_empty() {
        (ch, data) = data.split_first().unwrap();
        match ch {
            b'{' => level += 1,
            b'<' => data = strip_garbage(data),
            b'}' => {
                score += level;
                level -= 1;
            }
            _ => {}
        }
    }
    score
}

fn part_two(data: &str) -> u32 {
    let mut data = data.as_bytes();
    let mut ch;
    let mut score = 0_u32;

    while !data.is_empty() {
        (ch, data) = data.split_first().unwrap();
        if ch == &b'<' {
            let (count, new) = count_garbage(data);
            score += count;
            data = new;
        }
    }
    score
}

fn strip_garbage(input: &[u8]) -> &[u8] {
    let mut escaped = false;
    let mut data = input;
    let mut ch;

    while !data.is_empty() {
        (ch, data) = data.split_first().unwrap();
        match (escaped, ch) {
            (true, _) => escaped = false,
            (false, b'!') => escaped = true,
            (false, b'>') => return data,
            (_, _) => {}
        }
    }
    unreachable!()
}

fn count_garbage(input: &[u8]) -> (u32, &[u8]) {
    let mut escaped = false;
    let mut data = input;
    let mut ch;
    let mut count = 0;

    while !data.is_empty() {
        (ch, data) = data.split_first().unwrap();
        match (escaped, ch) {
            (true, _) => escaped = false,
            (false, b'!') => escaped = true,
            (false, b'>') => return (count, data),
            (_, _) => count += 1,
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(1, part_one("{}"));
        assert_eq!(6, part_one("{{{}}}"));
        assert_eq!(5, part_one("{{},{}}"));
        assert_eq!(16, part_one("{{{},{},{{}}}}"));
        assert_eq!(1, part_one("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, part_one("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, part_one("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, part_one("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }

    #[test]
    fn two() {
        assert_eq!(0, part_two("<>"));
        assert_eq!(17, part_two("<random characters>"));
        assert_eq!(3, part_two("<<<<>"));
        assert_eq!(2, part_two("<{!>}>"));
        assert_eq!(0, part_two("<!!>"));
        assert_eq!(0, part_two("<!!!>>"));
        assert_eq!(10, part_two("<{o\"i!a,<{i<a>"));
    }

    #[test]
    fn test_strip_garbage() {
        assert!(strip_garbage(b"<>").is_empty());
        assert!(strip_garbage(b"<random characters>").is_empty());
        assert!(strip_garbage(b"<<<<>").is_empty());
        assert!(strip_garbage(b"<{!>}>").is_empty());
        assert!(strip_garbage(b"<!!>").is_empty());
        assert!(strip_garbage(b"<!!!>>").is_empty());
        assert!(strip_garbage(b"<{o\"i!a,<{i<a>").is_empty());
    }
}
