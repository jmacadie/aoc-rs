#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use std::str::FromStr;

#[allow(clippy::missing_panics_doc)]
pub fn main() {
    let data = include_str!("input.txt");
    let mut game = data.parse::<CamelCards>().unwrap();
    println!("Part 1: {}", part_one(&mut game));
    println!("Part 2: {}", part_two(&mut game));
}

fn part_one(game: &mut CamelCards) -> u32 {
    game.score()
}

fn part_two(game: &mut CamelCards) -> u32 {
    game.add_jokers();
    game.score()
}

#[derive(Debug)]
struct CamelCards {
    data: Vec<(Hand, u32)>,
}

impl CamelCards {
    fn score(&mut self) -> u32 {
        self.data.iter_mut().for_each(|h| h.0.score());
        self.data.sort_unstable_by_key(|&(h, _)| h);
        self.data
            .iter()
            .enumerate()
            .map(|(i, &(_, b))| u32::try_from(i + 1).unwrap() * b)
            .sum()
    }

    fn add_jokers(&mut self) {
        self.data.iter_mut().for_each(|h| h.0.add_jokers());
    }
}

impl FromStr for CamelCards {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        for l in s.lines() {
            let (hand, bid) = l
                .split_once(' ')
                .ok_or_else(|| format!("Cannot split line into the hand and bid: {l}"))?;
            let hand = hand.parse::<Hand>()?;
            let bid = bid
                .parse::<u32>()
                .map_err(|_| format!("Cannot convert the bid {bid} into a number"))?;
            data.push((hand, bid));
        }
        Ok(Self { data })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    category: HandType,
}

impl Hand {
    fn score(&mut self) {
        let jokers = self.cards.iter().filter(|&&c| c == Card::Joker).count();
        let mut temp = self.cards;
        temp.sort_unstable();
        let counts = temp
            .into_iter()
            .filter(|&c| c != Card::Joker)
            .dedup_with_count()
            .map(|(i, _)| i)
            .sorted_unstable_by_key(|&i| std::cmp::Reverse(i))
            .collect_vec();
        self.category = match (counts.first(), counts.get(1), jokers) {
            (Some(5), None, 0)
            | (Some(4), None, 1)
            | (Some(3), None, 2)
            | (Some(2), None, 3)
            | (Some(1), None, 4)
            | (None, None, 5) => HandType::FiveOfAKind,
            (Some(4), Some(1), 0)
            | (Some(3), Some(1), 1)
            | (Some(2), Some(1), 2)
            | (Some(1), Some(1), 3) => HandType::FourOfAKind,
            (Some(3), Some(2), 0) | (Some(2), Some(2), 1) => HandType::FullHouse,
            (Some(3), Some(1), 0) | (Some(2), Some(1), 1) | (Some(1), Some(1), 2) => {
                HandType::ThreeOfAKind
            }
            (Some(2), Some(2), 0) => HandType::TwoPair,
            (Some(2), Some(1), 0) | (Some(1), Some(1), 1) => HandType::OnePair,
            (Some(1), Some(1), 0) => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    fn add_jokers(&mut self) {
        self.cards
            .iter_mut()
            .filter(|c| **c == Card::Jack)
            .for_each(|c| *c = Card::Joker);
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(format!(
                "Hand is expected to be made of 5 cards exactly: {s}"
            ));
        }
        let mut cards = [Card::Two; 5];
        for (i, c) in cards.iter_mut().enumerate() {
            *c = s[i..=i].parse()?;
        }
        let output = Self {
            cards,
            category: HandType::HighCard,
        };
        Ok(output)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.category
            .cmp(&other.category)
            .then(self.cards[0].cmp(&other.cards[0]))
            .then(self.cards[1].cmp(&other.cards[1]))
            .then(self.cards[2].cmp(&other.cards[2]))
            .then(self.cards[3].cmp(&other.cards[3]))
            .then(self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    const fn rank(self) -> u8 {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    const fn rank(self) -> u8 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
            Self::Joker => 1,
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(format!("Card {s} is expected to only be one character"));
        }
        match s {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Jack),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            _ => Err(format!("Card {s} is not recognised")),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let mut game = data.parse::<CamelCards>().unwrap();
        assert_eq!(6440, part_one(&mut game));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let mut game = data.parse::<CamelCards>().unwrap();
        assert_eq!(5905, part_two(&mut game));
    }
}
