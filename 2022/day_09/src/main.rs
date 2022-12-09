use std::cmp::Ordering;

pub fn main() {
    let data = include_str!("input.txt");
    let (child, tail) = compute_visited(data);
    println!("Part 1: {}", part_one(&child));
    println!("Part 2: {}", part_two(&tail));
}

fn part_one(visited: &Map) -> usize {
    visited.iter().flatten().filter(|&b| *b).count()
}

fn part_two(visited: &Map) -> usize {
    visited.iter().flatten().filter(|&b| *b).count()
}

fn compute_visited(data: &str) -> (Map, Map) {
    let mut rope = [Point(0, 0); 10];
    let len = rope.len();
    let mut child_visited: Map = [[false; 400]; 400];
    let mut tail_visited: Map = [[false; 400]; 400];
    child_visited.visit(rope[1]);
    tail_visited.visit(rope[9]);
    for line in data.lines() {
        let (dir, num) = parse_line(line);
        for _ in 0..num {
            rope[0].move_head(dir);
            for i in 1..len {
                if rope[i].is_touching(rope[i - 1]) {
                    break;
                }
                rope[i].child_move(rope[i - 1]);
                if i == 1 {
                    child_visited.visit(rope[i]);
                }
                if i == (len - 1) {
                    tail_visited.visit(rope[i]);
                }
            }
        }
    }
    (child_visited, tail_visited)
}

fn parse_line(line: &str) -> (char, u8) {
    let mut parts = line.split(' ');
    let (Some(dir), Some(val), None) = (parts.next(), parts.next(), parts.next()) else {
        unreachable!();
    };
    let dir = dir.chars().next().unwrap();
    let val = val.parse().unwrap();
    (dir, val)
}

trait Visitable {
    fn visit(&mut self, loc: Point);
}

type Map = [[bool; 400]; 400];
impl Visitable for Map {
    fn visit(&mut self, loc: Point) {
        let x: usize = (loc.0 + 200).try_into().unwrap();
        let y: usize = (loc.1 + 200).try_into().unwrap();
        self[x][y] = true;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point(i16, i16);

impl Point {
    fn move_head(&mut self, dir: char) {
        match dir {
            'U' => self.0 += 1,
            'D' => self.0 -= 1,
            'R' => self.1 += 1,
            'L' => self.1 -= 1,
            _ => unreachable!(),
        }
    }

    fn child_move(&mut self, parent: Self) {
        match parent.0.cmp(&self.0) {
            Ordering::Greater => self.0 += 1,
            Ordering::Less => self.0 -= 1,
            Ordering::Equal => (),
        }
        match parent.1.cmp(&self.1) {
            Ordering::Greater => self.1 += 1,
            Ordering::Less => self.1 -= 1,
            Ordering::Equal => (),
        }
    }

    fn is_touching(&self, other: Self) -> bool {
        (-1..=1).contains(&(self.0 - other.0)) && (-1..=1).contains(&(self.1 - other.1))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (child, _) = compute_visited(data);
        assert_eq!(13, part_one(&child));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (_, tail) = compute_visited(data);
        assert_eq!(1, part_two(&tail));
    }

    #[test]
    fn two_v2() {
        let data = include_str!("test2.txt");
        let (_, tail) = compute_visited(data);
        assert_eq!(36, part_two(&tail));
    }
}
