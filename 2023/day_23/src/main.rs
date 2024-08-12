#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::VecDeque, thread};

pub fn main() {
    let data = include_str!("input.txt");
    let graph = Graph::new::<141>(data);
    println!("Part 1: {}", part_one(&graph));
    println!("Part 2: {}", part_two(&graph));
}

fn part_one(graph: &Graph) -> usize {
    graph.max_path(true)
}

fn part_two(graph: &Graph) -> usize {
    graph.max_path_threaded(false)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PathType {
    Open,
    Forest,
    Downslope,
    Upslope,
}

type PointIdx = usize;
type Distance = usize;

#[derive(PartialEq, Eq, Debug)]
enum StepResult {
    None,
    Single(Position),
    Junction(Position),
    End,
}

#[derive(Debug)]
struct Destinations {
    data: [Option<(PointIdx, Distance, Direction)>; 4],
    edge: bool,
    south: bool,
    east: bool,
}

impl Destinations {
    const fn new() -> Self {
        Self {
            data: [None; 4],
            edge: true,
            south: false,
            east: false,
        }
    }

    fn has_direction(&self, dir: Direction) -> bool {
        self.data
            .iter()
            .filter_map(|&x| x)
            .any(|(_, _, d)| d == dir)
    }

    fn get_direction(&self, dir: Direction) -> Option<(PointIdx, Distance)> {
        self.data
            .iter()
            .filter_map(|&x| x)
            .find(|(_, _, d)| *d == dir)
            .map(|(p, d, _)| (p, d))
    }

    fn add(&mut self, p: PointIdx, dist: Distance, dir: Direction) {
        *self
            .data
            .iter_mut()
            .find(|x| x.is_none())
            .expect("not all directions are already taken") = Some((p, dist, dir));
        match dir {
            Direction::South => self.south = true,
            Direction::East => self.east = true,
            _ => (),
        }
        if !self.data.iter().any(Option::is_none) {
            self.edge = false;
        }
    }

    fn iter(&self, simple: bool) -> impl Iterator<Item = (PointIdx, Distance)> + '_ {
        let (edge, south, east) = if simple {
            (true, true, true)
        } else {
            (self.edge, self.south, self.east)
        };
        self.data
            .iter()
            .filter_map(move |&x| match (edge, south, east, x) {
                (true, true, _, Some((_, _, Direction::North)))
                | (true, _, true, Some((_, _, Direction::West))) => None,
                _ => x.map(|(p, d, _)| (p, d)),
            })
    }
}

#[derive(Debug)]
struct Graph {
    data: Vec<(Point, Destinations)>,
}

impl Graph {
    const END: usize = 100;

    fn new<const N: usize>(s: &str) -> Self {
        let map = s.as_bytes();
        let mut data: Vec<(Point, Destinations)> = Vec::with_capacity(Self::END);
        let mut queue = Queue::new();
        let start = Position {
            loc: Point { x: 1, y: 0 },
            dir: Direction::South,
        };

        queue.push(start);
        data.push((start.loc, Destinations::new()));

        while let Some(from_pos) = queue.pop() {
            if !data
                .iter()
                .filter(|(p, _)| *p == from_pos.loc)
                .any(|(_, arr)| arr.has_direction(from_pos.dir))
            {
                let (res, dist) = Self::walk_segment::<N>(map, from_pos, &mut queue);
                match res {
                    StepResult::None => (),                  // Dead end
                    StepResult::Single(_) => unreachable!(), // Should have walked on
                    StepResult::Junction(to_pos) => {
                        if let Some(from_idx) = data.iter().position(|&(p, _)| p == from_pos.loc) {
                            let to_idx = data
                                .iter()
                                .position(|&(p, _)| p == to_pos.loc)
                                .unwrap_or_else(|| {
                                    data.push((to_pos.loc, Destinations::new()));
                                    data.len() - 1
                                });
                            data[from_idx].1.add(to_idx, dist, from_pos.dir);
                            data[to_idx].1.add(from_idx, dist, to_pos.dir);
                        }
                    }
                    StepResult::End => {
                        if let Some(from_idx) = data.iter().position(|&(p, _)| p == from_pos.loc) {
                            data[from_idx].1.add(Self::END, dist, from_pos.dir);
                        }
                    }
                }
            }
        }

        Self { data }
    }

    fn walk_segment<const N: usize>(
        map: &[u8],
        from: Position,
        queue: &mut Queue,
    ) -> (StepResult, Distance) {
        let mut res = Self::step::<N>(map, from, queue);
        let mut dist = 1;
        while let StepResult::Single(next) = res {
            res = Self::step::<N>(map, next, queue);
            dist += 1;
        }
        (res, dist)
    }

    fn step<const N: usize>(map: &[u8], from: Position, queue: &mut Queue) -> StepResult {
        let next = from.step(from.dir).invert();
        if next.loc.y == N - 2 && next.loc.x == N - 2 {
            return StepResult::End;
        }
        let north = next.step(Direction::North);
        let south = next.step(Direction::South);
        let east = next.step(Direction::East);
        let west = next.step(Direction::West);
        let north = (next.turn(Direction::North), Self::get::<N>(map, north));
        let south = (next.turn(Direction::South), Self::get::<N>(map, south));
        let east = (next.turn(Direction::East), Self::get::<N>(map, east));
        let west = (next.turn(Direction::West), Self::get::<N>(map, west));
        let routes_out = match next.dir {
            Direction::North => [south, east, west],
            Direction::South => [north, east, west],
            Direction::East => [north, south, west],
            Direction::West => [north, south, east],
        };
        let mut ways_out = routes_out
            .iter()
            .filter(|(_, pt)| pt != &PathType::Forest)
            .map(|(p, _)| p);
        let Some(&first) = ways_out.next() else {
            return StepResult::None;
        };
        let Some(&second) = ways_out.next() else {
            return StepResult::Single(first);
        };
        queue.push(first);
        queue.push(second);
        if let Some(&third) = ways_out.next() {
            queue.push(third);
        };
        StepResult::Junction(next)
    }

