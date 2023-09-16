#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let components: Vec<Component> = data.lines().map(|l| l.parse().unwrap()).collect();
    find_strongest_bridge(0, 0, 0, &components)
}

fn part_two(data: &str) -> u32 {
    let components: Vec<Component> = data.lines().map(|l| l.parse().unwrap()).collect();
    find_longest_bridge(0, (0, 0), (0, 0), &components).1
}

fn find_strongest_bridge(
    current_end: Port,
    current_bridge: u32,
    found_max: u32,
    remaining: &[Component],
) -> u32 {
    let mut max = found_max;
    for (idx, c) in remaining.iter().enumerate() {
        if let Some(end) = c.joins(current_end) {
            let strength = current_bridge + c.strength();
            max = std::cmp::max(max, strength);
            let mut remaining = remaining.to_vec();
            remaining.remove(idx);
            max = std::cmp::max(max, find_strongest_bridge(end, strength, max, &remaining));
        }
    }
    max
}

fn find_longest_bridge(
    current_end: Port,
    current_bridge: (u8, u32),
    found_max: (u8, u32),
    remaining: &[Component],
) -> (u8, u32) {
    let mut max = found_max;
    for (idx, c) in remaining.iter().enumerate() {
        if let Some(end) = c.joins(current_end) {
            let length = current_bridge.0 + 1;
            let strength = current_bridge.1 + c.strength();
            let new_bridge = (length, strength);
            max = longest_bridge(max, new_bridge);
            let mut remaining = remaining.to_vec();
            remaining.remove(idx);
            let new = find_longest_bridge(end, new_bridge, max, &remaining);
            max = longest_bridge(max, new);
        }
    }
    max
}

fn longest_bridge(a: (u8, u32), b: (u8, u32)) -> (u8, u32) {
    match a.0.cmp(&b.0) {
        std::cmp::Ordering::Less => b,
        std::cmp::Ordering::Greater => a,
        std::cmp::Ordering::Equal => match a.1.cmp(&b.1) {
            std::cmp::Ordering::Less => b,
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => a,
        },
    }
}

type Port = u8;

#[derive(Debug, Clone, Copy)]
struct Component {
    ports: (Port, Port),
}

impl Component {
    const fn joins(self, other: Port) -> Option<Port> {
        if self.ports.0 == other {
            return Some(self.ports.1);
        }
        if self.ports.1 == other {
            return Some(self.ports.0);
        }
        None
    }

    fn strength(self) -> u32 {
        u32::from(self.ports.0) + u32::from(self.ports.1)
    }
}

impl FromStr for Component {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('/')
            .ok_or_else(|| format!("Cannot split {s} with a '/'"))?;
        let a = a
            .parse()
            .map_err(|_| format!("Cannot parse {a} into a u8"))?;
        let b = b
            .parse()
            .map_err(|_| format!("Cannot parse {b} into a u8"))?;
        Ok(Self { ports: (a, b) })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(31, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(19, part_two(data));
    }
}
