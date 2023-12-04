#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    let mut cards = Vec::with_capacity(250);
    cards.extend(data.lines().map(score_scratchcard));
    println!("Part 1: {}", part_one(&cards));
    println!("Part 2: {}", part_two(&cards));
}

fn part_one(cards: &[usize]) -> u32 {
    cards.iter().map(|&wins| scratchcard_value(wins)).sum()
}

fn part_two(cards: &[usize]) -> u32 {
    let mut queue = [0; 250];
    queue.iter_mut().take(cards.len()).for_each(|q| *q = 1);
    cards.iter().enumerate().fold(0, |acc, (i, &wins)| {
        let current = queue[i];
        queue
            .iter_mut()
            .skip(i + 1)
            .take(wins)
            .for_each(|q| *q += current);
        acc + current
    })
}

fn score_scratchcard(s: &str) -> usize {
    let (_, numbers) = s.split_once(": ").unwrap();
    let (winning_numbers, given_numbers) = numbers.split_once(" | ").unwrap();

    let numbers = winning_numbers
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());
    let mut winning_numbers = Vec::with_capacity(10);
    winning_numbers.extend(numbers);

    given_numbers
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .filter(|g| winning_numbers.contains(g))
        .count()
}

const fn scratchcard_value(wins: usize) -> u32 {
    if wins == 0 {
        return 0;
    }
    1 << (wins - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let mut cards = Vec::with_capacity(250);
        cards.extend(data.lines().map(score_scratchcard));
        assert_eq!(13, part_one(&cards));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let mut cards = Vec::with_capacity(250);
        cards.extend(data.lines().map(score_scratchcard));
        assert_eq!(30, part_two(&cards));
    }
}
