use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    single_sides(data).len()
}

fn part_two(data: &str) -> usize {
    let (cubes, bound) = get_cubes(data);
    let outer = get_outer(&cubes, bound);
    let inner = get_inner(&cubes, &outer, bound);
    let interior_sides = inner
        .into_iter()
        .flat_map(point_to_cube)
        .sorted_unstable()
        .dedup_with_count()
        .filter(|&(c, _)| c == 1)
        .map(|(_, p)| p)
        .collect_vec();
    single_sides(data).len() - interior_sides.len()
}

fn read_line(line: &str) -> Point {
    let mut parts = line.split(',');
    let (Some(x), Some(y), Some(z), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
        unreachable!();
    };
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    let z = z.parse().unwrap();
    Point { x, y, z }
}

fn single_sides(data: &str) -> Vec<Side> {
    data.lines()
        .map(read_line)
        .flat_map(point_to_cube)
        .sorted_unstable()
        .dedup_with_count()
        .filter(|&(c, _)| c == 1)
        .map(|(_, p)| p)
        .collect_vec()
}

fn get_cubes(data: &str) -> (Vec<Point>, Point) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    let mut cubes: Vec<Point> = Vec::new();
    for line in data.lines() {
        let p = read_line(line);
        max_x = std::cmp::max(p.x, max_x);
        max_y = std::cmp::max(p.y, max_y);
        max_z = std::cmp::max(p.z, max_z);
        cubes.push(p);
    }
    let bound = Point {
        x: max_x + 1,
        y: max_y + 1,
        z: max_z + 1,
    };
    (cubes, bound)
}

fn get_outer(cubes: &[Point], bound: Point) -> Vec<Point> {
    let mut frontier = Vec::new();
    let mut outer = Vec::new();
    let zero = Point { x: 0, y: 0, z: 0 };
    frontier.push(zero);
    outer.push(zero);
    while !frontier.is_empty() {
        let mut next = Vec::new();
        while !frontier.is_empty() {
            let point = frontier.pop().unwrap();
            for neighbour in point.get_neighbours(bound) {
                if !cubes.contains(&neighbour) && !outer.contains(&neighbour) {
                    next.push(neighbour);
                    outer.push(neighbour);
                }
            }
        }
        frontier = next;
    }
    outer
}

fn get_inner(cubes: &[Point], outer: &[Point], bound: Point) -> Vec<Point> {
    let mut inner = Vec::new();
    for x in 0..=bound.x {
        for y in 0..=bound.y {
            for z in 0..=bound.z {
                let p = Point { x, y, z };
                if !cubes.contains(&p) && !outer.contains(&p) {
                    inner.push(p);
                }
            }
        }
    }
    inner
}

fn point_to_cube(source: Point) -> Cube {
    let (x, y, z) = (source.x, source.y, source.z);
    [
        Side(
            Point { x, y, z },
            Point { x: x + 1, y, z },
            Point {
                x: x + 1,
                y: y + 1,
                z,
            },
            Point { x, y: y + 1, z },
        ),
        Side(
            Point { x, y, z },
            Point { x: x + 1, y, z },
            Point {
                x: x + 1,
                y,
                z: z + 1,
            },
            Point { x, y, z: z + 1 },
        ),
        Side(
            Point { x, y, z },
            Point { x, y: y + 1, z },
            Point {
                x,
                y: y + 1,
                z: z + 1,
            },
            Point { x, y, z: z + 1 },
        ),
        Side(
            Point { x, y, z: z + 1 },
            Point {
                x: x + 1,
                y,
                z: z + 1,
            },
            Point {
                x: x + 1,
                y: y + 1,
                z: z + 1,
            },
            Point {
                x,
                y: y + 1,
                z: z + 1,
            },
        ),
        Side(
            Point { x, y: y + 1, z },
            Point {
                x: x + 1,
                y: y + 1,
                z,
            },
            Point {
                x: x + 1,
                y: y + 1,
                z: z + 1,
            },
            Point {
                x,
                y: y + 1,
                z: z + 1,
            },
        ),
        Side(
            Point { x: x + 1, y, z },
            Point {
                x: x + 1,
                y: y + 1,
                z,
            },
            Point {
                x: x + 1,
                y: y + 1,
                z: z + 1,
            },
            Point {
                x: x + 1,
                y,
                z: z + 1,
            },
        ),
    ]
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
    z: u8,
}

impl Point {
    fn get_neighbours(&self, bound: Point) -> Vec<Point> {
        let mut neighbours = Vec::new();
        if self.x > 0 {
            neighbours.push(Point {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            });
        }
        if self.y > 0 {
            neighbours.push(Point {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            });
        }
        if self.z > 0 {
            neighbours.push(Point {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            });
        }
        if self.x < bound.x {
            neighbours.push(Point {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            });
        }
        if self.y < bound.y {
            neighbours.push(Point {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            });
        }
        if self.z < bound.z {
            neighbours.push(Point {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            });
        }
        neighbours
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
struct Side(Point, Point, Point, Point);

type Cube = [Side; 6];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(64, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(58, part_two(data));
    }
}
