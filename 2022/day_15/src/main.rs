use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<2_000_000>(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one<const ROW: i32>(data: &str) -> i32 {
    data.lines()
        .map(read_line)
        .map(|(s, b)| (s, manhattan_distance(s, b)))
        .filter_map(points_on_line::<ROW>)
        .sorted_unstable_by_key(|(a, _)| a.0)
        .fold(Default::default(), fold_partition)
        .items
        - 1
}

fn part_two(data: &str) -> u64 {
    let p = data
        .lines()
        .map(read_line)
        .map(|(s, b)| (s, manhattan_distance(s, b)))
        .combinations(2)
        .filter_map(one_separated)
        .map(get_separation_line)
        .combinations(2)
        .find_map(get_intersection)
        .unwrap_or(Point(0, 0));
    //println!("{p:?}");
    let x = u64::try_from(p.0).unwrap();
    let y = u64::try_from(p.1).unwrap();
    x * 4_000_000 + y
}

#[derive(Debug)]
struct FoldAcc {
    partition: (i32, i32),
    items: i32,
}

impl Default for FoldAcc {
    fn default() -> Self {
        FoldAcc {
            partition: (i32::MIN, i32::MIN),
            items: 0,
        }
    }
}

fn fold_partition(accumulator: FoldAcc, (a, b): (Point, Point)) -> FoldAcc {
    if b.0 <= accumulator.partition.1 {
        return accumulator;
    }
    if a.0 > accumulator.partition.1 + 1 {
        return FoldAcc {
            partition: (a.0, b.0),
            items: accumulator.items + b.0 - a.0 + 1,
        };
    }
    FoldAcc {
        partition: (accumulator.partition.0, b.0),
        items: accumulator.items + b.0 - accumulator.partition.1,
    }
}

fn one_separated(combination: Vec<(Point, u32)>) -> Option<((Point, u32), (Point, u32))> {
    let (s1, d1) = combination[0];
    let (s2, d2) = combination[1];
    if manhattan_distance(s1, s2) == d1 + d2 + 2 {
        Some(((s1, d1), (s2, d2)))
    } else {
        None
    }
}

fn get_separation_line(((s1, d1), (s2, d2)): ((Point, u32), (Point, u32))) -> (Point, Point) {
    let d1 = i32::try_from(d1).unwrap() + 1;
    let d2 = i32::try_from(d2).unwrap() + 1;
    let top: Point;
    let bottom: Point;
    match (s1.0 < s2.0, s1.1 < s2.1) {
        (true, true) => {
            if s1.1 + d1 < s2.1 {
                top = Point(s1.0, s1.1 + d1);
            } else {
                top = Point(s2.0 - d2, s2.1);
            }
            if s1.0 + d1 < s2.0 {
                bottom = Point(s1.0 + d1, s1.1);
            } else {
                bottom = Point(s2.0, s2.1 - d2);
            }
        }
        (false, true) => {
            if s1.1 + d1 < s2.1 {
                top = Point(s1.0, s1.1 + d1);
            } else {
                top = Point(s2.0 - d2, s2.1);
            }
            if s1.0 - d1 > s2.0 {
                bottom = Point(s1.0 - d1, s1.1);
            } else {
                bottom = Point(s2.0, s2.1 - d2);
            }
        }
        (true, false) => {
            if s1.0 + d1 < s2.0 {
                top = Point(s1.0 + d1, s1.1);
            } else {
                top = Point(s2.0, s2.1 + d2);
            }
            if s1.1 - d1 > s2.1 {
                bottom = Point(s1.0, s1.1 - d1);
            } else {
                bottom = Point(s2.0 - d2, s2.1);
            }
        }
        (false, false) => {
            if s1.0 - d1 > s2.0 {
                top = Point(s1.0 - d1, s1.1);
            } else {
                top = Point(s2.0, s2.1 + d2);
            }
            if s1.1 - d1 > s2.0 {
                bottom = Point(s1.0, s1.1 - d1);
            } else {
                bottom = Point(s2.0 - d2, s2.1);
            }
        }
    }
    (top, bottom)
}

fn get_intersection(combination: Vec<(Point, Point)>) -> Option<Point> {
    let line_1 = combination[0];
    let line_2 = combination[1];
    let s1 = slope(line_1);
    let s2 = slope(line_2);
    if s1 == s2 {
        return None;
    }
    let mult = ((line_1.0 .0 - line_2.0 .0) * s1 - (line_2.0 .1 - line_1.0 .1)) / 2;
    if mult < 0 || mult > (line_1.0 .0 - line_1.1 .0).abs() {
        return None;
    }
    Some(Point(line_1.0 .0 - s1 * mult, line_1.0 .1 - mult))
}

fn slope(line: (Point, Point)) -> i32 {
    (line.0 .0 - line.1 .0) / (line.0 .1 - line.1 .1)
}

fn read_line(line: &str) -> (Point, Point) {
    let (sensor, beacon) = line.split_once(':').unwrap();

    let sensor = sensor.trim_start_matches("Sensor at ");
    let (x, y) = sensor.split_once(", ").unwrap();
    let x = x.trim_start_matches("x=").parse().unwrap();
    let y = y.trim_start_matches("y=").parse().unwrap();
    let sensor = Point(x, y);

    let beacon = beacon.trim_start_matches(" closest beacon is at ");
    let (x, y) = beacon.split_once(", ").unwrap();
    let x = x.trim_start_matches("x=").parse().unwrap();
    let y = y.trim_start_matches("y=").parse().unwrap();
    let beacon = Point(x, y);

    (sensor, beacon)
}

fn manhattan_distance(a: Point, b: Point) -> u32 {
    (a.0 - b.0).unsigned_abs() + (a.1 - b.1).unsigned_abs()
}

fn points_on_line<const ROW: i32>((source, size): (Point, u32)) -> Option<(Point, Point)> {
    let y_diff = (source.1 - ROW).unsigned_abs();
    if y_diff > size {
        return None;
    }
    let rem = i32::try_from(size - y_diff).unwrap();
    let point_left = Point(source.0 - rem, ROW);
    let point_right = Point(source.0 + rem, ROW);
    Some((point_left, point_right))
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Point(i32, i32);

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(26, part_one::<10>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(56000011, part_two(data));
    }
}