    fn get<const N: usize>(map: &[u8], p: Position) -> PathType {
        let idx = p.loc.x + (N + 1) * p.loc.y;
        match (map[idx], p.dir) {
            (b'#', _) => PathType::Forest,
            (b'.', _) => PathType::Open,
            (b'^', Direction::North)
            | (b'v', Direction::South)
            | (b'>', Direction::East)
            | (b'<', Direction::West) => PathType::Downslope,
            (b'^', Direction::South)
            | (b'v', Direction::North)
            | (b'>', Direction::West)
            | (b'<', Direction::East) => PathType::Upslope,
            _ => unreachable!(),
        }
    }

    fn max_path(&self, simple: bool) -> Distance {
        // Add one as we're starting one step in, to avoid dealing with the edge of the board
        self.max_path_inner_imperative(0, 1, simple)
            .expect("at least one path exists")
            + 1
    }

    fn max_path_threaded(&self, simple: bool) -> Distance {
        let s = Direction::South;
        let e = Direction::East;
        let _n = Direction::North;
        let _w = Direction::West;
        // let starts = [
        //     [s, e, e, e], // 1, 3, 6, 10
        //     [s, e, e, s], // 1, 3, 6, 9
        //     [s, e, s, e], // 1, 3, 5, 9
        //     [s, e, s, s], // 1, 3, 5, 8
        //     [s, e, s, w], // 1, 3, 5, 2
        //     [s, s, e, e], // 1, 2, 5, 9
        //     [s, s, e, s], // 1, 2, 5, 8
        //     [s, s, e, n], // 1, 2, 5, 3
        //     [s, s, s, e], // 1, 2, 4, 8
        //     [s, s, s, s], // 1, 2, 4, 7
        // ];
        let starts = [
            [s, e, e], // 1, 3, 6
            [s, e, s], // 1, 3, 5
            [s, s, e], // 1, 2, 5
            [s, s, s], // 1, 2, 4
        ];

        thread::scope(|s| {
            let mut handles = Vec::with_capacity(4);
            for dirs in starts {
                let handle = s.spawn(move || {
                    dirs.into_iter()
                        .try_fold((0, 1, 1), |(p, d, v), dir| {
                            let (p_next, d_next) = self.data[p].1.get_direction(dir)?;
                            Some((p_next, d + d_next, v | (1 << p_next)))
                        })
                        .and_then(|(p, d, v)| {
                            self.max_path_inner_imperative(p, v, simple)
                                .map(|inner| inner + d)
                        })
                });

                handles.push(handle);
            }

            handles
                .into_iter()
                .map(|h| h.join().unwrap().unwrap())
                .max()
                .unwrap()
        })
    }

    #[allow(dead_code)]
    fn max_path_inner_functional(
        &self,
        from: PointIdx,
        visited: u64,
        simple: bool,
    ) -> Option<Distance> {
        self.data[from]
            .1
            .iter(simple)
            .map(|(p, d)| {
                if p == Self::END {
                    return Some(d);
                }
                let mask = 1 << p;
                if mask & visited != 0 {
                    return None;
                }
                self.max_path_inner_functional(p, visited | mask, simple)
                    .map(|inner| inner + d)
            })
            .max()?
    }

    fn max_path_inner_imperative(
        &self,
        from: PointIdx,
        visited: u64,
        simple: bool,
    ) -> Option<Distance> {
        let mut max_dist = None;
        for (p, d) in self.data[from].1.iter(simple) {
            let new = if p == Self::END {
                Some(d)
            } else {
                let mask = 1 << p;
                if (visited & mask) != 0 {
                    None
                } else {
                    self.max_path_inner_imperative(p, visited | mask, simple)
                        .map(|inner| inner + d)
                }
            };
            max_dist = match (max_dist, new) {
                (a, None) => a,
                (None, b) => b,
                (Some(a), Some(b)) => Some(std::cmp::max(a, b)),
            };
        }
        max_dist
    }
}

struct Queue {
    data: VecDeque<Position>,
}

impl Queue {
    fn new() -> Self {
        Self {
            data: VecDeque::with_capacity(20),
        }
    }

    fn push(&mut self, p: Position) {
        self.data.push_back(p);
    }

    fn pop(&mut self) -> Option<Position> {
        self.data.pop_front()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Position {
    loc: Point,
    dir: Direction,
}

impl Position {
    const fn step(self, dir: Direction) -> Self {
        Self {
            loc: self.loc.step(dir),
            dir,
        }
    }

    const fn turn(self, dir: Direction) -> Self {
        Self { loc: self.loc, dir }
    }

    const fn invert(self) -> Self {
        Self {
            loc: self.loc,
            dir: self.dir.invert(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn step(self, dir: Direction) -> Self {
        match dir {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn invert(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let graph = Graph::new::<23>(data);
        assert_eq!(94, part_one(&graph));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let graph = Graph::new::<23>(data);
        assert_eq!(154, part_two(&graph));
    }
}
