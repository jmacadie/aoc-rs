use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: Max calories is {}", part_one(data));
    println!("Part 2: Top 3 calories is {}", part_two(data));
}

fn part_one(data: &str) -> u64 {
    data.lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max()
        .unwrap_or_default()
}

fn part_two(data: &str) -> u64 {
    data.lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(24_000, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(45_000, part_two(data));
    }
}
