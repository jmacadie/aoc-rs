#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::split_password::SplitPassword;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    data.lines().filter(|l| valid_password(l, false)).count()
}

fn part_two(data: &'static str) -> usize {
    data.lines().filter(|l| valid_password(l, true)).count()
}

mod word {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Word {
        data: [u8; 8],
    }

    impl From<&[u8]> for Word {
        fn from(v: &[u8]) -> Self {
            let mut data = [0; 8];
            for (o, i) in data.iter_mut().zip(v.iter()) {
                *o = *i;
            }
            Self { data }
        }
    }

    impl Word {
        pub const fn new() -> Self {
            Self { data: [0; 8] }
        }

        pub fn sort(&mut self) {
            self.data.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
        }
    }
}

mod split_password {
    use crate::word::Word;

    pub struct SplitPassword {
        data: [Word; 15],
        length: usize,
    }

    impl SplitPassword {
        pub const fn new() -> Self {
            Self {
                data: [Word::new(); 15],
                length: 0,
            }
        }

        pub fn push(&mut self, new: &'static [u8], sorted: bool) {
            let mut new: Word = new.into();
            if sorted {
                new.sort();
            }
            self.data[self.length] = new;
            self.length += 1;
        }

        pub fn pop(&mut self) -> Option<Word> {
            if self.length == 0 {
                return None;
            }
            self.length -= 1;
            Some(self.data[self.length])
        }
    }

    impl Default for SplitPassword {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct Iter<'a> {
        source: &'a SplitPassword,
        position: usize,
    }

    impl<'a> SplitPassword {
        const fn iter(&'a self) -> Iter<'a> {
            Iter {
                source: self,
                position: 0,
            }
        }
    }

    impl<'a> IntoIterator for &'a SplitPassword {
        type IntoIter = Iter<'a>;
        type Item = Word;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = Word;

        fn next(&mut self) -> Option<Self::Item> {
            if self.position >= self.source.length {
                return None;
            }
            self.position += 1;
            Some(self.source.data[self.position - 1])
        }
    }
}

fn split(input: &'static [u8], sorted: bool) -> SplitPassword {
    let mut out = SplitPassword::new();
    let mut remainder = input;

    let split_next = |d: &'static [u8]| {
        d.iter()
            .position(|&x| x == b' ')
            .map(|pos| (&d[..pos], &d[pos + 1..]))
    };

    while let Some((a, b)) = split_next(remainder) {
        out.push(a, sorted);
        remainder = b;
    }
    out.push(remainder, sorted);
    out
}

fn repeated_parts(mut data: SplitPassword) -> bool {
    while let Some(next) = data.pop() {
        for other in &data {
            if next == other {
                return true;
            }
        }
    }
    false
}

fn valid_password(data: &'static str, sorted: bool) -> bool {
    let data_slice = data.trim().as_bytes();
    let split_data = split(data_slice, sorted);
    !repeated_parts(split_data)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert!(valid_password("aa bb cc dd ee", false));
        assert!(!valid_password("aa bb cc dd aa", false));
        assert!(valid_password("aa bb cc dd aaa", false));
    }

    #[test]
    fn two() {
        assert!(valid_password("abcde fghij", true));
        assert!(!valid_password("abcde xyz ecdab", true));
        assert!(valid_password("a ab abc abd abf abj", true));
        assert!(valid_password("iiii oiii ooii oooi oooo", true));
        assert!(!valid_password("oiii ioii iioi iiio", true));
    }
}
