#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    data.bytes()
        .fold((0, State::default()), |(sum, s), ch| match s.step(ch) {
            State::Answer(n) => (sum + n, State::default()),
            next => (sum, next),
        })
        .0
}

enum State {
    Prefix(usize),
    FirstNumber(usize, u32),
    SecondNumber(usize, u32, u32),
    Answer(u32),
}

impl State {
    fn step(&self, char: u8) -> Self {
        match *self {
            Self::Prefix(i) => {
                if char != b"mul("[i] {
                    return Self::default();
                }
                if i == 3 {
                    return Self::FirstNumber(0, 0);
                }
                Self::Prefix(i + 1)
            }
            Self::FirstNumber(i, n) => {
                if i > 0 && char == b',' {
                    return Self::SecondNumber(0, 0, n);
                }
                if i > 2 || !char.is_ascii_digit() {
                    return Self::default();
                }
                let num = 10 * n + u32::from(char - b'0');
                Self::FirstNumber(i + 1, num)
            }
            Self::SecondNumber(i, n, m) => {
                if i > 0 && char == b')' {
                    return Self::Answer(n * m);
                }
                if i > 2 || !char.is_ascii_digit() {
                    return Self::default();
                }
                let num = 10 * n + u32::from(char - b'0');
                Self::SecondNumber(i + 1, num, m)
            }
            Self::Answer(_) => unreachable!(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Prefix(0)
    }
}

fn part_two(data: &str) -> u32 {
    data.bytes()
        .fold((0, State2::default()), |(sum, s), ch| match s.step(ch) {
            State2::Answer(n) => (sum + n, State2::default()),
            next => (sum, next),
        })
        .0
}

#[derive(Debug)]
enum State2 {
    Do(usize, bool),
    Dont(usize),
    Prefix(usize),
    FirstNumber(usize, u32),
    SecondNumber(usize, u32, u32),
    Answer(u32),
}

impl State2 {
    fn step(&self, char: u8) -> Self {
        match *self {
            Self::Do(i, on) => {
                if char != b"do()"[i] {
                    if !on {
                        return Self::Do(0, on);
                    }
                    if i == 2 && char == b'n' {
                        return Self::Dont(3);
                    }
                    return Self::default();
                }
                if i == 3 {
                    return Self::default();
                }
                Self::Do(i + 1, on)
            }
            Self::Dont(i) => {
                if char != b"don't()"[i] {
                    return Self::default();
                }
                if i == 6 {
                    return Self::Do(0, false);
                }
                Self::Dont(i + 1)
            }
            Self::Prefix(i) => {
                if char == b'd' {
                    return Self::Do(1, true);
                }
                if char != b"mul("[i] {
                    return Self::default();
                }
                if i == 3 {
                    return Self::FirstNumber(0, 0);
                }
                Self::Prefix(i + 1)
            }
            Self::FirstNumber(i, n) => {
                if char == b'd' {
                    return Self::Do(1, true);
                }
                if i > 0 && char == b',' {
                    return Self::SecondNumber(0, 0, n);
                }
                if i > 2 || !char.is_ascii_digit() {
                    return Self::default();
                }
                let num = 10 * n + u32::from(char - b'0');
                Self::FirstNumber(i + 1, num)
            }
            Self::SecondNumber(i, n, m) => {
                if char == b'd' {
                    return Self::Do(1, true);
                }
                if i > 0 && char == b')' {
                    return Self::Answer(n * m);
                }
                if i > 2 || !char.is_ascii_digit() {
                    return Self::default();
                }
                let num = 10 * n + u32::from(char - b'0');
                Self::SecondNumber(i + 1, num, m)
            }
            Self::Answer(_) => unreachable!(),
        }
    }
}

impl Default for State2 {
    fn default() -> Self {
        Self::Prefix(0)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(161, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(48, part_two(data));
    }
}
