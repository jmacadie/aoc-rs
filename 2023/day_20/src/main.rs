#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod module;
mod pulse;
mod state;
mod system;

use module::modules::Modules;
use pulse::Pulses;
use state::records::Records;
use state::State;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<59>(data));
    println!("Part 2: {}", part_two::<59>(data));
}

fn part_one<const N: usize>(data: &'static str) -> usize {
    let mut system = system::System::<N>::new(data);
    system.run()
}

fn part_two<const N: usize>(data: &'static str) -> usize {
    let mut system = system::System::<N>::new(data);
    system.find_output_cycle()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_a() {
        let data = include_str!("test_a.txt");
        assert_eq!(32_000_000, part_one::<6>(data));
    }

    #[test]
    fn one_b() {
        let data = include_str!("test_b.txt");
        assert_eq!(11_687_500, part_one::<6>(data));
    }
}
