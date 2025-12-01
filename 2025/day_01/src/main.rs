#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    gen_turns(data)
        .scan(50, |pos, rot| {
            *pos = (*pos + rot).rem_euclid(100);
            Some(*pos)
        })
        .filter(|&pos| pos == 0)
        .count()
}

fn part_two(data: &str) -> i32 {
    gen_turns(data)
        .scan(50, |pos, rot| {
            let start = *pos;
            let passes = count_passes_over_zero(start, rot);
            *pos = (start + rot).rem_euclid(100);
            Some(passes)
        })
        .sum()
}

fn gen_turns(data: &str) -> impl Iterator<Item = i32> {
    data.lines().map(|rot| {
        let (dir, num_str) = rot.split_at(1);
        let num = num_str.parse::<i32>().unwrap();
        match dir {
            "R" => num,
            "L" => -num,
            _ => unreachable!(),
        }
    })
}

const fn count_passes_over_zero(start: i32, rot: i32) -> i32 {
    let raw = start + rot;
    let end = raw.rem_euclid(100);
    let mut passes = (end - raw).abs() / 100;
    if start == 0 && raw < 0 {
        passes -= 1;
    }
    if end == 0 && raw <= 0 {
        passes += 1;
    }
    passes
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(3, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(6, part_two(data));
    }
}
