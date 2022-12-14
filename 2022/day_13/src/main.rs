use crate::packet::Packet;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    data.split("\n\n")
        .enumerate()
        .map(|(i, part)| (i, part.split_once('\n').unwrap()))
        .map(|(i, (p1, p2))| (i, (Packet::new(p1), Packet::new(p2))))
        .filter(|(_, (p1, p2))| p1 < p2)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_two(data: &'static str) -> usize {
    let indexes = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(Packet::new)
        .filter(|p| p < &Packet::new("[[6]]"))
        .fold((1, 2), |acc: (usize, usize), p| {
            if p < Packet::new("[[2]]") {
                (acc.0 + 1, acc.1 + 1)
            } else {
                (acc.0, acc.1 + 1)
            }
        });
    indexes.0 * indexes.1
}

mod packet {
    use std::cmp::Ordering;

    #[derive(PartialEq, Eq, Debug)]
    pub struct Packet {
        data: &'static str,
    }

    impl Packet {
        pub fn new(data: &'static str) -> Self {
            Packet { data }
        }
    }

    impl Ord for Packet {
        fn cmp(&self, other: &Self) -> Ordering {
            for (elem_1, elem_2) in self.iter().zip(other.iter()) {
                match (elem_1.starts_with('['), elem_2.starts_with('[')) {
                    (false, false) => match compare_elements(elem_1, elem_2) {
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    },
                    (_, _) => match Packet::new(elem_1).cmp(&Packet::new(elem_2)) {
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    },
                }
            }
            let len_1 = self.data.len() + 2 * usize::from(!self.data.starts_with('['));
            let len_2 = other.data.len() + 2 * usize::from(!other.data.starts_with('['));
            len_1.cmp(&len_2)
        }
    }

    fn compare_elements(a: &'static str, b: &'static str) -> Ordering {
        a.parse::<u32>().unwrap().cmp(&b.parse::<u32>().unwrap())
    }

    impl PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub struct Iter {
        data: &'static str,
        position: usize,
        end: bool,
    }

    impl Iterator for Iter {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.end {
                return None;
            }
            let mut count = 0_u32;
            let start = self.position;
            for (i, char) in self.data[self.position..].char_indices() {
                match (char, count, i) {
                    ('[', _, _) => count += 1,
                    (']', 0, 0) => return None,
                    (']', 0, _) => {
                        self.end = true;
                        return Some(&self.data[start..start + i]);
                    }
                    (']', _, _) => count -= 1,
                    (',', 0, _) => {
                        self.position += i + 1;
                        return Some(&self.data[start..start + i]);
                    }
                    (_, _, _) => (),
                }
            }
            self.end = true;
            Some(&self.data[start..])
        }
    }

    impl Packet {
        fn iter(&self) -> Iter {
            let pos = usize::from(self.data.starts_with('['));
            Iter {
                data: self.data,
                position: pos,
                end: false,
            }
        }
    }

    impl IntoIterator for &Packet {
        type Item = &'static str;
        type IntoIter = Iter;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(13, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(140, part_two(data));
    }
}
