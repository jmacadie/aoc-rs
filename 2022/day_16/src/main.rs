use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data = include_str!("input.txt");
    let (flows, map) = parse_input(data);
    println!("{flows:?}");
    println!("Part 1: {}", part_one(&flows, &map));
    println!("Part 2: {}", part_two(&flows, &map));
}

fn part_one(flows: &Flows, map: &Map) -> u32 {
    let mut human_path = Vec::with_capacity(20);
    let mut elephant_path = Vec::with_capacity(20);
    human_path.push("AA");
    elephant_path.push("AA");
    let (paths, value) = solve([&human_path, &elephant_path], [30, 0], 0, flows, map, 0).unwrap();
    println!("{paths:?}");
    value
}

fn part_two(flows: &Flows, map: &Map) -> u32 {
    let mut human_path = Vec::with_capacity(20);
    let mut elephant_path = Vec::with_capacity(20);
    human_path.push("AA");
    elephant_path.push("AA");
    let (paths, value) = solve([&human_path, &elephant_path], [26, 26], 0, flows, map, 0).unwrap();
    println!("{paths:?}");
    value
}

fn solve(
    paths: [&[&'static str]; 2],
    time: [u32; 2],
    value: u32,
    flows: &Flows,
    map: &Map,
    mut best: u32,
) -> Option<([Path; 2], u32)> {
    if time[1] > time[0] {
        let flipped_paths = [paths[1], paths[0]];
        let flipped_time = [time[1], time[0]];
        return solve(flipped_paths, flipped_time, value, flows, map, best);
    }
    let mut path = paths[0].to_vec();
    let mut out = None;
    let mut priority_list = get_priority_list(paths, time[0], flows, map);
    while let Some((node, new_time, opportunity)) = priority_list.pop() {
        path.push(node);
        let value = value + opportunity;
        if (priority_list.is_empty() || (new_time + time[1] == 0)) && value > best {
            return Some(([path, paths[1].to_vec()], value));
        }
        let updated_paths = [&path, paths[1]];
        let max_time = std::cmp::max(new_time, time[1]);
        if value + remaining_opportunity(updated_paths, max_time, flows, map) <= best {
            path.pop();
            continue;
        }
        let updated_time = [new_time, time[1]];
        if let Some((p, v)) = solve(updated_paths, updated_time, value, flows, map, best) {
            best = v;
            out = Some((p, v));
        }
        path.pop(); // Take this item off to try the next
    }
    out
}

fn remaining_opportunity(paths: [&[&'static str]; 2], time: u32, flows: &Flows, map: &Map) -> u32 {
    get_priority_list(paths, time, flows, map)
        .into_iter()
        .map(|(_, _, v)| v)
        .sum()
}

fn get_priority_list(
    paths: [&[&'static str]; 2],
    time: u32,
    flows: &Flows,
    map: &Map,
) -> Priorities {
    let &node = paths[0].last().unwrap();
    map.get(node)
        .unwrap()
        .iter()
        .filter(|(valve, _)| !paths.into_iter().flatten().contains(valve))
        .map(|(&valve, &dist)| {
            let rem = if time == 0 || dist > time - 1 {
                0
            } else {
                time - dist - 1
            };
            (valve, rem, rem * flows.get(valve).unwrap())
        })
        //.filter(|&(_, _, opportunity)| opportunity > 0)
        .sorted_by_key(|&(_, _, opportunity)| opportunity)
        .collect_vec()
}

type Flows = HashMap<&'static str, u32>;
type UnitMap = HashMap<&'static str, Vec<&'static str>>;
type Map = HashMap<&'static str, HashMap<&'static str, u32>>;
type Priorities = Vec<(&'static str, u32, u32)>;
type Path = Vec<&'static str>;

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
        let (flows, map) = parse_input(data);
        assert_eq!(1707, part_two(&flows, &map));
    }
}
