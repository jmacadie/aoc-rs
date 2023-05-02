#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let (v1, v2) = get_special_numbers(data);
    factorial(7) + v1 * v2
}

fn part_two(data: &str) -> u32 {
    let (v1, v2) = get_special_numbers(data);
    factorial(12) + v1 * v2
}

fn get_special_numbers(input: &str) -> (u32, u32) {
    let mut data = input.lines();
    let mut first = data.nth(19).unwrap().split_whitespace();
    let mut second = data.next().unwrap().split_whitespace();
    let (Some("cpy"), Some(v1), Some("c"), None) = (first.next(), first.next(), first.next(), first.next()) else {
        unreachable!();
    };
    let (Some("jnz"), Some(v2), Some("d"), None) = (second.next(), second.next(), second.next(), second.next()) else {
        unreachable!();
    };
    let v1 = v1
        .parse()
        .unwrap_or_else(|_| panic!("Could not convert v1: {v1} to a number"));
    let v2 = v2
        .parse()
        .unwrap_or_else(|_| panic!("Could not convert v2: {v2} to a number"));
    (v1, v2)
}

fn factorial(num: u32) -> u32 {
    (1..=num).into_iter().product()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
