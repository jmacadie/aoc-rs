#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use md5::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Md5,
};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> u32 {
    let mut finder = PadFinder::new(data.trim(), 0);
    finder.find_pads();
    finder.found[63].unwrap().index
}

fn part_two(data: &'static str) -> u32 {
    let mut finder = PadFinder::new(data.trim(), 2016);
    finder.find_pads();
    finder.found[63].unwrap().index
}

#[derive(Debug)]
struct PadFinder {
    index: u32,
    candidates: [Option<Key>; 200],
    candidate_count: usize,
    found: [Option<Key>; 70],
    found_count: usize,
    prefix: &'static str,
    stretching: u16,
}

impl PadFinder {
    const fn new(prefix: &'static str, stretching: u16) -> Self {
        Self {
            index: 0,
            candidates: [None; 200],
            candidate_count: 0,
            found: [None; 70],
            found_count: 0,
            prefix,
            stretching,
        }
    }

    fn find_pads(&mut self) {
        while self.found_count < 64 {
            self.step();
        }
    }

    fn step(&mut self) {
        self.remove_unmatched_keys();

        let digest = self.hash();
        let (three, fives) = find_repeats(&digest);
        self.find_complete_keys(fives);

        // Add candidate key
        if let Some(three) = three {
            let key = Key::new(self.index, three);
            self.candidates[self.candidate_count] = Some(key);
            self.candidate_count += 1;
        }

        self.index += 1;
    }

    fn hash(&self) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>> {
        let mut digest = Md5::digest(format!("{}{}", self.prefix, self.index));
        for _ in 0..self.stretching {
            digest = Md5::digest(format!("{digest:x}"));
        }
        digest
    }

    fn find_complete_keys(&mut self, fives: [Option<u8>; 4]) {
        let fives_iter = fives.iter().filter_map(|&f| f);
        let candidates_iter = self
            .candidates
            .iter()
            .take(self.candidate_count)
            .map(|c| c.unwrap())
            .enumerate();
        let mut found = [0; 20];
        let mut found_count = 0;
        for (five, (i, mut candidate)) in fives_iter.cartesian_product(candidates_iter) {
            if candidate.character == five {
                candidate.final_index = self.index;
                self.found[self.found_count] = Some(candidate);
                self.found_count += 1;

                found[found_count] = i;
                found_count += 1;
            }
        }
        for i in found.iter().take(found_count).rev() {
            self.remove_key(*i);
        }
    }

    fn remove_unmatched_keys(&mut self) {
        if let Some(key) = self.candidates[0] {
            if key.index + 1_000 <= self.index {
                self.remove_key(0);
            }
        }
    }

    fn remove_key(&mut self, index: usize) {
        if index < self.candidate_count {
            self.candidate_count -= 1;
            for i in index..self.candidate_count {
                self.candidates[i] = self.candidates[i + 1];
            }
            // Not strictly necessary
            self.candidates[self.candidate_count] = None;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Key {
    index: u32,
    final_index: u32,
    character: u8,
}

impl Key {
    const fn new(index: u32, character: u8) -> Self {
        Self {
            index,
            final_index: 0,
            character,
        }
    }
}

fn find_repeats(digest: &impl AsRef<[u8]>) -> (Option<u8>, [Option<u8>; 4]) {
    let mut count = 0;
    let mut hex_char = None;
    let mut out = (None, [None; 4]);
    let mut fives = 0;
    for pair in digest.as_ref() {
        let one = pair >> 4;
        let two = pair & 0b1111;
        if Some(one) == hex_char {
            count += 1;
            if count == 3 && out.0.is_none() {
                out.0 = hex_char;
            }
            if count == 5 {
                out.1[fives] = hex_char;
                fives += 1;
            }
        } else {
            hex_char = Some(one);
            count = 1;
        }
        if Some(two) == hex_char {
            count += 1;
            if count == 3 && out.0.is_none() {
                out.0 = hex_char;
            }
            if count == 5 {
                out.1[fives] = hex_char;
                fives += 1;
            }
        } else {
            hex_char = Some(two);
            count = 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(22728, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(22551, part_two(data));
    }
}
