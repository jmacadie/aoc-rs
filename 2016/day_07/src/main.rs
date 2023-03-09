#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    data.lines()
        .map(IPv7::new)
        .filter(IPv7::supports_tls)
        .count()
}

fn part_two(data: &'static str) -> usize {
    data.lines()
        .map(IPv7::new)
        .filter(IPv7::supports_ssl)
        .count()
}

#[derive(Debug)]
struct IPv7 {
    supernets: Vec<&'static str>,
    hypernets: Vec<&'static str>,
    abas: Vec<&'static str>,
}

impl IPv7 {
    fn new(line: &'static str) -> Self {
        let mut supernets = Vec::new();
        let mut hypernets = Vec::new();
        for (i, segment) in line.split(&['[', ']']).enumerate() {
            if i & 1 == 0 {
                supernets.push(segment);
            } else {
                hypernets.push(segment);
            }
        }
        // TODO: This is a bit of an uneccesarry overhead in part 1
        let abas = Self::find_abas(&supernets);

        Self {
            supernets,
            hypernets,
            abas,
        }
    }

    fn supports_ssl(&self) -> bool {
        for bab in self.abas.iter().map(|&aba| Self::bab(aba)) {
            for &hypernet in &self.hypernets {
                if hypernet.as_bytes().windows(3).any(|s| *s == bab) {
                    return true;
                }
            }
        }
        false
    }

    fn bab(aba: &str) -> [u8; 3] {
        // Converts 'aba' string to bytes
        // Then uses biwsie logic to negate the bits of the index and then compare on the least
        // signifcant bit:
        //
        // n (dec) n (bin) !n (bin) !n & 1 (bin)
        // 0        0        1       1
        // 1        1        0       0
        // 2       10       01       1
        // This allows us to invert the aba order in the outgoing byte array
        // Kept as a byte array (rather than converting back to str) as more ergonomic to pass
        // around and I can compare byte arrays later anyway
        let bytes = aba.as_bytes();
        std::array::from_fn(|i| bytes[!i & 1])
    }

    fn find_abas(supernets: &[&'static str]) -> Vec<&'static str> {
        let mut abas = Vec::new();
        for &supernet in supernets {
            for aba in supernet
                .as_bytes()
                .windows(3)
                .filter(|&slice| slice[0] == slice[2] && slice[0] != slice[1])
                .map(|bytes| std::str::from_utf8(bytes).unwrap())
            {
                abas.push(aba);
            }
        }
        abas
    }

    fn supports_tls(&self) -> bool {
        self.no_abba_hypernets() && self.abba_supernet()
    }

    fn no_abba_hypernets(&self) -> bool {
        !self.hypernets.iter().any(|&s| Self::contains_abba(s))
    }

    fn abba_supernet(&self) -> bool {
        self.supernets.iter().any(|&s| Self::contains_abba(s))
    }

    fn contains_abba(text: &str) -> bool {
        text.as_bytes()
            .windows(4)
            .any(|slice| slice[0] == slice[3] && slice[1] == slice[2] && slice[0] != slice[1])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(3, part_two(data));
    }
}
