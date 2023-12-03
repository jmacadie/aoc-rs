#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[allow(clippy::large_stack_frames)]
pub fn main() {
    let data = include_str!("input.txt");
    let (parts, gears) = find_parts::<140>(data.as_bytes());
    println!("Part 1: {parts}");
    println!("Part 2: {}", part_two(gears));
}

fn part_two<const R: usize>(gears: [[GearParts; R]; R]) -> u32 {
    gears.iter().flatten().map(GearParts::gear_ratio).sum()
}

fn find_parts<const R: usize>(data: &[u8]) -> (u32, [[GearParts; R]; R]) {
    let mut acc = 0;
    let mut gears = [[GearParts::None; R]; R];
    let mut part = 0;
    let mut start = None;
    for (idx, ch) in data.iter().enumerate() {
        match (start, ch.is_ascii_digit()) {
            (Some(_), true) => {
                // In a number sting & continuing
                part *= 10;
                part += u32::from(ch - b'0');
            }
            (Some(s), false) => {
                // In a number sting & ended
                if has_symbol_adjacent::<R>(data, s, idx - 1) {
                    acc += part;
                    check_for_gears(&mut gears, data, s, idx - 1, part);
                }
                part = 0;
                start = None;
            }
            (None, true) => {
                // Not in a number but just started
                part = u32::from(ch - b'0');
                start = Some(idx);
            }
            (None, false) => {} // Not in a number & not started one
        }
    }
    (acc, gears)
}

fn has_symbol_adjacent<const R: usize>(data: &[u8], start: usize, end: usize) -> bool {
    (start > 0 && is_symbol(data, start - 1))
        || (end < data.len() && is_symbol(data, end + 1))
        || (start > R && ((start - R - 2)..=(end - R)).any(|i| is_symbol(data, i)))
        || (start + R < data.len() && ((start + R)..=(end + R + 2)).any(|i| is_symbol(data, i)))
}

const fn is_symbol(data: &[u8], idx: usize) -> bool {
    let val = data[idx];
    !(val.is_ascii_digit() || val == b'.' || val == b'\n')
}

fn check_for_gears<const R: usize>(
    gears: &mut [[GearParts; R]; R],
    data: &[u8],
    start: usize,
    end: usize,
    part: u32,
) {
    if start > 0 {
        check_for_gear(gears, data, start - 1, part);
    }
    if end < data.len() {
        check_for_gear(gears, data, end + 1, part);
    }
    if start > R {
        for idx in (start - R - 2)..=(end - R) {
            check_for_gear(gears, data, idx, part);
        }
    }
    if start + R < data.len() {
        for idx in (start + R)..=(end + R + 2) {
            check_for_gear(gears, data, idx, part);
        }
    }
}

fn check_for_gear<const R: usize>(
    gears: &mut [[GearParts; R]; R],
    data: &[u8],
    idx: usize,
    part: u32,
) {
    if data[idx] == b'*' {
        let row = idx % (R + 1);
        let col = idx / (R + 1);
        gears[row][col].add(part);
    }
}

#[derive(Clone, Copy)]
enum GearParts {
    None,
    One(u32),
    Two((u32, u32)),
    Overflow,
}

impl GearParts {
    fn add(&mut self, val: u32) {
        match *self {
            Self::None => *self = Self::One(val),
            Self::One(prev) => *self = Self::Two((prev, val)),
            Self::Two(_) => *self = Self::Overflow,
            Self::Overflow => {}
        }
    }

    const fn gear_ratio(&self) -> u32 {
        match *self {
            Self::Two((a, b)) => a * b,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (parts, _) = find_parts::<10>(data.as_bytes());
        assert_eq!(4361, parts);
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (_, gears) = find_parts::<10>(data.as_bytes());
        assert_eq!(467_835, part_two::<10>(gears));
    }
}
