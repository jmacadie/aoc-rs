#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> i32 {
    data.lines().map(oasis_next).sum()
}

fn part_two(data: &str) -> i32 {
    data.lines().map(oasis_prev).sum()
}

fn oasis_next(line: &str) -> i32 {
    let count = line.split_whitespace().count();
    line.split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .rev()
        .enumerate()
        .fold(0, |acc, (i, val)| {
            if i & 1 == 0 {
                acc + combin(i, count) * val
            } else {
                acc - combin(i, count) * val
            }
        })
}

fn oasis_prev(line: &str) -> i32 {
    let count = line.split_whitespace().count();
    line.split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .enumerate()
        .fold(0, |acc, (i, val)| {
            if i & 1 == 0 {
                acc + combin(i, count) * val
            } else {
                acc - combin(i, count) * val
            }
        })
}

fn combin(num: usize, base: usize) -> i32 {
    const COMBIN_21: [i32; 21] = [
        21, 210, 1_330, 5_985, 20_349, 54_264, 116_280, 203_490, 293_930, 352_716, 352_716,
        293_930, 203_490, 116_280, 54_264, 20_349, 5_985, 1_330, 210, 21, 1,
    ];
    const COMBIN_6: [i32; 6] = [6, 15, 20, 15, 6, 1];
    match base {
        6 => COMBIN_6[num],
        21 => COMBIN_21[num],
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(114, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_two(data));
    }
}
