#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<141>(data));
    println!("Part 2: {}", part_two::<141, 141>(data));
}

fn part_one<const N: usize>(data: &str) -> usize {
    pathfind::<N, N>(data, 1, 3)
}

fn part_two<const R: usize, const C: usize>(data: &str) -> usize {
    pathfind::<R, C>(data, 4, 10)
}

fn pathfind<const R: usize, const C: usize>(
    data: &str,
    min_turn: usize,
    max_straight: usize,
) -> usize {
    let cost_map = parse_cost_map::<R, C>(data);
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::with_capacity(20 * R * C);

    let s = State::<R, C>::new(&cost_map, min_turn, max_straight);
    visited.insert((s.location, s.facing, s.straight_line_count));

    s.add_next_states(&mut heap);

    while let Some(s) = heap.pop() {
        if s.location.row == R - 1 && s.location.col == C - 1 {
            return s.total_cost;
        }
        if (min_turn..s.straight_line_count).any(|c| visited.contains(&(s.location, s.facing, c)))
            || !visited.insert((s.location, s.facing, s.straight_line_count))
        {
            continue;
        }
        s.add_next_states(&mut heap);
    }
    0
}

fn parse_cost_map<const R: usize, const C: usize>(data: &str) -> [[usize; C]; R] {
    let mut cost_map = [[0; C]; R];
    for (line, row) in data.lines().zip(cost_map.iter_mut()) {
        line.as_bytes()
            .iter()
            .zip(row.iter_mut())
            .for_each(|(x, m)| *m = (*x - b'0').into());
    }
    cost_map
}

#[derive(Debug, PartialEq, Eq)]
struct State<'a, const R: usize, const C: usize> {
    total_cost: usize,
    heuristic: usize,
    straight_line_count: usize,
    location: Point,
    facing: Direction,
    min_turn: usize,
    max_straight: usize,
    cost_map: &'a [[usize; C]; R],
}

impl<'a, const R: usize, const C: usize> State<'a, R, C> {
    const fn new(cost_map: &'a [[usize; C]; R], min_turn: usize, max_straight: usize) -> Self {
        Self {
            total_cost: 0,
            heuristic: R + C - 2,
            straight_line_count: 0,
            location: Point::new(0, 0),
            facing: Direction::East,
            min_turn,
            max_straight,
            cost_map,
        }
    }

    fn add_next_states(&self, heap: &mut BinaryHeap<Self>) {
        if self.straight_line_count < self.max_straight {
            self.add_next_state(heap, self.facing, self.straight_line_count);
        }

        self.add_next_state(heap, self.facing.turn(Side::Right), 0);
        self.add_next_state(heap, self.facing.turn(Side::Left), 0);
    }

    fn add_next_state(
        &self,
        heap: &mut BinaryHeap<Self>,
        dir: Direction,
        straight_line_count: usize,
    ) {
        let steps = if straight_line_count == 0 {
            self.min_turn
        } else {
            1
        };
        if let Some(next_point) = self.location.step(dir, R - 1, C - 1, steps) {
            let total_cost = self.total_cost
                + (0..steps)
                    .map(|i| match dir {
                        Direction::South => self.cost_map[next_point.row - i][next_point.col],
                        Direction::North => self.cost_map[next_point.row + i][next_point.col],
                        Direction::East => self.cost_map[next_point.row][next_point.col - i],
                        Direction::West => self.cost_map[next_point.row][next_point.col + i],
                    })
                    .sum::<usize>();
            heap.push(Self {
                total_cost,
                heuristic: total_cost + R + C - next_point.row - next_point.col - 2,
                straight_line_count: straight_line_count + steps,
                location: next_point,
                facing: dir,
                min_turn: self.min_turn,
                max_straight: self.max_straight,
                cost_map: self.cost_map,
            });
        }
    }
}

impl<'a, const R: usize, const C: usize> Ord for State<'a, R, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl<'a, const R: usize, const C: usize> PartialOrd for State<'a, R, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    const fn step(
        self,
        dir: Direction,
        row_lim: usize,
        col_lim: usize,
        steps: usize,
    ) -> Option<Self> {
        match (dir, self.row, self.col) {
            // Bounds checks
            (Direction::North, r, _) if r < steps => None,
            (Direction::West, _, c) if c < steps => None,
            (Direction::South, r, _) if r + steps > row_lim => None,
            (Direction::East, _, c) if c + steps > col_lim => None,
            // In bounds, normal movement
            (Direction::North, r, c) => Some(Self {
                row: r - steps,
                col: c,
            }),
            (Direction::West, r, c) => Some(Self {
                row: r,
                col: c - steps,
            }),
            (Direction::South, r, c) => Some(Self {
                row: r + steps,
                col: c,
            }),
            (Direction::East, r, c) => Some(Self {
                row: r,
                col: c + steps,
            }),
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    const fn turn(self, side: Side) -> Self {
        match (self, side) {
            (Self::North, Side::Left) | (Self::South, Side::Right) => Self::West,
            (Self::South, Side::Left) | (Self::North, Side::Right) => Self::East,
            (Self::East, Side::Left) | (Self::West, Side::Right) => Self::North,
            (Self::West, Side::Left) | (Self::East, Side::Right) => Self::South,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Right,
    Left,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(102, part_one::<13>(data));
    }

    #[test]
    fn two_a() {
        let data = include_str!("test.txt");
        assert_eq!(94, part_two::<13, 13>(data));
    }

    #[test]
    fn two_b() {
        let data = include_str!("test_b.txt");
        assert_eq!(71, part_two::<5, 12>(data));
    }

    #[test]
    fn two_c() {
        let data = include_str!("test_c.txt");
        assert_eq!(40, part_two::<7, 19>(data));
    }
}
