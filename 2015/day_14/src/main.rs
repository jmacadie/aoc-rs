#![warn(clippy::all, clippy::pedantic)]
use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data, 2_503));
    println!("Part 2: {}", part_two(data, 2_503));
}

fn part_one(data: &str, time: u32) -> u32 {
    data.lines()
        .map(read_line)
        .map(|r| distance_travelled(r, time))
        .max()
        .unwrap()
}

fn part_two(data: &str, time: u32) -> u32 {
    let mut reindeer = [(0_u32, 0_u32, 0_u32, 0_u32); 9];
    let num = data.lines().count();
    for (line, store) in data.lines().zip(reindeer.iter_mut()) {
        let (s, t, r) = read_line(line);
        *store = (s, t, r, 0);
    }
    for curr_time in 1..=time {
        for winner in reindeer
            .iter()
            .take(num)
            .enumerate()
            .map(|(idx, &(s, t, r, _))| (idx, distance_travelled((s, t, r), curr_time)))
            .max_set_by_key(|&(_, d)| d)
            .iter()
            .map(|&(idx, _)| idx)
        {
            reindeer[winner].3 += 1;
        }
    }
    reindeer.into_iter().map(|(_, _, _, d)| d).max().unwrap()
}

fn distance_travelled(reindeer: (u32, u32, u32), travel_time: u32) -> u32 {
    let (speed, time, rest) = reindeer;
    let cycle_dist = speed * time;
    let cycle = time + rest;
    let whole_cycles = travel_time / cycle;
    let rem = std::cmp::min(travel_time % cycle, time);
    whole_cycles * cycle_dist + rem * speed
}

fn read_line(line: &str) -> (u32, u32, u32) {
    let (_, rest) = line.split_once(" can fly ").unwrap();
    let (speed, rest) = rest.split_once(" km/s for ").unwrap();
    let (time, rest) = rest
        .split_once(" seconds, but then must rest for ")
        .unwrap();
    let (rest, _) = rest.split_once(" seconds").unwrap();

    let speed = speed.parse().unwrap();
    let time = time.parse().unwrap();
    let rest = rest.parse().unwrap();

    (speed, time, rest)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(1_120, part_one(data, 1_000));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(689, part_two(data, 1_000));
    }
}
