use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<200, 150>(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one<const R: usize, const C: usize>(data: &'static str) -> usize {
    let mut gps = Gps::<R, C>::new(data);
    gps.step_all();
    println!("{gps}");
    gps.password()
}

fn part_two(_data: &str) -> usize {
    0
}

#[derive(Debug)]
struct Gps<const R: usize, const C: usize> {
    map: [[MapSquare; C]; R],
    location: (Point, Direction),
    directions: &'static [u8],
    direction_index: usize,
}

impl<const R: usize, const C: usize> Display for Gps<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.map.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if [j, i] == self.location.0 {
                    match self.location.1 {
                        Direction::Up => write!(f, "^")?,
                        Direction::Down => write!(f, "v")?,
                        Direction::Right => write!(f, ">")?,
                        Direction::Left => write!(f, "<")?,
                    }
                } else {
                    match elem {
                        MapSquare::Blank => write!(f, " ")?,
                        MapSquare::Open => write!(f, ".")?,
                        MapSquare::Wall => write!(f, "#")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const R: usize, const C: usize> Gps<R, C> {
    fn new(data: &'static str) -> Self {
        let mut map = [[MapSquare::Blank; C]; R];
        let mut found = false;
        let mut loc = 0;

        let (map_data, directions) = data.split_once("\n\n").unwrap();
        for (row, line) in map_data.lines().enumerate() {
            for (i, c) in line.char_indices() {
                match c {
                    ' ' => (),
                    '.' => {
                        if row == 0 && !found {
                            loc = i;
                            found = true;
                        }
                        map[row][i] = MapSquare::Open;
                    }
                    '#' => map[row][i] = MapSquare::Wall,
                    _ => unreachable!(),
                }
            }
        }

        Self {
            map,
            location: ([loc, 0], Direction::Right),
            directions: directions.trim().as_bytes(),
            direction_index: 0,
        }
    }

    fn password(&self) -> usize {
        let x = self.location.0[0] + 1;
        let y = self.location.0[1] + 1;
        match self.location.1 {
            Direction::Right => 1_000 * y + 4 * x,
            Direction::Down => 1_000 * y + 4 * x + 1,
            Direction::Left => 1_000 * y + 4 * x + 2,
            Direction::Up => 1_000 * y + 4 * x + 3,
        }
    }

    fn step_all(&mut self) {
        while self.direction_index < self.directions.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.get_next_direction() {
            DirectionNum::Dir(d) => self.location.1 = d,
            DirectionNum::Num(v) => {
                for _ in 0..v {
                    let next = self.get_next_cell();
                    if self.map[next[1]][next[0]] == MapSquare::Wall {
                        return;
                    }
                    self.location.0 = next;
                }
            }
        }
    }

    fn get_next_cell(&self) -> Point {
        let x = self.location.0[0];
        let y = self.location.0[1];
        match self.location.1 {
            Direction::Up => {
                if y > 0 && self.map[y - 1][x] != MapSquare::Blank {
                    return [x, y - 1];
                }
                for new in (0..R).rev() {
                    if self.map[new][x] != MapSquare::Blank {
                        return [x, new];
                    }
                }
                unreachable!();
            }
            Direction::Right => {
                if x + 1 < C && self.map[y][x + 1] != MapSquare::Blank {
                    return [x + 1, y];
                }
                for new in 0..C {
                    if self.map[y][new] != MapSquare::Blank {
                        return [new, y];
                    }
                }
                unreachable!();
            }
            Direction::Down => {
                if y + 1 < R && self.map[y + 1][x] != MapSquare::Blank {
                    return [x, y + 1];
                }
                for new in 0..R {
                    if self.map[new][x] != MapSquare::Blank {
                        return [x, new];
                    }
                }
                unreachable!();
            }
            Direction::Left => {
                if x > 0 && self.map[y][x - 1] != MapSquare::Blank {
                    return [x - 1, y];
                }
                for new in (0..C).rev() {
                    if self.map[y][new] != MapSquare::Blank {
                        return [new, y];
                    }
                }
                unreachable!();
            }
        }
    }

    fn get_next_direction(&mut self) -> DirectionNum {
        match self.directions[self.direction_index] {
            b'R' => {
                self.direction_index += 1;
                match self.location.1 {
                    Direction::Up => DirectionNum::Dir(Direction::Right),
                    Direction::Right => DirectionNum::Dir(Direction::Down),
                    Direction::Down => DirectionNum::Dir(Direction::Left),
                    Direction::Left => DirectionNum::Dir(Direction::Up),
                }
            }
            b'L' => {
                self.direction_index += 1;
                match self.location.1 {
                    Direction::Up => DirectionNum::Dir(Direction::Left),
                    Direction::Right => DirectionNum::Dir(Direction::Up),
                    Direction::Down => DirectionNum::Dir(Direction::Right),
                    Direction::Left => DirectionNum::Dir(Direction::Down),
                }
            }
            _ => {
                let mut i = self.direction_index + 1;
                while i < self.directions.len()
                    && self.directions[i] != b'R'
                    && self.directions[i] != b'L'
                {
                    i += 1;
                }
                let val = self.directions[self.direction_index..i]
                    .iter()
                    .fold(0, |acc, v| acc * 10 + usize::try_from(*v - b'0').unwrap());
                self.direction_index = i;
                DirectionNum::Num(val)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapSquare {
    Blank,
    Open,
    Wall,
}

type Point = [usize; 2];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

enum DirectionNum {
    Dir(Direction),
    Num(usize),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(6032, part_one::<12, 16>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
