use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> i32 {
    data.lines()
        .map(split_in_half)
        .map(|(a, b)| find_common((a, b)).unwrap_or('a'))
        .map(priority)
        .sum()
}

fn part_two(data: &str) -> i32 {
    let mut sum = 0;
    for lines in &data.lines().chunks(3) {
        for (a, b, c) in lines.tuples() {
            let z = find_common_3((a, b, c)).unwrap();
            sum += priority(z);
        }
    }
    sum
}

fn find_common_3((a, b, c): (&str, &str, &str)) -> Option<char> {
    for a_c in a.chars() {
        for b_c in b.chars() {
            if a_c == b_c {
                for c_c in c.chars() {
                    if c_c == a_c {
                        return Some(a_c);
                    }
                }
            }
        }
    }
    None
}

fn split_in_half(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

fn find_common((a, b): (&str, &str)) -> Option<char> {
    for a_c in a.chars() {
        for b_c in b.chars() {
            if a_c == b_c {
                return Some(a_c);
            }
        }
    }
    None
}

fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(157, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(70, part_two(data));
    }
}
