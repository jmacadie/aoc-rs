use std::str::FromStr;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data)?);
    println!("Part 2: {}", part_two(data));

    Ok(())
}

fn part_one(data: &str) -> color_eyre::Result<u64> {
    let mut total = 0;
    for round in data.lines().map(|l| l.parse::<Round>()) {
        total += round?.points();
    }

    Ok(total)
}

fn part_two(_data: &str) -> u64 {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opp: Move,
    ours: Move,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Move {
    fn points(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
 
    fn beats(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        if self.ours.beats() == self.opp {
            Outcome::Win
        } else if self.opp.beats() == self.ours {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn points(&self) -> u64 {
        self.ours.points() + self.outcome().points()
    }
}

impl Outcome {
    fn points(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;
    
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}"))
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opp), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <opp>SP<ours>EOL, got {s:?}"));
        };

        Ok(Self {
            opp: opp.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(15, part_one(data).unwrap());
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(12, part_two(data));
    }
}
