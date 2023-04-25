#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two::<26, 38>(data));
}

fn part_one(data: &str) -> usize {
    let nodes: Vec<Node> = data.lines().skip(2).map(|l| l.parse().unwrap()).collect();
    let max_avail = nodes.iter().map(|n| n.avail).max().unwrap();
    let min_used = nodes
        .iter()
        .map(|n| n.used)
        .filter(|&u| u > 0)
        .min()
        .unwrap();
    let move_candidates = nodes.iter().filter(|&n| n.used <= max_avail && n.used > 0);
    let recieve_candidates = nodes.iter().filter(|&n| n.avail >= min_used);
    move_candidates
        .cartesian_product(recieve_candidates)
        .filter(|&(m, r)| m.used <= r.avail)
        .count()
}

fn part_two<const R: usize, const C: usize>(data: &str) -> usize {
    let nodes: Vec<Node> = data.lines().skip(2).map(|l| l.parse().unwrap()).collect();
    let fs = FileSystem::<R, C>::new(&nodes);

    // First move the empty node up to the node adjacent to the top-right node, using a BFS
    let step_1 = fs.move_empty_to_top_row();

    // Need to assert the top two rows are clean or the next logic won't work
    assert!(fs.clean_row(0));
    assert!(fs.clean_row(1));

    // Given the top two rows are clean, we can move the target node one space to the left with a
    // series of 5 moves:
    //
    // 0) . _ T
    //    . . .
    //
    // 1) . T _
    //    . . .
    //
    // 2) . T .
    //    . . _
    //
    // 3) . T .
    //    . _ .
    //
    // 4) . T .
    //    _ . .
    //
    // 5) _ T .
    //    . . .
    //
    // Given we have a file system C nodes wide, we need to do C - 2 multiples of this pattern.
    // This will move the target node over to one space to the right of the goal node, with the
    // empty node on the goal node.
    let step_2 = 5 * (C - 2);

    // From this position we only need a single further move to reach the target state
    step_1 + step_2 + 1
}

#[derive(Debug)]
struct Node {
    x: u8,
    y: u8,
    used: u16,
    avail: u16,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (Some(name), Some(_size), Some(used), Some(avail), Some(_used_pcnt), None) = (parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) else {
            return Err(format!("Filesystem text incorrectly formatted: {s}"));
        };
        let (_, temp) = name.split_once("-x").ok_or(format!(
            "Badly formatted node name, expecting a '-x': {name}"
        ))?;
        let (x, y) = temp.split_once("-y").ok_or(format!(
            "Badly formatted node name, expecting a '-y': {name}"
        ))?;
        let x = x
            .parse()
            .map_err(|_| format!("x location is not a number: {x}"))?;
        let y = y
            .parse()
            .map_err(|_| format!("y location is not a number: {y}"))?;
        let used = used
            .trim_end_matches('T')
            .parse()
            .map_err(|_| format!("used is incorrectly formatted: {used}"))?;
        let avail = avail
            .trim_end_matches('T')
            .parse()
            .map_err(|_| format!("avail is incorrectly formatted: {avail}"))?;
        Ok(Self { x, y, used, avail })
    }
}

#[derive(Debug)]
struct FileSystem<const R: usize, const C: usize>([[NodeType; C]; R]);

impl<const R: usize, const C: usize> FileSystem<R, C> {
    fn new(nodes: &[Node]) -> Self {
        let max_avail = nodes.iter().map(|n| n.avail).max().unwrap();
        let mut data = [[NodeType::Interchangable; C]; R];
        for node in nodes {
            data[node.y as usize][node.x as usize] = match (node.used, node.used.cmp(&max_avail)) {
                (0, _) => NodeType::Empty,
                (_, Ordering::Greater) => NodeType::Full,
                (_, Ordering::Less | Ordering::Equal) => NodeType::Interchangable,
            };
        }
        Self(data)
    }

    fn get_empty(&self) -> Option<Point> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if node == &NodeType::Empty {
                    return Some(Point { x, y });
                }
            }
        }
        None
    }

    fn move_empty_to_top_row(&self) -> usize {
        let mut visited = HashSet::new();
        let mut frontier = Vec::new();
        let mut next_frontier = Vec::new();
        let mut step = 0;

        let Some(start) = self.get_empty() else {
            unreachable!();
        };
        visited.insert(start);
        frontier.push(start);

        while !frontier.is_empty() {
            step += 1;
            while let Some(next) = frontier.pop() {
                for n in Self::get_neighbours(next) {
                    if n.y == 0 && n.x == C - 2 {
                        return step;
                    }
                    if self.0[n.y][n.x] == NodeType::Full || visited.contains(&n) {
                        continue;
                    }
                    visited.insert(n);
                    next_frontier.push(n);
                }
            }
            frontier.append(&mut next_frontier);
        }

        0
    }

    fn get_neighbours(location: Point) -> Vec<Point> {
        let mut neighbours = Vec::new();
        if location.x > 0 {
            neighbours.push(Point {
                x: location.x - 1,
                y: location.y,
            });
        }
        if location.y > 0 {
            neighbours.push(Point {
                x: location.x,
                y: location.y - 1,
            });
        }
        if location.x < C - 1 {
            neighbours.push(Point {
                x: location.x + 1,
                y: location.y,
            });
        }
        if location.y < R - 1 {
            neighbours.push(Point {
                x: location.x,
                y: location.y + 1,
            });
        }
        neighbours
    }

    fn clean_row(&self, row: usize) -> bool {
        !self
            .0
            .iter()
            .nth(row)
            .map_or(true, |r| r.iter().any(|&n| n == NodeType::Full))
    }
}

impl<const R: usize, const C: usize> Display for FileSystem<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for node in row {
                write!(f, "{node}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Empty,
    Interchangable,
    Full,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "_")?,
            Self::Interchangable => write!(f, ".")?,
            Self::Full => write!(f, "#")?,
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(7, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(7, part_two::<3, 3>(data));
    }
}
