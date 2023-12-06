#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let (time, distance) = data.split_once('\n').unwrap();
    let time_iter = time
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let distance_iter = distance
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    time_iter
        .zip(distance_iter)
        .map(|(t, d)| quadratic_solver(t, d))
        .product()
}

fn part_two(data: &str) -> u32 {
    let (time, distance) = data.split_once('\n').unwrap();
    let time = time
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .fold(0, |acc, x| acc * next_power_of_10(x) + x);
    let distance = distance
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .fold(0, |acc, x| acc * next_power_of_10(x) + x);
    quadratic_solver(time, distance)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn next_power_of_10(num: u64) -> u64 {
    10_u64.pow(((num as f64).log10().ceil()) as u32)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn quadratic_solver(time: u64, distance: u64) -> u32 {
    let sqrt = ((time * time - 4 * distance) as f64).sqrt() - 0.00001;
    let floor = ((time as f64 - sqrt) / 2_f64) as u32;
    let ceiling = ((time as f64 + sqrt) / 2_f64) as u32;
    ceiling - floor
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(288, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(71_503, part_two(data));
    }
}
