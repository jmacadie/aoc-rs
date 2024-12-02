#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(|l| {
            let mut nums = l.split_whitespace().map(|num| num.parse::<u8>().unwrap());
            let mut prev = nums.next().unwrap();
            let curr = nums.next().unwrap();
            let increasing = curr > prev;
            if !right_gap(prev, curr, increasing) {
                return false;
            }
            prev = curr;
            for curr in nums {
                if !right_gap(prev, curr, increasing) {
                    return false;
                }
                prev = curr;
            }
            true
        })
        .filter(|&v| v)
        .count()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .map(|l| {
            let mut numbers = l.split_whitespace().map(|num| num.parse::<u8>().unwrap());
            let (Some(num1), Some(num2), Some(num3), Some(num4)) = (
                numbers.next(),
                numbers.next(),
                numbers.next(),
                numbers.next(),
            ) else {
                unreachable!();
            };
            let mut error = false;
            let (increasing, mut prev, mut prev_prev) = match find_alignment(num1, num2, num3, num4)
            {
                Aligned::Not => return false,
                Aligned::OneError(dir, last) => {
                    error = true;
                    (dir, last, 0)
                }
                Aligned::All(dir) => (dir, num4, num3),
            };
            for curr in numbers {
                if right_gap(prev, curr, increasing) {
                    prev_prev = prev;
                    prev = curr;
                } else {
                    if error {
                        return false;
                    }
                    if right_gap(prev_prev, curr, increasing) {
                        prev = curr;
                        //prev_prev no longer relevant
                    }
                    error = true;
                }
            }
            true
        })
        .filter(|&v| v)
        .count()
}

const fn right_gap(num1: u8, num2: u8, increasing: bool) -> bool {
    if ((num2 < num1) && increasing) || ((num2 > num1) && !increasing) {
        return false;
    }
    let diff = if increasing { num2 - num1 } else { num1 - num2 };
    diff >= 1 && diff <= 3
}

const fn find_alignment(num1: u8, num2: u8, num3: u8, num4: u8) -> Aligned {
    if let Some(increasing) = three_aligned(num1, num2, num3) {
        if right_gap(num3, num4, increasing) {
            return Aligned::All(increasing);
        }
        if right_gap(num2, num4, increasing) {
            return Aligned::OneError(increasing, num4);
        }
        return Aligned::OneError(increasing, num3);
    }
    if let Some(increasing) = three_aligned(num1, num2, num4) {
        return Aligned::OneError(increasing, num4);
    }
    if let Some(increasing) = three_aligned(num1, num3, num4) {
        return Aligned::OneError(increasing, num4);
    }
    if let Some(increasing) = three_aligned(num2, num3, num4) {
        return Aligned::OneError(increasing, num4);
    }
    Aligned::Not
}

enum Aligned {
    All(bool),
    OneError(bool, u8),
    Not,
}

const fn three_aligned(num1: u8, num2: u8, num3: u8) -> Option<bool> {
    if num1 == num2 || num2 == num3 {
        return None;
    }
    let increasing = num2 > num1;
    if !right_gap(num1, num2, increasing) || !right_gap(num2, num3, increasing) {
        return None;
    }
    Some(increasing)
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
        assert_eq!(4, part_two(data));
    }
}
