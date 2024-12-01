#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::cmp::Ordering;

pub fn main() {
    let data = include_str!("input.txt");
    let (list_1, list_2) = parse_lists::<1000>(data);
    println!("Part 1: {}", part_one(&list_1, &list_2));
    println!("Part 2: {}", part_two(&list_1, &list_2));
}

fn part_one(list_1: &[u32], list_2: &[u32]) -> u32 {
    list_1
        .iter()
        .zip(list_2.iter())
        .map(|(a, b)| match a.cmp(b) {
            Ordering::Less => b - a,
            _ => a - b,
        })
        .sum()
}

const fn part_two(list_1: &[u32], list_2: &[u32]) -> u32 {
    let mut i = 0;
    let mut j = 0;
    let mut val_1;
    let mut count_2;
    let mut sum = 0;
    let max_1 = list_1.len();
    let max_2 = list_2.len();
    while i < list_1.len() {
        val_1 = list_1[i];
        while list_2[j] < val_1 {
            j += 1;
            if j == max_2 {
                return sum;
            }
        }
        count_2 = 0;
        while list_2[j] == val_1 {
            count_2 += 1;
            j += 1;
            if j == max_2 {
                break;
            }
        }
        while i < max_1 && list_1[i] == val_1 {
            sum += val_1 * count_2;
            i += 1;
        }
    }
    sum
}

fn parse_lists<const N: usize>(data: &str) -> ([u32; N], [u32; N]) {
    let mut list_1 = std::array::from_fn(|_| 0);
    let mut list_2 = std::array::from_fn(|_| 0);
    data.lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let (Some(a), Some(b), None) = (parts.next(), parts.next(), parts.next()) else {
                unreachable!();
            };
            let a = a
                .parse::<u32>()
                .expect("{list_1} should be parsed into a number");
            let b = b
                .parse::<u32>()
                .expect("{list_2} should be parsed into a number");
            (a, b)
        })
        .zip(list_1.iter_mut())
        .zip(list_2.iter_mut())
        .for_each(|(((a, b), l1), l2)| {
            *l1 = a;
            *l2 = b;
        });
    list_1.sort_unstable();
    list_2.sort_unstable();
    (list_1, list_2)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (list_1, list_2) = parse_lists::<6>(data);
        assert_eq!(11, part_one(&list_1, &list_2));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (list_1, list_2) = parse_lists::<6>(data);
        assert_eq!(31, part_two(&list_1, &list_2));
    }
}
