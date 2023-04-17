#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{fmt::Display, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<8>(data));
    println!("Part 2: {}", part_two::<8>(data));
}

fn part_one<const N: usize>(data: &str) -> Password<N> {
    let mut password = Password::new();
    for line in data.lines() {
        let instr: Instruction<N> = line.parse().unwrap();
        password.apply(instr);
    }
    password
}

fn part_two<const N: usize>(data: &str) -> Password<N> {
    let mut password: Password<N> = "fbgdceah".parse().unwrap();
    for line in data.lines().rev() {
        let instr: Instruction<N> = line.parse().unwrap();
        password.apply(instr.reverse());
    }
    password
}

struct Password<const N: usize>([u8; N]);

impl<const N: usize> Password<N> {
    fn new() -> Self {
        let data = std::array::from_fn(|i| b'a' + u8::try_from(i).unwrap());
        Self(data)
    }

    fn apply(&mut self, instr: Instruction<N>) {
        match instr {
            Instruction::SwapIndex((i, j)) => self.swap_elements(i.0, j.0),
            Instruction::SwapLetter((x, y)) => {
                self.swap_elements(self.find_letter(x.0), self.find_letter(y.0));
            }
            Instruction::RotateLeft(i) => self.rotate(N - i.0),
            Instruction::RotateRight(i) => self.rotate(i.0),
            Instruction::RotateLetter(x) => {
                let mut index = self.find_letter(x.0);
                if index >= 4 {
                    index += 1;
                }
                index += 1;
                self.rotate(index);
            }
            Instruction::RotateReverseLetter(x) => {
                // Normal mapping has unique from -> to relationship:
                // 0 -> 1 (0 + 1 + 0 + 0)
                // 1 -> 3 (1 + 1 + 1 + 0)
                // 2 -> 5 (2 + 1 + 2 + 0)
                // 3 -> 7 (3 + 1 + 3 + 0)
                // 4 -> 2 (4 + 1 + 4 + 1) % 8
                // 5 -> 4 (5 + 1 + 5 + 1) % 8
                // 6 -> 6 (6 + 1 + 6 + 1) % 8
                // 7 -> 0 (7 + 1 + 7 + 1) % 8
                // This means we can just hard-code the reverse mapping
                let map_to = [7, 0, 4, 1, 5, 2, 6, 3];
                let from = self.find_letter(x.0);
                let to = map_to[from];
                self.rotate(to + N - from);
            }
            Instruction::Reverse((i, j)) => self.reverse(i.0, j.0),
            Instruction::Move((i, j)) => self.move_letter(i.0, j.0),
        }
    }

    fn swap_elements(&mut self, i: usize, j: usize) {
        let tmp = self.0[i];
        let tmp = std::mem::replace(&mut self.0[j], tmp);
        self.0[i] = tmp;
    }

    fn find_letter(&self, letter: u8) -> usize {
        self.0
            .iter()
            .enumerate()
            .find(|&(_, l)| l == &letter)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn rotate(&mut self, by: usize) {
        let tmp = std::array::from_fn(|i| self.0[(i + 2 * N - by) % N]);
        self.0 = tmp;
    }

    fn reverse(&mut self, from: usize, to: usize) {
        let mut tot = from + to;
        // Bump up if odd
        if tot & 1 == 1 {
            tot += 1;
        }
        let half = tot / 2;
        for i in from..half {
            let j = to - (i - from);
            self.swap_elements(i, j);
        }
    }

    fn move_letter(&mut self, from: usize, to: usize) {
        let min = std::cmp::min(from, to);
        let max = std::cmp::max(from, to);
        let tmp = std::array::from_fn(|i| match i {
            i if i < min => self.0[i],
            i if i > max => self.0[i],
            i if i == to => self.0[from],
            i if from < to => self.0[i + 1],
            i => self.0[i - 1],
        });
        self.0 = tmp;
    }
}

impl<const N: usize> FromStr for Password<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != N {
            return Err(format!(
                "String length {} does not match required password length {N}",
                s.len()
            ));
        }
        let bytes = s.as_bytes();
        let data = std::array::from_fn(|i| bytes[i]);
        Ok(Self(data))
    }
}

