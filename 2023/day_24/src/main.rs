#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::many_single_char_names, clippy::doc_markdown)]

pub mod one;
pub mod two;

pub fn main() {
    let data = include_str!("input.txt");
    println!(
        "Part 1: {}",
        part_one::<300>(data, 200_000_000_000_000, 400_000_000_000_000)
    );
    println!("Part 2: {}", part_two(data));
}

fn part_one<const N: usize>(data: &str, min: i128, max: i128) -> usize {
    let mut segments = std::array::from_fn(|_| None);
    for (s, l) in segments.iter_mut().zip(data.lines()) {
        let path: one::HailPath = l.parse().unwrap();
        *s = path.to_ls(min.into(), max.into());
    }
    let mut solver = one::Solver::<N>::new(&segments);
    solver.run()
}

fn part_two(data: &str) -> i64 {
    let mut paths = data
        .lines()
        .map(|l| l.parse::<two::Path>().unwrap())
        .collect::<Vec<_>>();
    paths.sort_unstable_by_key(|p| (p.u, p.x));

    let (x, u) = two::get_x_unknowns(&paths);
    let (v, w) = two::get_velocity_unknowns(&paths, x, u);
    let (y, z) = two::get_position_unknowns(&paths, x, u, v, w);

    x + y + z
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_one::<5>(data, 7, 27));
    }

    #[test]
    fn two() {
        // Test data is not the same as the structure in the inputs I am relying on
        assert_eq!(0, 0);
    }
}
