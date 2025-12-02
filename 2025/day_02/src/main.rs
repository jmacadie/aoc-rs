#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<Range>().unwrap())
        .flat_map(|r| r.find_invalid())
        .sum()
}

fn part_two(data: &str) -> usize {
    data.trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<Range>().unwrap())
        .flat_map(|r| r.find_invalid_2())
        .sum()
}

fn make_invalid_num(base: usize, repeats: u32, base_len: u32) -> usize {
    (0..repeats).fold(0, |acc, _| acc * 10_usize.pow(base_len) + base)
}
struct Range {
    start: usize,
    end: usize,
    start_len: u32,
    end_len: u32,
}

impl Range {
    fn find_invalid(&self) -> Vec<usize> {
        let mut out = Vec::new();
        self.find_from_repeats(2, &mut out);
        out
    }

    fn find_invalid_2(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for repeats in 2..=self.end_len {
            self.find_from_repeats(repeats, &mut out);
        }
        out
    }

    fn find_from_repeats(&self, repeats: u32, out: &mut Vec<usize>) {
        if let Some((end, e_len)) = self.adj_down(repeats) {
            let (start, s_len) = self.adj_up(repeats);
            if start > end {
                return;
            }
            assert_eq!(s_len, e_len);

            let base_len = s_len / repeats;
            let start_base = start / 10_usize.pow(s_len - base_len);
            let end_base = end / 10_usize.pow(e_len - base_len);
            let start_num = make_invalid_num(start_base, repeats, base_len);
            let end_num = make_invalid_num(end_base, repeats, base_len);
            let valid_start = start_num >= start;
            let valid_end = end_num <= end;

            if start_base == end_base {
                if valid_start && valid_end && !out.contains(&start_num) {
                    out.push(start_num);
                }
                return;
            }

            if valid_start && !out.contains(&start_num) {
                out.push(start_num);
            }

            for base in start_base + 1..end_base {
                let num = make_invalid_num(base, repeats, base_len);
                if !out.contains(&num) {
                    out.push(num);
                }
            }

            if valid_end && !out.contains(&end_num) {
                out.push(end_num);
            }
        }
    }

    fn adj_up(&self, repeats: u32) -> (usize, u32) {
        if self.start_len.is_multiple_of(repeats) {
            return (self.start, self.start_len);
        }
        (self.start_len + 1..)
            .take(
                repeats
                    .try_into()
                    .expect("This length definitely can be a usize"),
            )
            .find(|l| l % repeats == 0)
            .map(|l| (10_usize.pow(l - 1), l))
            .expect("Can always find a multiple of repeats if we look high enough")
    }

    fn adj_down(&self, repeats: u32) -> Option<(usize, u32)> {
        if self.end_len.is_multiple_of(repeats) {
            return Some((self.end, self.end_len));
        }
        (repeats..self.end_len)
            .rev()
            .find(|l| l % repeats == 0)
            .map(|l| (10_usize.pow(l) - 1, l))
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or("No '-' delimeter")?;
        let start_len =
            u32::try_from(start.len()).map_err(|_| "length of start won't fit in a u32!!!")?;
        let end_len =
            u32::try_from(end.len()).map_err(|_| "length of end won't fit in a u32!!!")?;
        let start = start
            .parse::<usize>()
            .map_err(|_| "start cannot be parsed")?;
        let end = end.parse::<usize>().map_err(|_| "end cannot be parsed")?;
        Ok(Self {
            start,
            end,
            start_len,
            end_len,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(1_227_775_554, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(4_174_379_265, part_two(data));
    }
}
