use itertools::Itertools;
use std::cmp::Ordering;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    data.split("\n\n")
        .enumerate()
        .map(|(i, part)| (i, part.split_once('\n').unwrap()))
        .map(|(i, (p1, p2))| (i, (Packet { data: p1 }, Packet { data: p2 })))
        .filter(|(_, (p1, p2))| p1 < p2)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_two(data: &'static str) -> usize {
    data.split('\n')
        .filter(|l| !l.is_empty())
        .chain("[[2]]".split('\n'))
        .chain("[[6]]".split('\n'))
        .map(|l| Packet { data: l })
        .sorted_unstable()
        .enumerate()
        .filter(|(_, p)| p.data == "[[2]]" || p.data == "[[6]]")
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(PartialEq, Eq, Debug)]
struct Packet {
    data: &'static str,
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_lists(self.data, other.data)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_lists(list_1: &'static str, list_2: &'static str) -> Ordering {
    let parsed_1 = parse_list(list_1);
    let len_1 = parsed_1.len();
    let parsed_2 = parse_list(list_2);
    let len_2 = parsed_2.len();
    if len_1 == 0 || len_2 == 0 {
        return len_1.cmp(&len_2);
    }
    for (elem_1, elem_2) in parsed_1.into_iter().zip(parsed_2.into_iter()) {
        match (elem_1.starts_with('['), elem_2.starts_with('[')) {
            (false, false) => match elem_1
                .parse::<u16>()
                .unwrap()
                .cmp(&elem_2.parse::<u16>().unwrap())
            {
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            },
            (_, _) => match compare_lists(elem_1, elem_2) {
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            },
        }
    }
    len_1.cmp(&len_2)
}

fn parse_list(list: &'static str) -> Vec<&str> {
    let mut count = 0_u32;
    let pad = usize::from(list.starts_with('['));
    let mut start = 0_usize;
    let mut out = Vec::new();
    for (i, char) in list[pad..list.len()].char_indices() {
        match (char, count) {
            ('[', 0) => {
                count = 1;
                start = i;
            }
            ('[', _) => count += 1,
            (']', 0) => add_item(&mut out, list, start + pad, i + pad),
            (']', _) => count -= 1,
            (',', 0) => {
                add_item(&mut out, list, start + pad, i + pad);
                start = i + 1;
            }
            (_, _) => (),
        }
    }
    if pad == 0 {
        add_item(&mut out, list, start, list.len());
    }
    out
}

fn add_item(output: &mut Vec<&'static str>, source: &'static str, start: usize, end: usize) {
    if end > start {
        output.push(&source[start..end]);
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
