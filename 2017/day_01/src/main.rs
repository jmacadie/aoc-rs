#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    captcha(data.trim().as_bytes())
}

fn part_two(data: &str) -> u32 {
    captcha_2(data.trim().as_bytes())
}

fn captcha(data: &[u8]) -> u32 {
    captcha_main(data) + captcha_end(data)
}

fn captcha_end(data: &[u8]) -> u32 {
    let start = data.first().unwrap();
    let end = data.last().unwrap();
    if start != end {
        return 0;
    }
    (start - b'0').into()
}

fn captcha_main(data: &[u8]) -> u32 {
    data.windows(2)
        .filter(|x| x[0] == x[1])
        .map(|x| u32::from(x[0] - b'0'))
        .sum()
}

fn captcha_2(data: &[u8]) -> u32 {
    let (first, second) = data.split_at(data.len() / 2);
    first
        .iter()
        .zip(second.iter())
        .filter(|&(f, s)| f == s)
        .map(|(f, _)| u32::from(f - b'0'))
        .sum::<u32>()
        * 2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(3, part_one("1122"));
        assert_eq!(4, part_one("1111"));
        assert_eq!(0, part_one("1234"));
        assert_eq!(9, part_one("91212129"));
    }

    #[test]
    fn two() {
        assert_eq!(6, part_two("1212"));
        assert_eq!(0, part_two("1221"));
        assert_eq!(4, part_two("123425"));
        assert_eq!(12, part_two("123123"));
        assert_eq!(4, part_two("12131415"));
    }
}
