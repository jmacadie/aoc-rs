#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u16 {
    let counts = data.lines().map(Box::new).map(|b| b.checksum()).fold(
        (0, 0),
        |(mut doubles, mut triples), line| {
            if line.0 {
                doubles += 1;
            }
            if line.1 {
                triples += 1;
            }
            (doubles, triples)
        },
    );
    counts.0 * counts.1
}

fn part_two(data: &str) -> String {
    let mut boxes = Vec::with_capacity(250);
    boxes.extend(data.lines().map(Box::new));

    let mut remaining = &boxes[..];
    while let Some((first, body)) = remaining.split_first() {
        if let Some(idx) = body.iter().position(|b| b.one_different(first)) {
            return first.common_letters(&body[idx]);
        }
        remaining = body;
    }
    String::new()
}

struct Box<'a> {
    id: &'a [u8],
}

impl<'a> Box<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            id: data.as_bytes(),
        }
    }

    fn checksum(&self) -> (bool, bool) {
        let counts = self
            .id
            .iter()
            .fold([0_u8; 256], |mut counts: [u8; 256], &letter| {
                counts[usize::from(letter)] += 1;
                counts
            });
        let double = counts.iter().any(|&v| v == 2);
        let triple = counts.iter().any(|&v| v == 3);
        (double, triple)
    }

    fn one_different(&self, other: &Self) -> bool {
        let mut found = false;
        for (a, b) in self.id.iter().zip(other.id.iter()) {
            if a != b {
                if found {
                    return false;
                }
                found = true;
            }
        }
        found
    }

    fn common_letters(&self, other: &Self) -> String {
        String::from_utf8(
            self.id
                .iter()
                .zip(other.id.iter())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| *a)
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(12, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test2.txt");
        assert_eq!("fgij", part_two(data));
    }
}
