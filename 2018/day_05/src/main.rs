#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

const CAP_BIT: u8 = 0b0010_0000; // 32

pub fn main() {
    let data = include_str!("input.txt");
    let reduced = reduced_polymer(data.trim().as_bytes().iter().copied());
    println!("Part 1: {}", part_one(&reduced));
    println!("Part 2: {}", part_two(&reduced));
}

const fn part_one(reduced: &[u8]) -> usize {
    reduced.len()
}

fn part_two(reduced: &[u8]) -> usize {
    (b'a'..=b'z')
        .map(|removed| {
            reduced_polymer(
                reduced
                    .iter()
                    .copied()
                    .filter(|&unit| unit | CAP_BIT != removed),
            )
            .len()
        })
        .min()
        .unwrap()
}

fn reduced_polymer(input: impl Iterator<Item = u8>) -> Vec<u8> {
    input.fold(Vec::with_capacity(20_000), |mut polymer, unit| {
        if polymer.last().is_some_and(|&l| l ^ unit == CAP_BIT) {
            polymer.pop();
        } else {
            polymer.push(unit);
        }
        polymer
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let reduced = reduced_polymer(data.trim().as_bytes().iter().copied());
        assert_eq!(10, part_one(&reduced));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let reduced = reduced_polymer(data.trim().as_bytes().iter().copied());
        assert_eq!(4, part_two(&reduced));
    }
}
