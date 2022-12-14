use std::cmp::{max, min};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<200, 100>(data));
    println!("Part 2: {}", part_two::<200, 600>(data));
}

fn part_one<const R: usize, const C: usize>(data: &str) -> usize {
    let mut m = Map::<R, C>::new(data);
    let mut i = 0_usize;
    while m.add_sand().is_some() {
        i += 1;
    }
    i
}

fn part_two<const R: usize, const C: usize>(data: &str) -> usize {
    let mut m = Map::<R, C>::new(data);
    let mut i = 0_usize;
    while m.add_sand().unwrap_or(Point(0, 0)) != Point(INPUT, 0) {
        i += 1;
    }
    i + 1
}

// Going to span [450..550] & [..200]
#[derive(Debug)]
struct Map<const R: usize, const C: usize> {
    data: [[bool; C]; R],
    floor: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Point(usize, usize);

const INPUT: usize = 500;

impl<const R: usize, const C: usize> Map<R, C> {
    fn new(data: &str) -> Self {
        let mut map = [[false; C]; R];
        let mut last: Point;
        let mut max_depth = 0_usize;
        for line in data.lines() {
            last = Point(0, 0);
            let points = line.split(" -> ");
            for point in points {
                if let Some((x, y)) = point.split_once(',') {
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    if y > max_depth {
                        max_depth = y;
                    }
                    let end = Point(x, y);
                    if last != Point(0, 0) {
                        Self::add_line(&mut map, last, end);
                    }
                    last = end;
                }
            }
        }
        Self::add_line(
            &mut map,
            Point(INPUT - C / 2, max_depth + 2),
            Point(INPUT + C / 2, max_depth + 2),
        );
        Map {
            data: map,
            floor: max_depth + 2,
        }
    }

    fn add_line(map: &mut [[bool; C]; R], start: Point, end: Point) {
        let x_offset = INPUT - C / 2;
        if start.0 == end.0 {
            for row in map
                .iter_mut()
                .take(max(start.1, end.1) + 1)
                .skip(min(start.1, end.1))
            {
                row[start.0 - x_offset] = true;
            }
        }
        if start.1 == end.1 {
            for elem in map[start.1]
                .iter_mut()
                .take(max(start.0, end.0) - x_offset + 1)
                .skip(min(start.0, end.0) - x_offset)
            {
                *elem = true;
            }
        }
    }

    fn get(&self, loc: Point) -> bool {
        self.data[loc.1][loc.0 - (INPUT - C / 2)]
    }

    fn set(&mut self, loc: Point) {
        self.data[loc.1][loc.0 - (INPUT - C / 2)] = true
    }

    fn add_sand(&mut self) -> Option<Point> {
        let mut loc = Point(INPUT, 0);
        while let Some(p) = self.drop_one(loc) {
            loc = p;
            if loc.1 == self.floor - 1 {
                self.set(loc);
                return None;
            }
        }
        self.set(loc);
        Some(loc)
    }

    fn drop_one(&self, loc: Point) -> Option<Point> {
        let points = [
            Point(loc.0, loc.1 + 1),
            Point(loc.0 - 1, loc.1 + 1),
            Point(loc.0 + 1, loc.1 + 1),
        ];
        points.into_iter().find(|&p| !self.get(p))
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in self.data {
            for elem in row {
                if elem {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(24, part_one::<12, 20>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(93, part_two::<12, 30>(data));
    }
}
