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

const fn part_two(_data: &str) -> usize {
    0
}

struct Range {
    start: usize,
    end: usize,
    start_len: u32,
    end_len: u32,
}

impl Range {
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
        assert_eq!(0, part_two(data));
    }
}
