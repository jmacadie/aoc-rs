use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

pub fn main() {
    const N: usize = 2_963;
    const R: usize = 26;
    const C: usize = 121;
    const T: usize = 300;

    let data = include_str!("input.txt");
    let blizzards = Blizzards::<N, R, C, T>::new(data);
    let (steps, state) = part_one::<N, R, C, T>(blizzards);
    println!("Part 1: {}", steps);

    let blizzards = Blizzards::<N, R, C, T>::from_state(state);
    let steps_2 = part_two::<N, R, C, T>(blizzards);
    println!("Part 2: {}", steps + steps_2);
}

fn part_one<const N: usize, const R: usize, const C: usize, const T: usize>(
    blizzards: Blizzards<N, R, C, T>,
) -> (usize, [Blizzard; N]) {
    find_path([1, 0], [C - 1, R], blizzards)
}

fn part_two<const N: usize, const R: usize, const C: usize, const T: usize>(
    blizzards: Blizzards<N, R, C, T>,
) -> usize {
    let (steps_1, state) = find_path([C - 1, R], [1, 0], blizzards);

    let blizzards = Blizzards::<N, R, C, T>::from_state(state);
    let (steps_2, _) = find_path([1, 0], [C - 1, R], blizzards);
    steps_1 + steps_2
}

fn find_path<const N: usize, const R: usize, const C: usize, const T: usize>(
    start: Point,
    end: Point,
    mut blizzards: Blizzards<N, R, C, T>,
) -> (usize, [Blizzard; N]) {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut path = [Point::default(); T];
    path[0] = start;
    heap.push(State {
        cost: manhattan_distance(start, end),
        position: start,
        time: 0,
        path,
    });

    while let Some(s) = heap.pop() {
        if s.position == end {
            //println!("{:?}", s.path);
            return (s.time, blizzards.current);
        }
        if visited.contains(&(s.position, s.time)) {
            continue;
        }
        visited.insert((s.position, s.time));
        let time = s.time + 1;
        let mut path = s.path;
        for next in blizzards.get_neighbours(s.position, time) {
            if next == [0, 0] {
                break;
            }
            path[time] = next;
            let next_state = State {
                cost: time + manhattan_distance(next, end),
                position: next,
                time,
                path,
            };
            heap.push(next_state);
        }
    }
    (0, blizzards.current)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State<const T: usize> {
    cost: usize,
    position: Point,
    time: usize,
    path: [Point; T],
}

impl<const T: usize> Ord for State<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.time.cmp(&other.time))
    }
}

impl<const T: usize> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(location: Point, destination: Point) -> usize {
    let x = if location[0] > destination[0] {
        location[0] - destination[0]
    } else {
        destination[0] - location[0]
    };
    let y = if location[1] > destination[1] {
        location[1] - destination[1]
    } else {
        destination[1] - location[1]
    };
    x + y
}

struct Blizzards<const N: usize, const R: usize, const C: usize, const T: usize> {
    time: usize,
    current: [Blizzard; N],
    states: [[[bool; C]; R]; T],
}

impl<const N: usize, const R: usize, const C: usize, const T: usize> Display
    for Blizzards<N, R, C, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.states[self.time].iter().enumerate() {
            for (x, elem) in row.iter().enumerate() {
                if x == 1 && y == 0 {
                    write!(f, "_")?;
                } else if x == 0 || y == 0 {
                    write!(f, "#")?;
                } else if !elem {
                    write!(f, "+")?;
                } else {
                    write!(f, "_")?;
                }
            }
            writeln!(f, "#")?;
        }
        for x in 0..C {
            if x == C - 1 {
                write!(f, "_")?;
            } else {
                write!(f, "#")?;
            }
        }
        writeln!(f, "#")?;
        Ok(())
    }
}

