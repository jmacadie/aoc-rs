#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[allow(clippy::missing_panics_doc)]
pub fn main() {
    let data = include_str!("input.txt");
    let data = data.trim().parse().unwrap();
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

const fn part_one(target: u32) -> u32 {
    let bound = lower_bound_power(target, 2);
    let excess = target - bound;
    2 * excess + 1
}

const fn part_two(target: u32) -> u32 {
    let bound = lower_bound_power(target, 3);
    let excess = target - bound;
    if excess <= bound {
        excess
    } else {
        bound + 2 * (excess - bound)
    }
}

const fn lower_bound_power(target: u32, base: u32) -> u32 {
    let mut n = 1;
    while base.pow(n) < target {
        n += 1;
    }
    base.pow(n - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(3, part_one(5));
    }

    #[test]
    fn two() {
        assert_eq!(2, part_two(5));
    }
}
