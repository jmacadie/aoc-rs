use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn main() {
    let data = include_str!("input.txt");
    let (map, start, end) = parse_input::<41, 83>(data);
    println!("Part 1: {}", part_one::<41, 83>(&map, start, end));
    println!("Part 2: {}", part_two::<41, 83>(&map, end));
}

fn part_one<const R: usize, const C: usize>(map: &Map<u8, R, C>, start: Point, end: Point) -> u32 {
    solve::<R, C>(map, start, end, 0, Direction::Up).unwrap()
}

fn part_two<const R: usize, const C: usize>(map: &Map<u8, R, C>, end: Point) -> u32 {
    solve::<R, C>(map, end, (0, 0), b'a', Direction::Down).unwrap()
}

fn solve<const R: usize, const C: usize>(
    map: &Map<u8, R, C>,
    start: Point,
    end: Point,
    target_val: u8,
    dir: Direction,
) -> Option<u32> {
    let mut dist: Map<u32, R, C> = [[u32::MAX; C]; R];
    let mut heap = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end || map[position.0][position.1] == target_val {
            return Some(cost);
        }
        if cost > dist[position.0][position.1] {
            continue;
        }
        for n in get_neighbours::<R, C>(position, map, dir) {
            if cost + 1 < dist[n.0][n.1] {
                heap.push(State {
                    cost: cost + 1,
                    position: n,
                });
                dist[n.0][n.1] = cost + 1;
            }
        }
    }
    None
}

fn get_neighbours<const R: usize, const C: usize>(
    loc: Point,
    map: &Map<u8, R, C>,
    dir: Direction,
) -> Vec<Point> {
    let mut out = Vec::with_capacity(4);
    let steps = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let height = map[loc.0][loc.1];
    for step in steps {
        if let Some(new) = valid_step::<R, C>(loc, height, step, map, dir) {
            out.push(new);
        }
    }
    out
}

fn valid_step<const R: usize, const C: usize>(
    loc: Point,
    height: u8,
    step: (i8, i8),
    map: &Map<u8, R, C>,
    dir: Direction,
) -> Option<Point> {
    if in_bounds::<R, C>(loc, step) {
        let new = (
            (loc.0 as i8 + step.0) as usize,
            (loc.1 as i8 + step.1) as usize,
        );
        if (dir == Direction::Up && height + 1 >= map[new.0][new.1])
            || (dir == Direction::Down && height - 1 <= map[new.0][new.1])
        {
            return Some(new);
        }
    }
    None
}

fn in_bounds<const R: usize, const C: usize>(loc: Point, step: (i8, i8)) -> bool {
    if loc.0 == 0 && step.0 == -1 {
        return false;
    }
    if loc.0 == R - 1 && step.0 == 1 {
        return false;
    }
    if loc.1 == 0 && step.1 == -1 {
        return false;
    }
    if loc.1 == C - 1 && step.1 == 1 {
        return false;
    }
    true
}

fn parse_input<const R: usize, const C: usize>(data: &str) -> (Map<u8, R, C>, Point, Point) {
    let mut map: Map<u8, R, C> = [[0; C]; R];
    let mut start: Point = (0, 0);
    let mut end: Point = (0, 0);
    for (row, line) in data.lines().enumerate() {
        for (col, elem) in line.bytes().enumerate() {
            match elem {
                b'S' => {
                    start = (row, col);
                    map[row][col] = b'a';
                }
                b'E' => {
                    end = (row, col);
                    map[row][col] = b'z';
                }
                _ => map[row][col] = elem,
            };
        }
    }
    (map, start, end)
}

type Map<T, const R: usize, const C: usize> = [[T; C]; R];

trait Printable {
    fn print(&self);
}

impl<const R: usize, const C: usize> Printable for Map<u8, R, C> {
    fn print(&self) {
        for row in self {
            for elem in row {
                if elem > &0 {
                    print!("{}", *elem as char);
                }
            }
            println!();
        }
    }
}

type Point = (usize, usize);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point,
}

impl Ord for State {
    // Reverse direction so we have a min heap
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (map, start, end) = parse_input::<5, 8>(data);
        assert_eq!(31, part_one::<5, 8>(&map, start, end));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (map, _, end) = parse_input::<5, 8>(data);
        assert_eq!(29, part_two::<5, 8>(&map, end));
    }
}