impl<const N: usize, const R: usize, const C: usize, const T: usize> Blizzards<N, R, C, T> {
    fn new(data: &str) -> Self {
        let mut current = [Blizzard::default(); N];
        let mut counter = 0;
        for (i, row) in data.lines().enumerate().skip(1) {
            for (j, elem) in row.char_indices() {
                match elem {
                    '^' => {
                        current[counter] = Blizzard {
                            location: [j, i],
                            direction: Direction::Up,
                        };
                        counter += 1;
                    }
                    '>' => {
                        current[counter] = Blizzard {
                            location: [j, i],
                            direction: Direction::Right,
                        };
                        counter += 1;
                    }
                    'v' => {
                        current[counter] = Blizzard {
                            location: [j, i],
                            direction: Direction::Down,
                        };
                        counter += 1;
                    }
                    '<' => {
                        current[counter] = Blizzard {
                            location: [j, i],
                            direction: Direction::Left,
                        };
                        counter += 1;
                    }
                    '.' | '#' => (),
                    _ => unreachable!(),
                }
            }
        }
        Self {
            time: 0,
            current,
            states: [[[false; C]; R]; T],
        }
    }

    fn from_state(current: [Blizzard; N]) -> Self {
        Self {
            time: 0,
            current,
            states: [[[false; C]; R]; T],
        }
    }

    fn get_neighbours(&mut self, location: Point, time: usize) -> [Point; 5] {
        let mut out = [Point::default(); 5];
        if time > self.time {
            self.step();
        }
        let paths = self.states[time];
        let mut count = 0;
        for step in [[0, 0], [1, 0], [0, 1], [-1, 0], [0, -1]] {
            let x = location[0] as i32 + step[0];
            let y = location[1] as i32 + step[1];
            if Self::in_bounds(x, y) {
                let x = x as usize;
                let y = y as usize;
                if (x == C - 1 && y == R) || paths[y][x] {
                    out[count] = [x, y];
                    count += 1;
                }
            }
        }
        out
    }

    fn in_bounds(x: i32, y: i32) -> bool {
        if (x == 1 && y == 0) || (x + 1 == C as i32 && y == R as i32) {
            return true;
        }
        if x < 1 || x >= C as i32 || y < 1 || y >= R as i32 {
            return false;
        }
        true
    }

    fn step(&mut self) {
        let mut paths = [[true; C]; R];
        for blizzard in self.current.iter_mut() {
            if blizzard.location == Point::default() {
                break;
            }
            *blizzard = blizzard.step::<R, C>();
            paths[blizzard.location[1]][blizzard.location[0]] = false;
        }
        self.time += 1;
        self.states[self.time] = paths;
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    location: Point,
    direction: Direction,
}

impl Default for Blizzard {
    fn default() -> Self {
        Blizzard {
            location: Point::default(),
            direction: Direction::Up,
        }
    }
}

impl Blizzard {
    fn step<const R: usize, const C: usize>(&self) -> Self {
        let new = match self.direction {
            Direction::Up => {
                if self.location[1] == 1 {
                    [self.location[0], R - 1]
                } else {
                    [self.location[0], self.location[1] - 1]
                }
            }
            Direction::Right => {
                if self.location[0] == C - 1 {
                    [1, self.location[1]]
                } else {
                    [self.location[0] + 1, self.location[1]]
                }
            }
            Direction::Down => {
                if self.location[1] == R - 1 {
                    [self.location[0], 1]
                } else {
                    [self.location[0], self.location[1] + 1]
                }
            }
            Direction::Left => {
                if self.location[0] == 1 {
                    [C - 1, self.location[1]]
                } else {
                    [self.location[0] - 1, self.location[1]]
                }
            }
        };
        Self {
            location: new,
            direction: self.direction,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Point = [usize; 2];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        const N: usize = 19;
        const R: usize = 5;
        const C: usize = 7;
        const T: usize = 19;

        let data = include_str!("test.txt");
        let blizzards = Blizzards::<N, R, C, T>::new(data);
        let (steps, _) = part_one::<N, R, C, T>(blizzards);
        assert_eq!(18, steps);
    }

    #[test]
    fn two() {
        const N: usize = 19;
        const R: usize = 5;
        const C: usize = 7;
        const T: usize = 24;

        let data = include_str!("test.txt");
        let blizzards = Blizzards::<N, R, C, T>::new(data);
        let (steps, state) = part_one::<N, R, C, T>(blizzards);

        let blizzards = Blizzards::<N, R, C, T>::from_state(state);
        let steps_2 = part_two::<N, R, C, T>(blizzards);
        assert_eq!(54, steps + steps_2);
    }
}
