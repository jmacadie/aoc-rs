pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data)?);
    println!("Part 2: {}", part_two(data)?);

    Ok(())
}

pub fn bench() {
    let data = include_str!("input.txt");
    let _ = part_one(data);
    let _ = part_two(data);
}

fn part_one(data: &str) -> color_eyre::Result<u64> {
    let mut total = 0;
    for round in data.lines().map(Round::from_moves_str) {
        total += round?.points();
    }

    Ok(total)
}

fn part_two(data: &str) -> color_eyre::Result<u64> {
    let mut total = 0;
    for round in data.lines().map(Round::from_outcome_str) {
        total += round?.points();
    }

    Ok(total)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
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

    fn beaten_by(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

impl Round {
    fn from_moves_str(line: &str) -> color_eyre::Result<Self> {
        let mut parts = line.chars();
        let (Some(opp), Some(' '), Some(ours), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
            return Err(color_eyre::eyre::eyre!("was expecting opp<SP>ours<EOL>, got {line}"));
        };
        let opp = opp.try_into()?;
        let ours = ours.try_into()?;
        Ok(Self::from_moves(opp, ours))
    }

    fn from_moves(opp: Move, ours: Move) -> Self {
        Self { opp, ours }
    }

    fn from_outcome_str(line: &str) -> color_eyre::Result<Self> {
        let mut parts = line.chars();
        let (Some(opp), Some(' '), Some(outcome), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
            return Err(color_eyre::eyre::eyre!("was expecting opp<SP>outcome<EOL>, got {line}"));
        };
        let opp = opp.try_into()?;
        let outcome = outcome.try_into()?;
        Ok(Self::from_outcome(opp, outcome))
    }

    fn from_outcome(opp: Move, outcome: Outcome) -> Self {
        let ours = match outcome {
            Outcome::Win => opp.beaten_by(),
            Outcome::Draw => opp,
            Outcome::Loss => opp.beats(),
        };
        Self { opp, ours }
    }

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
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
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
        assert_eq!(12, part_two(data).unwrap());
    }
}
