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
    fn find_invalid_2(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for base_len in 1..=(self.end_len / 2) {
            if let Some((end, e_len)) = self.adj_down(base_len) {
                let (start, s_len) = self.adj_up(base_len);
                if start > end {
                    continue;
                }

                let start_base = start / 10_usize.pow(s_len - base_len);
                let end_base = end / 10_usize.pow(e_len - base_len);
                let s_repeats = s_len / base_len;
                let e_repeats = e_len / base_len;
                let start_num = make_invalid_num(start_base, s_repeats, base_len);
                let end_num = make_invalid_num(end_base, e_len / base_len, base_len);
                let valid_start = start_num >= start;
                let valid_end = end_num <= end;

                if start_base == end_base {
                    if valid_start && valid_end && !out.contains(&start_num) {
                        out.push(start_num);
                    }
                    continue;
                }

                if valid_start && !out.contains(&start_num) {
                    out.push(start_num);
                }

                assert!(e_repeats - s_repeats < 2);
                if s_repeats == e_repeats {
                    for base in start_base + 1..end_base {
                        let num = make_invalid_num(base, s_repeats, base_len);
                        if !out.contains(&num) {
                            out.push(num);
                        }
                    }
                } else if s_repeats == e_repeats - 1 {
                    for base in start_base + 1..10_usize.pow(base_len) {
                        let num = make_invalid_num(base, s_repeats, base_len);
                        if !out.contains(&num) {
                            out.push(num);
                        }
                    }
                    for base in 10_usize.pow(base_len - 1)..end_base {
                        let num = make_invalid_num(base, e_repeats, base_len);
                        if !out.contains(&num) {
                            out.push(num);
                        }
                    }
                } else {
                    unreachable!()
                }

                if valid_end && !out.contains(&end_num) {
                    out.push(end_num);
                }
            }
        }
        out
    }

    fn adj_up(&self, base_len: u32) -> (usize, u32) {
        // Can't have single digit numbers
        if self.start < 10 && base_len == 1 {
            return (10, 2);
        }
        if self.start_len.is_multiple_of(base_len) {
            return (self.start, self.start_len);
        }
        (self.start_len + 1..)
            .take(
                base_len
                    .try_into()
                    .expect("This length definitely can be a usize"),
            )
            .find(|l| l % base_len == 0)
            .map(|l| (10_usize.pow(l - 1), l))
            .expect("Can always find a multiple of base len if we look high enough")
    }

    fn adj_down(&self, base_len: u32) -> Option<(usize, u32)> {
        if self.end_len.is_multiple_of(base_len) {
            return Some((self.end, self.end_len));
        }
        (base_len + 1..self.end_len)
            .rev()
            .find(|l| l % base_len == 0)
            .map(|l| (10_usize.pow(l) - 1, l))
    }

    fn find_invalid(&self) -> Vec<usize> {
        let mut output = Vec::new();
        let (start, start_len) = self.adj_start();
        let (end, end_len) = self.adj_end();
        if end <= start {
            return output;
        }

        let start_base = Self::base(start, start_len);
        let end_base = Self::base(end, end_len);
        let valid_start = Self::above_base(start, start_len, start_base);
        let valid_end = Self::below_base(end, end_len, end_base);
        // println!("{start}:{start_base} - {valid_start}, {end}:{end_base} - {valid_end}");

        if start_base == end_base {
            if valid_start && valid_end {
                output.push(Self::make_invalid_num(start_base, start_len));
            }
            return output;
        }

        if valid_start {
            output.push(Self::make_invalid_num(start_base, start_len));
        }
        if valid_end {
            output.push(Self::make_invalid_num(end_base, end_len));
        }
        for base in (start_base + 1)..end_base {
            output.push(Self::make_invalid_num(base, start_len));
        }

        output
    }

    const fn make_invalid_num(base: usize, len: u32) -> usize {
        base + base * 10_usize.pow(len / 2)
    }

    const fn above_base(num: usize, len: u32, base: usize) -> bool {
        let rem = num - base * 10_usize.pow(len / 2);
        rem <= base
    }

    const fn below_base(num: usize, len: u32, base: usize) -> bool {
        let rem = num - base * 10_usize.pow(len / 2);
        rem >= base
    }

    const fn base(num: usize, len: u32) -> usize {
        let divisor = 10_usize.pow(len / 2);
        num / divisor
    }

    const fn adj_start(&self) -> (usize, u32) {
        if self.start_len & 1 == 0 {
            return (self.start, self.start_len);
        }
        (10_usize.pow(self.start_len), self.start_len + 1)
    }

    const fn adj_end(&self) -> (usize, u32) {
        if self.end_len & 1 == 0 {
            return (self.end, self.end_len);
        }
        (10_usize.pow(self.end_len - 1) - 1, self.end_len - 1)
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
