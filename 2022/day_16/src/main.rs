use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data = include_str!("input.txt");
    let (flows, map) = parse_input(data);
    println!("Part 1: {}", part_one(&flows, &map));
    println!("Part 2: {}", part_two(data));
}

fn part_one(flows: &Flows, map: &Map) -> u32 {
    let mut curr_path = Vec::with_capacity(20);
    curr_path.push("AA");
    let (_path, value) = solve(&curr_path, 0, 30, flows, map, 0).unwrap();
    //println!("{path:?}");
    value
}

fn solve(
    path: &[&'static str],
    value: u32,
    time: u32,
    flows: &Flows,
    map: &Map,
    mut best: u32,
) -> Option<(Vec<&'static str>, u32)> {
    let mut path = path.to_vec();
    let mut priority_list = get_priority_list(&path, time, flows, map);
    let mut out = None;
    while let Some((node, new_time, opportunity)) = priority_list.pop() {
        path.push(node);
        let value = value + opportunity;
        if priority_list.is_empty() && value > best {
            return Some((path, value));
        }
        if value + remaining_opportunity(&path, new_time, flows, map) <= best {
            path.pop();
            continue;
        }
        if let Some(t) = solve(&path, value, new_time, flows, map, best) {
            best = t.1;
            out = Some(t);
        }
        path.pop(); // Take this item off to try the next
    }
    out
}

fn remaining_opportunity(path: &[&'static str], time: u32, flows: &Flows, map: &Map) -> u32 {
    get_priority_list(path, time, flows, map)
        .into_iter()
        .map(|(_, _, v)| v)
        .sum()
}

fn get_priority_list(
    path: &[&'static str],
    time: u32,
    flows: &Flows,
    map: &Map,
) -> Vec<(&'static str, u32, u32)> {
    let node = path.last().unwrap();
    map.get(node)
        .unwrap()
        .iter()
        .filter(|(valve, _)| !path.contains(valve))
        .map(|(&valve, &dist)| {
            let rem = if time == 0 || dist > time - 1 {
                0
            } else {
                time - dist - 1
            };
            (valve, rem, rem * flows.get(valve).unwrap())
        })
        .sorted_by_key(|&(_, _, opportunity)| opportunity)
        .collect_vec()
}

fn part_two(_data: &str) -> usize {
    0
}

type Flows = HashMap<&'static str, u32>;
type UnitMap = HashMap<&'static str, Vec<&'static str>>;
type Map = HashMap<&'static str, HashMap<&'static str, u32>>;

fn parse_input(data: &'static str) -> (Flows, Map) {
    let mut flows = HashMap::with_capacity(20);
    let mut unit_map = HashMap::with_capacity(100);
    for line in data.lines() {
        let (name, flow, connected) = read_line(line);
        if flow > 0 || name == "AA" {
            flows.insert(name, flow);
        }
        unit_map.insert(name, connected);
    }
    let map = create_map(&unit_map, &flows);
    (flows, map)
}

fn create_map(unit_map: &UnitMap, flows: &Flows) -> Map {
    let mut map = HashMap::with_capacity(20);
    for &valve in flows.keys() {
        let mut valve_map = HashMap::with_capacity(20);
        let mut frontier = vec![valve];
        let mut visited = HashSet::with_capacity(100);
        let mut i = 1;
        while !frontier.is_empty() {
            let mut next_frontier = Vec::new();
            for elem in frontier {
                let connected = unit_map.get(elem).unwrap();
                for &node in connected {
                    if !visited.contains(node) && node != valve {
                        if flows.contains_key(node) {
                            valve_map.insert(node, i);
                        }
                        visited.insert(node);
                        next_frontier.push(node);
                    }
                }
            }
            i += 1;
            frontier = next_frontier;
        }
        map.insert(valve, valve_map);
    }
    map
}

fn read_line(line: &str) -> (&str, u32, Vec<&str>) {
    let (valve, connected) = line.split_once(';').unwrap();
    let (name, flow) = valve.split_once('=').unwrap();

    let name = name
        .trim_start_matches("Valve ")
        .trim_end_matches(" has flow rate");

    let flow = flow.parse().unwrap();

    let connected = connected
        .trim_start_matches(" tunnels lead to valves ")
        .trim_start_matches(" tunnel leads to valve ")
        .split(", ")
        .collect_vec();

    (name, flow, connected)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (flows, map) = parse_input(data);
        assert_eq!(1651, part_one(&flows, &map));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
