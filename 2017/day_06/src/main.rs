#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    str::FromStr,
};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u16 {
    let mut banks: MemoryBanks = data.parse().unwrap();
    //let mut set: HashSet<MemoryBanks, _> = HashSet::with_hasher(FnvHasherBuilder::default());
    let mut set: HashSet<MemoryBanks> = HashSet::new();

    set.insert(banks);

    for i in 0.. {
        banks.step();
        if set.contains(&banks) {
            return i + 1;
        }
        set.insert(banks);
    }
    0
}

fn part_two(data: &str) -> u16 {
    let mut banks: MemoryBanks = data.parse().unwrap();
    //let mut set: HashSet<MemoryBanks, _> = HashSet::with_hasher(FnvHasherBuilder::default());
    let mut set: HashMap<MemoryBanks, u16> = HashMap::new();

    set.insert(banks, 0);

    for i in 0.. {
        banks.step();
        if set.contains_key(&banks) {
            return i - set.get(&banks).unwrap();
        }
        set.insert(banks, i);
    }
    0
}

#[derive(Clone, Copy)]
struct Location(u8);

impl From<u8> for Location {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Blocks(u8);

impl From<u8> for Blocks {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

const BLOCK_MAP: u128 = 0xff;
const BLOCK_SIZE: u8 = 8;

// The 16 banks each take 8 bits of the u128 data
// to indicate how many blocks they hold
// This means that each bank can hold up to 256 blocks,
// which is more than we need as there are only 112 blocks
// in total in the input data
//
// Sadly I couldn't get away with a u64, which would have
// meant 4 bits for each bank as I could see some of the
// banks would go over 15 in value
#[derive(Eq, Clone, Copy)]
struct MemoryBanks {
    data: u128,
    size: u8,
}

impl MemoryBanks {
    fn max(&self) -> (Location, Blocks) {
        let mut data = self.data;
        let mut loc = 0.into();
        let mut val: Blocks = 0.into();

        for i in 0..self.size {
            let block_val = u8::try_from(data & BLOCK_MAP).unwrap();
            if block_val > val.0 {
                loc = i.into();
                val = block_val.into();
            }
            data >>= BLOCK_SIZE;
        }
        (loc, val)
    }

    fn allocate_from(&mut self, loc: Location, num: Blocks) {
        let base = u128::from(num.0 / self.size);
        let rem = num.0 % self.size;
        let rel_index = |i: u8| ((i + self.size) - loc.0 - 1) % self.size;
        let allocated = |i: u8| base + u128::from(rel_index(i) < rem);

        let mut temp = self.data;
        self.data = 0_u128;
        for i in 0..self.size {
            let start = if loc.0 == i { 0 } else { temp & BLOCK_MAP };
            temp >>= BLOCK_SIZE;
            let new = start + allocated(i);
            self.data |= new << (BLOCK_SIZE * i);
        }
    }

    fn step(&mut self) {
        let (loc, num) = self.max();
        self.allocate_from(loc, num);
    }
}

impl Hash for MemoryBanks {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl PartialEq for MemoryBanks {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl FromStr for MemoryBanks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = 0_u128;
        let mut size = 0;

        for (i, val) in s.split_ascii_whitespace().rev().enumerate() {
            if i >= 16 {
                return Err(
                    "More thank 16 parts to the input, which is not accomodated for".to_owned(),
                );
            }
            data <<= BLOCK_SIZE;
            let val: u128 = val
                .parse()
                .map_err(|_| "Cannot parse '{val}' into a positive number")?;
            data |= val;
            size += 1;
        }
        Ok(Self { data, size })
    }
}

impl Display for MemoryBanks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.data;
        for _ in 0..self.size {
            let val = temp & BLOCK_MAP;
            temp >>= BLOCK_SIZE;
            write!(f, "{val} ")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(5, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(4, part_two(data));
    }
}
