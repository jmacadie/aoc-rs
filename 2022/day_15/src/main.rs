use std::collections::HashSet;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data, 2_000_000));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str, row: i32) -> usize {
    let v = data
        .lines()
        .map(read_line)
        .map(|(s, b)| (s, manhatten_distance(s, b)))
        .filter_map(|(s, d)| points_on_line(s, row, d))
        .fold(HashSet::new(), |mut acc, (a, b)| {
            for x in a.0..=b.0 {
                acc.insert(x);
            }
            acc
        });
    v.len() - 1
}

fn part_two(_data: &str) -> usize {
    0
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

fn manhatten_distance(a: Point, b: Point) -> u32 {
    (a.0 - b.0).unsigned_abs() + (a.1 - b.1).unsigned_abs()
}

fn points_on_line(source: Point, row: i32, size: u32) -> Option<(Point, Point)> {
    let y_diff = (source.1 - row).unsigned_abs();
    if y_diff > size {
        return None;
    }
    let rem = i32::try_from(size - y_diff).unwrap();
    let point_left = Point(source.0 - rem, row);
    let point_right = Point(source.0 + rem, row);
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
        assert_eq!(26, part_one(data, 10));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
