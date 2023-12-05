#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use core::panic;
use std::cmp::Ordering;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let (seeds, a, b, c, d, e, f, g) = parse_input(data);
    seeds
        .into_iter()
        .map(|s| a.run(s))
        .map(|s| b.run(s))
        .map(|s| c.run(s))
        .map(|s| d.run(s))
        .map(|s| e.run(s))
        .map(|s| f.run(s))
        .map(|s| g.run(s))
        .min()
        .unwrap()
}

fn part_two(data: &str) -> u32 {
    let (seeds, a, b, c, d, e, f, g) = parse_input(data);
    seeds
        .chunks_exact(2)
        .map(|s| (s[0], s[0] + s[1] - 1))
        .map(|s| a.run_interval(s))
        .map(|s| b.run_intervals(s))
        .map(|s| c.run_intervals(s))
        .map(|s| d.run_intervals(s))
        .map(|s| e.run_intervals(s))
        .map(|s| f.run_intervals(s))
        .map(|s| g.run_intervals(s))
        .map(|v| v.into_iter().map(|(a, _)| a).min().unwrap())
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<u32>, Map, Map, Map, Map, Map, Map, Map) {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .expect("Cannot find seeds in input")
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|num| {
            num.parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert seed {num} to a number"))
        })
        .collect::<Vec<_>>();

    let mut gen_map = |desc: &str| {
        Map::new(
            sections
                .next()
                .unwrap_or_else(|| panic!("Cannot find {desc} map"))
                .split_once('\n')
                .unwrap_or_else(|| panic!("Cannot split the header line from the {desc} map"))
                .1,
        )
    };

    let seed_to_soil = gen_map("seed to soil");
    let soil_to_fertilizer = gen_map("soil to fertilizer");
    let fertilzer_to_water = gen_map("fertilizer to water");
    let water_to_light = gen_map("water to light");
    let light_to_temperature = gen_map("light to temperature");
    let temperature_to_humidity = gen_map("temperature to humidity");
    let humidity_to_location = gen_map("humidity to location");

    (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilzer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    )
}

type Interval = (u32, u32);

struct Map {
    data: Vec<(u32, u32, u32)>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut data = Vec::new();
        data.extend(input.lines().map(|l| {
            let mut parts = l.split_whitespace();
            let (Some(to), Some(from), Some(length), None) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            else {
                unreachable!("{}", format!("Cannot split line into three numbers: {l}"));
            };
            let to = to
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert {to} to a number"));
            let from = from
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert {from} to a number"));
            let length = length
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert {length} to a number"));
            (to, from, length)
        }));
        Self { data }
    }

    fn run(&self, val: u32) -> u32 {
        for &(to, from, length) in &self.data {
            if (from..(from + length)).contains(&val) {
                return val + to - from;
            }
        }
        val
    }

    fn run_intervals(&self, intervals: Vec<Interval>) -> Vec<Interval> {
        let mut output = Vec::new();
        intervals
            .into_iter()
            .for_each(|i| output.extend(self.run_interval(i)));
        output
    }

    fn run_interval(&self, input: Interval) -> Vec<Interval> {
        let mut queue = vec![input];
        let mut mapped = Vec::new();

        for &(to, from, length) in &self.data {
            if queue.is_empty() {
                return mapped;
            }
            queue = Self::run_one_level(&mut mapped, queue, from, from + length - 1, to);
        }

        mapped.extend(queue);
        mapped
    }

    fn run_one_level(
        mapped: &mut Vec<Interval>,
        mut queue: Vec<Interval>,
        from: u32,
        end: u32,
        to: u32,
    ) -> Vec<Interval> {
        let mut next_queue = Vec::new();
        while let Some(inner) = queue.pop() {
            match (
                inner.0.cmp(&from),
                inner.0.cmp(&end),
                inner.1.cmp(&from),
                inner.1.cmp(&end),
            ) {
                (_, _, Ordering::Less, _) | (_, Ordering::Greater, _, _) => {
                    next_queue.push(inner);
                }
                (Ordering::Greater | Ordering::Equal, _, _, Ordering::Less | Ordering::Equal) => {
                    mapped.push((inner.0 + to - from, inner.1 + to - from));
                }
                (Ordering::Less, _, _, Ordering::Less | Ordering::Equal) => {
                    next_queue.push((inner.0, from - 1));
                    mapped.push((to, inner.1 + to - from));
                }
                (Ordering::Greater | Ordering::Equal, _, _, Ordering::Greater) => {
                    mapped.push((inner.0 + to - from, end + to - from));
                    next_queue.push((end + 1, inner.1));
                }
                (Ordering::Less, _, _, Ordering::Greater) => {
                    next_queue.push((inner.0, from - 1));
                    mapped.push((to, end + to - from));
                    next_queue.push((end + 1, inner.1));
                }
            }
        }
        next_queue
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(35, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(46, part_two(data));
    }
}