impl<const N: usize> Display for Password<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for letter in self.0 {
            let letter_array = &[letter];
            let char = std::str::from_utf8(letter_array).map_err(|_| std::fmt::Error)?;
            write!(f, "{char}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction<const N: usize> {
    SwapIndex((PasswordIndex<N>, PasswordIndex<N>)),
    SwapLetter((PasswordChar<N>, PasswordChar<N>)),
    RotateLeft(PasswordIndex<N>),
    RotateRight(PasswordIndex<N>),
    RotateLetter(PasswordChar<N>),
    RotateReverseLetter(PasswordChar<N>),
    Reverse((PasswordIndex<N>, PasswordIndex<N>)),
    Move((PasswordIndex<N>, PasswordIndex<N>)),
}

impl<const N: usize> FromStr for Instruction<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, rest) = s.split_once(' ').ok_or("Badly formatted")?;
        match first {
            "swap" => {
                let (next, rest2) = rest
                    .split_once(' ')
                    .ok_or("Badly formatted swap instruction")?;
                match next {
                    "letter" => {
                        let (letter1, rest3) = rest2
                            .split_once(' ')
                            .ok_or("Badly formatted swap letter instruction")?;
                        let letter2 = rest3.trim_start_matches("with letter ");
                        let letter1 = letter1.parse()?;
                        let letter2 = letter2.parse()?;
                        Ok(Self::SwapLetter((letter1, letter2)))
                    }
                    "position" => {
                        let (position1, rest3) = rest2
                            .split_once(' ')
                            .ok_or("Badly formatted swap letter instruction")?;
                        let position2 = rest3.trim_start_matches("with position ");
                        let position1 = position1.parse()?;
                        let position2 = position2.parse()?;
                        Ok(Self::SwapIndex((position1, position2)))
                    }
                    _ => Err("Incorrect swap type".to_owned()),
                }
            }
            "rotate" => {
                let (next, rest2) = rest
                    .split_once(' ')
                    .ok_or("Badly formatted rotate instruction")?;
                match next {
                    "left" => {
                        let (count, _) = rest2
                            .split_once(' ')
                            .ok_or("Badly formatted rotate left instruction")?;
                        let count = count.parse()?;
                        Ok(Self::RotateLeft(count))
                    }
                    "right" => {
                        let (count, _) = rest2
                            .split_once(' ')
                            .ok_or("Badly formatted rotate right instruction")?;
                        let count = count.parse()?;
                        Ok(Self::RotateRight(count))
                    }
                    "based" => {
                        let (_, letter) = rest2
                            .split_once("position of letter ")
                            .ok_or("Badly formatted rotate based on letter instruction")?;
                        let letter = letter.parse()?;
                        Ok(Self::RotateLetter(letter))
                    }
                    _ => Err("Incorrect rotate type".to_owned()),
                }
            }
            "reverse" => {
                let rest2 = rest.trim_start_matches("positions ");
                let (position1, rest3) = rest2
                    .split_once(' ')
                    .ok_or("Badly formatted reverse instruction")?;
                let position2 = rest3.trim_start_matches("through ");
                let position1 = position1.parse()?;
                let position2 = position2.parse()?;
                Ok(Self::Reverse((position1, position2)))
            }
            "move" => {
                let rest2 = rest.trim_start_matches("position ");
                let (position1, rest3) = rest2
                    .split_once(' ')
                    .ok_or("Badly formatted move instruction")?;
                let position2 = rest3.trim_start_matches("to position ");
                let position1 = position1.parse()?;
                let position2 = position2.parse()?;
                Ok(Self::Move((position1, position2)))
            }
            _ => Err("Incorrect first instruction word".to_owned()),
        }
    }
}

impl<const N: usize> Instruction<N> {
    const fn reverse(self) -> Self {
        match self {
            Self::SwapIndex(..) | Self::SwapLetter(..) | Self::Reverse(..) => self,
            Self::RotateLeft(i) => Self::RotateRight(i),
            Self::RotateRight(i) => Self::RotateLeft(i),
            Self::Move((i, j)) => Self::Move((j, i)),
            Self::RotateLetter(x) => Self::RotateReverseLetter(x),
            Self::RotateReverseLetter(x) => Self::RotateLetter(x),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PasswordIndex<const N: usize>(usize);

impl<const N: usize> FromStr for PasswordIndex<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: usize = s
            .parse()
            .map_err(|_| "Cannot parse supplied index {s} into a usize number")?;
        if n > N {
            return Err(format!(
                "Index number {n} is greater than the max allowed of {N}"
            ));
        }
        Ok(Self(n))
    }
}

#[derive(Debug, Clone, Copy)]
struct PasswordChar<const N: usize>(u8);

impl<const N: usize> FromStr for PasswordChar<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(format!("More than one character supplied: {s}"));
        }
        let n = s.as_bytes()[0];
        let end = b'a' + u8::try_from(N).unwrap();

        if !(b'a'..=end).contains(&n) {
            return Err(format!(
                "Character {s} is not between 'a' and '{}'",
                std::str::from_utf8(&[end]).unwrap()
            ));
        }
        Ok(Self(n))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!("decab", part_one::<5>(data).to_string());
    }
}
