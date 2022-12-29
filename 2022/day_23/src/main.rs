use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let mut m = Map::new(data);
    for _ in 0..10 {
        m.run_round();
    }
    m.count_empty()
}

fn part_two(data: &str) -> u32 {
    let mut m = Map::new(data);
    let mut out = 0;
    for round in 1.. {
        let count = m.run_round();
        if count == 0 || round > 10_000 {
            out = round;
            break;
        }
    }
    out
}

#[derive(Debug)]
struct Map {
    data: [[bool; 200]; 200],
    proposed: [[Move; 200]; 200],
    next_direction: Move,
}

type Point = [usize; 2];

impl Map {
    fn new(input: &str) -> Self {
        let size = input.lines().count();
        let offset = 100 - size / 2;
        let mut data = [[false; 200]; 200];
        let proposed = [[Move::None; 200]; 200];
        for (i, row) in input.lines().enumerate() {
            for (j, elem) in row.char_indices() {
                if elem == '#' {
                    data[i + offset][j + offset] = true;
                }
            }
        }
        Self {
            data,
            proposed,
            next_direction: Move::North,
        }
    }

    // TODO: Maybe store this info?
    fn get_bounding_rect(&self) -> (Point, Point) {
        let mut y_min = 0_usize;
        let mut y_max = 199_usize;
        let mut x_min = 0_usize;
        let mut x_max = 199_usize;
        while !self.check_row(y_min, x_min, x_max) {
            y_min += 1;
        }
        while !self.check_row(y_max, x_min, x_max) {
            y_max -= 1;
        }
        while !self.check_col(x_min, y_min, y_max) {
            x_min += 1;
        }
        while !self.check_col(x_max, y_min, y_max) {
            x_max -= 1;
        }
        ([x_min, y_min], [x_max, y_max])
    }

    fn check_row(&self, row: usize, min: usize, max: usize) -> bool {
        for i in min..=max {
            if self.data[row][i] {
                return true;
            }
        }
        false
    }

    fn check_col(&self, col: usize, min: usize, max: usize) -> bool {
        for i in min..=max {
            if self.data[i][col] {
                return true;
            }
        }
        false
    }

    fn check(&self, location: Point) -> Move {
        let sur = [
            self.data[location[1] - 1][location[0] - 1],
            self.data[location[1] - 1][location[0]],
            self.data[location[1] - 1][location[0] + 1],
            self.data[location[1]][location[0] + 1],
            self.data[location[1] + 1][location[0] + 1],
            self.data[location[1] + 1][location[0]],
            self.data[location[1] + 1][location[0] - 1],
            self.data[location[1]][location[0] - 1],
        ];
        if !sur[0] && !sur[1] && !sur[2] && !sur[3] && !sur[4] && !sur[5] && !sur[6] && !sur[7] {
            return Move::None;
        }

        let zip = [Move::North, Move::South, Move::West, Move::East]
            .into_iter()
            .zip([[0, 1, 2], [4, 5, 6], [6, 7, 0], [2, 3, 4]].into_iter())
            .cycle();
        let zip = match self.next_direction {
            Move::North => zip.skip(0).take(4),
            Move::South => zip.skip(1).take(4),
            Move::West => zip.skip(2).take(4),
            Move::East => zip.skip(3).take(4),
            Move::None => unreachable!(),
        };
        for (m, e) in zip {
            if !sur[e[0]] && !sur[e[1]] && !sur[e[2]] {
                return m;
            }
        }
        Move::None
    }

    fn run_round(&mut self) -> u32 {
        let (min, max) = self.get_bounding_rect();
        for y in min[1]..=max[1] {
            for x in min[0]..=max[0] {
                if self.data[y][x] {
                    self.proposed[y][x] = self.check([x, y]);
                }
            }
        }

        let mut count = 0;
        for y in min[1]..=max[1] {
            for x in min[0]..=max[0] {
                match self.proposed[y][x] {
                    Move::North => {
                        self.data[y][x] = false;
                        self.data[y - 1][x] = true;
                        count += 1;
                    }
                    Move::South => {
                        if self.proposed[y + 2][x] == Move::North {
                            self.proposed[y + 2][x] = Move::None;
                        } else {
                            self.data[y][x] = false;
                            self.data[y + 1][x] = true;
                            count += 1;
                        }
                    }
                    Move::West => {
                        self.data[y][x] = false;
                        self.data[y][x - 1] = true;
                        count += 1;
                    }
                    Move::East => {
                        if self.proposed[y][x + 2] == Move::West {
                            self.proposed[y][x + 2] = Move::None;
                        } else {
                            self.data[y][x] = false;
                            self.data[y][x + 1] = true;
                            count += 1;
                        }
                    }
                    Move::None => (),
                }
            }
        }

        self.set_next_direction();
        self.proposed = [[Move::None; 200]; 200];
        count
    }

    fn set_next_direction(&mut self) {
        self.next_direction = [Move::North, Move::South, Move::West, Move::East]
            .into_iter()
            .cycle()
            .skip_while(|&m| m != self.next_direction)
            .nth(1)
            .unwrap();
    }

    fn count_empty(&self) -> u32 {
        let (min, max) = self.get_bounding_rect();
        let mut count = 0;
        for y in min[1]..=max[1] {
            for x in min[0]..=max[0] {
                if !self.data[y][x] {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.get_bounding_rect();
        for y in min[1]..=max[1] {
            for x in min[0]..=max[0] {
                if self.data[y][x] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    North,
    South,
    West,
    East,
    None,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_one() {
        let data = include_str!("test_simple.txt");
        assert_eq!(25, part_one(data));
    }

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(110, part_one(data));
    }

    #[test]
    fn simple_two() {
        let data = include_str!("test_simple.txt");
        assert_eq!(4, part_two(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(20, part_two(data));
    }
}
