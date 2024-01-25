#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod part;
mod part_range;
mod workflows;

use crate::part::Part;
use crate::workflows::Workflows;

#[allow(clippy::missing_panics_doc)]
pub fn main() {
    let data = include_str!("input.txt");
    let (workflows, parts) = data.split_once("\n\n").unwrap();
    let mut workflows: Workflows<'_, 541> = workflows.try_into().unwrap();
    workflows.build_ranges();

    println!("Part 1: {}", part_one::<541>(&workflows, parts));
    println!("Part 2: {}", part_two::<541>(&workflows));
}

fn part_one<const N: usize>(workflows: &Workflows<'_, N>, parts: &str) -> u32 {
    parts
        .lines()
        .map(|l| l.parse::<Part>().unwrap())
        .filter(|&p| workflows.accepted(p))
        .map(Part::rating)
        .sum()
}

fn part_two<const N: usize>(workflows: &Workflows<'_, N>) -> u64 {
    workflows.combinations()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (workflows, parts) = data.split_once("\n\n").unwrap();
        let mut workflows: Workflows<'_, 11> = workflows.try_into().unwrap();
        workflows.build_ranges();
        assert_eq!(19_114, part_one::<11>(&workflows, parts));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (workflows, _) = data.split_once("\n\n").unwrap();
        let mut workflows: Workflows<'_, 11> = workflows.try_into().unwrap();
        workflows.build_ranges();
        assert_eq!(167_409_079_868_000, part_two::<11>(&workflows));
    }
}
