#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data, "northpole object storage"));
}

fn part_one(data: &'static str) -> u32 {
    data.lines()
        .map(Room::new)
        .filter(Room::real)
        .map(|r| r.sector_id)
        .sum()
}

fn part_two(data: &'static str, check: &str) -> u32 {
    let room = data
        .lines()
        .map(Room::new)
        .filter(Room::real)
        .find(|r| r.decrypted_equals(check))
        .unwrap();
    room.sector_id
}

struct Room {
    encrypted_name: &'static str,
    sector_id: u32,
    checksum: &'static str,
}

impl Room {
    fn new(line: &'static str) -> Self {
        let (body, checksum) = line.split_once('[').unwrap();
        let checksum = checksum.trim_end_matches(']');

        let (encrypted_name, sector_id) = body.rsplit_once('-').unwrap();
        let sector_id = sector_id.parse().unwrap();

        Self {
            encrypted_name,
            sector_id,
            checksum,
        }
    }

    fn real(&self) -> bool {
        self.encrypted_name
            .chars()
            .filter(|&c| c != '-')
            .sorted_unstable()
            .dedup_with_count()
            .sorted_unstable_by_key(|&(c, _)| std::cmp::Reverse(c))
            .take(5)
            .map(|(_, ch)| ch)
            .collect::<String>()
            == self.checksum
    }

    fn decrypted_equals(&self, check: &str) -> bool {
        if self.encrypted_name.len() != check.len() {
            return false;
        }
        let shift = u8::try_from(self.sector_id % 26).unwrap();
        let limit = b'z' - shift;
        for (this, test) in self
            .encrypted_name
            .as_bytes()
            .iter()
            .zip(check.as_bytes().iter())
        {
            if this == &b'-' && test == &b' ' {
                continue;
            }
            let shifted = if this > &limit {
                this - 26 + shift
            } else {
                this + shift
            };
            if shifted != *test {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(1514, part_one(data));
        assert_eq!(343, part_one("qzmt-zixmtkozy-ivhz-343[zimth]"));
    }

    #[test]
    fn two() {
        assert_eq!(
            343,
            part_two("qzmt-zixmtkozy-ivhz-343[zimth]", "very encrypted name")
        );
    }
}
