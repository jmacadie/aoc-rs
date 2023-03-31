#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{BinaryHeap, HashSet};

pub fn main() {
    println!("Part 1: {}", part_one::<31, 39, 1352>());
    println!("Part 2: {}", part_two::<31, 39, 1352>());
}

fn part_one<const X: u32, const Y: u32, const M: u32>() -> u32 {
    solve::<X, Y, M>()
}

fn part_two<const X: u32, const Y: u32, const M: u32>() -> u32 {
    squares_within::<X, Y, M>(50)
}

fn solve<const X: u32, const Y: u32, const M: u32>() -> u32 {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    let s = State::<X, Y, M>::new(SearchType::AStar);

    visited.insert(s.location);
    for m in s.valid_moves() {
        heap.push(m);
    }

    while let Some(s) = heap.pop() {
        if s.location.x == X && s.location.y == Y {
            return s.step;
        }
        if visited.contains(&s.location) {
            continue;
        }
        visited.insert(s.location);
        for m in s.valid_moves() {
            heap.push(m);
        }
    }
    0
}

fn squares_within<const X: u32, const Y: u32, const M: u32>(limit: u32) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    let s = State::<X, Y, M>::new(SearchType::Bfs);

    visited.insert(s.location);
    for m in s.valid_moves() {
        heap.push(m);
    }

    while let Some(s) = heap.pop() {
        if s.step > limit {
            return visited.len().try_into().unwrap();
        }
        if visited.contains(&s.location) {
            continue;
        }
        visited.insert(s.location);
        for m in s.valid_moves() {
            heap.push(m);
        }
    }
    0
}

#[derive(PartialEq, Eq)]
struct State<const X: u32, const Y: u32, const M: u32> {
    step: u32,
    heuristic: u32,
    location: Point,
    search: SearchType,
}

impl<const X: u32, const Y: u32, const M: u32> State<X, Y, M> {
    fn new(search: SearchType) -> Self {
        Self::from_parts(0, Point { x: 1, y: 1 }, search)
    }

    fn from_parts(step: u32, location: Point, search: SearchType) -> Self {
        let mut output = Self {
            step,
            heuristic: 0,
            location,
            search,
        };
        output.compute_heuristic();
        output
    }

    fn compute_heuristic(&mut self) {
        self.heuristic = match self.search {
            SearchType::AStar => {
                let dx = if self.location.x > X {
                    self.location.x - X
                } else {
                    X - self.location.x
                };
                let dy = if self.location.y > Y {
                    self.location.y - Y
                } else {
                    Y - self.location.y
                };
                self.step + dx + dy
            }
            SearchType::Bfs => self.step,
        }
    }

    fn valid_moves(&self) -> Vec<Self> {
        let mut moves = Vec::new();
        if self.location.x > 0 {
            self.try_add_valid(
                Point {
                    x: self.location.x - 1,
                    y: self.location.y,
                },
                &mut moves,
            );
        }
        if self.location.y > 0 {
            self.try_add_valid(
                Point {
                    x: self.location.x,
                    y: self.location.y - 1,
                },
                &mut moves,
            );
        }
        self.try_add_valid(
            Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
            &mut moves,
        );
        self.try_add_valid(
            Point {
                x: self.location.x,
                y: self.location.y + 1,
            },
            &mut moves,
        );
        moves
    }

    fn try_add_valid(&self, p: Point, moves: &mut Vec<Self>) {
        if Self::is_valid(p) {
            moves.push(Self::from_parts(&self.step + 1, p, self.search));
        }
    }

    const fn is_valid(p: Point) -> bool {
        let test = (p.x * p.x) + (3 * p.x) + (2 * p.x * p.y) + p.y + (p.y * p.y) + M;
        test.count_ones() & 1 == 0
    }
}

impl<const X: u32, const Y: u32, const M: u32> Ord for State<X, Y, M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heuristic
            .cmp(&self.heuristic)
            .then_with(|| self.step.cmp(&other.step))
    }
}

impl<const X: u32, const Y: u32, const M: u32> PartialOrd for State<X, Y, M> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchType {
    AStar,
    Bfs,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(11, part_one::<7, 4, 10>());
    }
}
