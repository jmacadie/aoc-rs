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
        let mut parts = line.split(' ');
        let (Some(dir), Some(val), None) = (parts.next(), parts.next(), parts.next()) else {
            unreachable!();
        };
        let val = val.parse().unwrap();
        for _ in 0..val {
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
        //println!("*******************");
        //println!("Move: {line}");
        let mut parts = line.split(' ');
        let (Some(dir), Some(val), None) = (parts.next(), parts.next(), parts.next()) else {
            unreachable!();
        };
        let val = val.parse().unwrap();
        for _ in 0..val {
            rope[0].move_dir(dir);
            for i in 0..9 {
                if rope[i].is_touching(rope[i + 1]) {
                    break;
                }
                rope[i + 1].move_child(rope[i]);
                //std::mem::swap(&mut rope[i + 1], &mut last);
                if i == 8 {
                    //println!("Updating Rope: {rope:?}");
                    visited.insert(rope[9]);
                }
            }
        }
        //println!("New rope: {rope:?}");
        //println!("*******************");
        //println!();
    }
    visited.len()
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point(i16, i16);

type Rope = [Point; 10];

impl Point {
    fn move_dir(&mut self, dir: &str) {
        let dir = dir.chars().next().unwrap();
        match dir {
            'U' => self.0 += 1,
            'D' => self.0 -= 1,
            'R' => self.1 += 1,
            'L' => self.1 -= 1,
            _ => unreachable!(),
        }
    }

    fn move_child(&mut self, parent: Self) {
        if parent.0 > self.0 {
            self.0 += 1
        } else if parent.0 < self.0 {
            self.0 -= 1
        }
        if parent.1 > self.1 {
            self.1 += 1
        } else if parent.1 < self.1 {
            self.1 -= 1
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
