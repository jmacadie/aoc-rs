#![warn(clippy::all, clippy::pedantic)]
use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<8>(data));
    println!("Part 2: {}", part_two::<8>(data));
}

fn part_one<const N: usize>(data: &str) -> i32 {
    let m = read_in::<N>(data);
    let mut max = 0;
    for mut perm in (0..N - 1).permutations(N - 1) {
        let mut happiness = 0;
        perm.push(N - 1);
        for (&a, &b) in perm.iter().circular_tuple_windows() {
            happiness += m[a][b];
            happiness += m[b][a];
        }
        max = std::cmp::max(max, happiness);
    }
    max
}

fn part_two<const N: usize>(data: &str) -> i32 {
    let m = read_in::<N>(data);
    let mut max = 0;
    for perm in (0..N).permutations(N) {
        let mut happiness = 0;
        for (&a, &b) in perm.iter().tuple_windows() {
            happiness += m[a][b];
            happiness += m[b][a];
        }
        max = std::cmp::max(max, happiness);
    }
    max
}

type HappinessMatrix<const N: usize> = [[i32; N]; N];

fn read_in<const N: usize>(data: &str) -> HappinessMatrix<N> {
    let mut out = [[0; N]; N];
    for line in data.lines() {
        let (source, target, val) = read_line(line);
        out[source][target] = val;
    }
    out
}

fn read_line(line: &str) -> (usize, usize, i32) {
    let (source, rest) = line.split_once(" would ").unwrap();
    let (happiness, target) = rest
        .split_once(" happiness units by sitting next to ")
        .unwrap();
    let target = target.trim_end_matches('.');

    let (dir, num) = happiness.split_once(' ').unwrap();
    let num = match dir {
        "gain" => num.parse().unwrap(),
        "lose" => -num.parse::<i32>().unwrap(),
        _ => unreachable!(),
    };

    (name_to_index(source), name_to_index(target), num)
}

fn name_to_index(name: &str) -> usize {
    const NAMES: [&str; 8] = [
        "Alice", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory",
    ];

    for (i, &target) in NAMES.iter().enumerate() {
        if name == target {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(330, part_one::<4>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(286, part_two::<4>(data));
    }
}
