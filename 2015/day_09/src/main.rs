#![warn(clippy::all, clippy::pedantic)]
use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<8>(data));
    println!("Part 2: {}", part_two::<8>(data));
}

fn part_one<const N: usize>(data: &'static str) -> u16 {
    let (map, dests) = build_map::<N>(data);

    let mut min = u16::MAX;
    for path in dests.into_iter().permutations(N) {
        let mut path_dist = 0_u16;
        for (a, b) in path.into_iter().tuple_windows() {
            path_dist += get_dist(a, b, &map);
            if path_dist > min {
                break;
            }
        }
        min = std::cmp::min(min, path_dist);
    }
    min
}

fn part_two<const N: usize>(data: &'static str) -> u16 {
    let (map, dests) = build_map::<N>(data);

    let mut maximum = 0_u16;
    for path in dests.into_iter().permutations(N) {
        let mut path_dist = 0_u16;
        for (a, b) in path.into_iter().tuple_windows() {
            path_dist += get_dist(a, b, &map);
        }
        maximum = std::cmp::max(maximum, path_dist);
    }
    maximum
}

type Map = [(&'static str, &'static str, u16); 28];
type Destinations<const N: usize> = [&'static str; N];

fn build_map<const N: usize>(data: &'static str) -> (Map, Destinations<N>) {
    let mut map = Map::default();
    for (i, line) in data.lines().enumerate() {
        map[i] = read_line(line);
    }

    let mut dest = [""; N];
    for (i, loc) in map
        .into_iter()
        .flat_map(|(a, b, _)| [a, b])
        .filter(|a| a != &"")
        .sorted_unstable()
        .dedup()
        .enumerate()
    {
        dest[i] = loc;
    }

    (map, dest)
}

fn read_line(line: &'static str) -> (&'static str, &'static str, u16) {
    let (route, dist) = line.split_once(" = ").unwrap();
    let (a, b) = route.split_once(" to ").unwrap();
    let dist = dist.parse().unwrap();
    (a, b, dist)
}

fn get_dist(a: &'static str, b: &'static str, map: &Map) -> u16 {
    for &(ma, mb, d) in map {
        if (ma == a && mb == b) || (ma == b && mb == a) {
            return d;
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(605, part_one::<3>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(982, part_two::<3>(data));
    }
}
