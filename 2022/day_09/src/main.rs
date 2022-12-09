use std::cmp::Ordering;
use std::collections::HashSet;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let mut head = Point(0, 0);
    let mut tail = head;
    let mut visited = HashSet::with_capacity(10_000);
    visited.insert(tail);
    for line in data.lines() {
        let (dir, num) = parse_line(line);
        for _ in 0..num {
            let last = head;
            head.move_dir(dir);
            if !head.is_touching(tail) {
                tail = last;
                visited.insert(tail);
            }
        }
    }
    visited.len()
}

fn part_two(data: &str) -> usize {
    let mut rope: Rope = [Point(0, 0); 10];
    let mut visited = HashSet::with_capacity(10_000);
    visited.insert(rope[9]);
    for line in data.lines() {
        let (dir, num) = parse_line(line);
        for _ in 0..num {
            rope[0].move_dir(dir);
            for i in 0..9 {
                if rope[i].is_touching(rope[i + 1]) {
                    break;
                }
                rope[i + 1].move_child(rope[i]);
                if i == 8 {
                    visited.insert(rope[9]);
                }
            }
        }
    }
    visited.len()
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point(i16, i16);

type Rope = [Point; 10];

impl Point {
    fn move_dir(&mut self, dir: char) {
        match dir {
            'U' => self.0 += 1,
            'D' => self.0 -= 1,
            'R' => self.1 += 1,
            'L' => self.1 -= 1,
            _ => unreachable!(),
        }
    }

    fn move_child(&mut self, parent: Self) {
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
        assert_eq!(13, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(1, part_two(data));
    }

    #[test]
    fn two_v2() {
        let data = include_str!("test2.txt");
        assert_eq!(36, part_two(data));
    }
}
