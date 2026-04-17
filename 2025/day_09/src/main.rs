#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{cmp::Ordering, str::FromStr};

pub fn main() {
    const POINTS: usize = 498;
    const LINES: usize = POINTS / 2 - 1;
    let data = include_str!("input.txt");
    let tiles = prep::<POINTS, LINES>(data);
    println!("Part 1: {}", part_one::<POINTS, LINES>(&tiles));
    println!("Part 2: {}", part_two::<POINTS, LINES>(&tiles));
}

fn prep<const N: usize, const M: usize>(data: &str) -> Tiles<N, M> {
    let mut points: [Point; N] = std::array::from_fn(|_| Point::default());
    for (i, p) in data
        .lines()
        .map(|l| l.parse::<Point>().unwrap())
        .enumerate()
    {
        points[i + 1] = p;
    }
    // Repeat the first and last points, so we can look at the lines either side of the
    // terminal points
    points[0] = points[N - 2];
    points[N - 1] = points[1];

    Tiles::<N, M>::new(points)
}

fn part_one<const N: usize, const M: usize>(tiles: &Tiles<N, M>) -> usize {
    let mut max = 0;
    for i in 1..N - 1 {
        for j in i + 1..N - 1 {
            max = std::cmp::max(max, tiles.area((i, j)));
        }
    }
    max
}

fn part_two<const N: usize, const M: usize>(tiles: &Tiles<N, M>) -> usize {
    let mut max = 0;
    for i in 1..N - 1 {
        for j in i + 1..N - 1 {
            let area = tiles.area((i, j));
            if area > max && tiles.fully_interior((i, j)) {
                max = area;
            }
        }
    }
    max
}

struct Tiles<const N: usize, const M: usize> {
    points: [Point; N],
    verticals: [Line; M],
    horizontals: [Line; M],
    chirality: Chirality,
}

impl<const N: usize, const M: usize> Tiles<N, M> {
    fn new(points: [Point; N]) -> Self {
        let chirality = Self::find_chirality(&points);
        let (verticals, horizontals) = Self::get_lines(&points);
        Self {
            points,
            verticals,
            horizontals,
            chirality,
        }
    }

    const fn area(&self, rect: (usize, usize)) -> usize {
        self.points[rect.0].area(self.points[rect.1])
    }

    fn fully_interior(&self, rect: (usize, usize)) -> bool {
        self.is_interior(rect) && self.no_intersect(rect)
    }

    fn no_intersect(&self, rect: (usize, usize)) -> bool {
        let (rect_0, rect_1) = (self.points[rect.0], self.points[rect.1]);
        let from_row = std::cmp::min(rect_0.row, rect_1.row);
        let to_row = std::cmp::max(rect_0.row, rect_1.row);
        let from_col = std::cmp::min(rect_0.col, rect_1.col);
        let to_col = std::cmp::max(rect_0.col, rect_1.col);

        let from_row_idx = self
            .horizontals
            .binary_search_by_key(&from_row, |l| l.start.row)
            .unwrap();
        let from_col_idx = self
            .verticals
            .binary_search_by_key(&from_col, |l| l.start.col)
            .unwrap();

        self.horizontals
            .iter()
            .skip(from_row_idx + 1)
            .take_while(|l| l.start.row < to_row)
            .all(|l| l.start.row == from_row || !l.horz_intersect(from_col, to_col))
            && self
                .verticals
                .iter()
                .skip(from_col_idx + 1)
                .take_while(|l| l.start.col < to_col)
                .all(|l| l.start.col == from_col || !l.vert_intersect(from_row, to_row))
    }

    fn is_interior(&self, rect: (usize, usize)) -> bool {
        let is_interior_one = |this_point: usize, other_point: usize| {
            let this = self.points[this_point];
            let other = self.points[other_point];
            let first = Line::new(this, self.points[this_point - 1]);
            let second = Line::new(this, self.points[this_point + 1]);
            // find the edge cases where the rectangle is a single line, aligned with one of the lines
            // from the point
            if first.aligned_points((this, other)) || second.aligned_points((this, other)) {
                return true;
            }

            let resolved_1 = first.collapse_onto(other);
            let resolved_2 = second.collapse_onto(other);
            if Self::chirality_at_point(&self.points[this_point - 1..this_point + 2])
                == self.chirality
            {
                // if the direction of turn matches the overall direction of the interior then the
                // rectangle must sit in that inner quadrant
                first.aligned(resolved_1) && second.aligned(resolved_2)
            } else {
                // otherwise it must sit in one of the three outer quadrants
                !first.aligned(resolved_1) || !second.aligned(resolved_2)
            }
        };

        is_interior_one(rect.0, rect.1) && is_interior_one(rect.1, rect.0)
    }

    fn find_chirality(points: &[Point]) -> Chirality {
        let count = points
            .windows(3)
            .map(Self::chirality_at_point)
            .fold(0, |acc, ch| {
                if ch == Chirality::Right {
                    acc + 1
                } else {
                    acc - 1
                }
            });
        if count == 4 {
            Chirality::Right
        } else {
            // We must have turned 4 times in aggregate.
            // If not, there has been some kind of error
            assert_eq!(count, -4);
            Chirality::Left
        }
    }

    fn chirality_at_point(three_points: &[Point]) -> Chirality {
        // We must be working with a set of 3 consecutive points, in order to create two consecutive lines
        assert_eq!(three_points.len(), 3);
        let first = Line::new(three_points[0], three_points[1]);
        let second = Line::new(three_points[1], three_points[2]);

        match (first.direction, second.direction) {
            (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Left, Direction::Up) => Chirality::Right,
            (Direction::Up, Direction::Left)
            | (Direction::Left, Direction::Down)
            | (Direction::Down, Direction::Right)
            | (Direction::Right, Direction::Up) => Chirality::Left,
            _ => unreachable!(),
        }
    }

    fn get_lines(points: &[Point]) -> ([Line; M], [Line; M]) {
        let first = Line::new(points[1], points[2]).direction;
        let vert_offset = usize::from(first == Direction::Left || first == Direction::Right);
        let mut verticals = std::array::from_fn(|i| {
            let idx = 2 * i + vert_offset + 1;
            Line::new_sorted(points[idx], points[idx + 1])
        });

        let hor_offset = usize::from(vert_offset != 1);
        let mut horizontals = std::array::from_fn(|i| {
            let idx = 2 * i + hor_offset + 1;
            Line::new_sorted(points[idx], points[idx + 1])
        });

        verticals.sort_unstable_by_key(|l| l.start.col);
        horizontals.sort_unstable_by_key(|l| l.start.row);

        (verticals, horizontals)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chirality {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        let direction = match (start.row.cmp(&end.row), start.col.cmp(&end.col)) {
            (Ordering::Equal, Ordering::Less) => Direction::Right,
            (Ordering::Equal, Ordering::Greater) => Direction::Left,
            (Ordering::Less, Ordering::Equal) => Direction::Down,
            (Ordering::Greater, Ordering::Equal) => Direction::Up,
            (Ordering::Equal, Ordering::Equal) => Direction::None,
            _ => unreachable!(),
        };
        Self {
            start,
            end,
            direction,
        }
    }

    fn new_sorted(start: Point, end: Point) -> Self {
        match (start.row.cmp(&end.row), start.col.cmp(&end.col)) {
            (Ordering::Equal, Ordering::Less) | (Ordering::Less, Ordering::Equal) => {
                Self::new(start, end)
            }
            (Ordering::Equal, Ordering::Greater) | (Ordering::Greater, Ordering::Equal) => {
                Self::new(end, start)
            }
            _ => unreachable!(),
        }
    }

    fn aligned(&self, other: Self) -> bool {
        self.direction == other.direction
    }

    const fn aligned_points(&self, other: (Point, Point)) -> bool {
        match self.direction {
            Direction::Right => other.0.row == other.1.row && other.0.col < other.1.col,
            Direction::Left => other.0.row == other.1.row && other.0.col > other.1.col,
            Direction::Down => other.0.col == other.1.col && other.0.row < other.1.row,
            Direction::Up => other.0.col == other.1.col && other.0.row > other.1.row,
            Direction::None => false,
        }
    }

    fn collapse_onto(&self, other: Point) -> Self {
        let end = match self.direction {
            Direction::Up | Direction::Down => Point {
                row: other.row,
                col: self.start.col,
            },
            Direction::Left | Direction::Right => Point {
                row: self.start.row,
                col: other.col,
            },
            Direction::None => self.end,
        };
        Self::new(self.start, end)
    }

    const fn vert_intersect(&self, from: usize, to: usize) -> bool {
        match self.direction {
            Direction::Down => to > self.start.row && from < self.end.row,
            Direction::Up => to > self.end.row && from < self.start.row,
            _ => false,
        }
    }

    const fn horz_intersect(&self, from: usize, to: usize) -> bool {
        match self.direction {
            Direction::Right => to > self.start.col && from < self.end.col,
            Direction::Left => to > self.end.col && from < self.start.col,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, col) = s
            .split_once(',')
            .ok_or_else(|| format!("Cannot split {s} into two parts"))?;
        let row = row.parse().map_err(|_| {
            format!("{row} will not be made into a number. You need to have a word")
        })?;
        let col = col.parse().map_err(|_| {
            format!("{col} will not be made into a number. You need to have a word")
        })?;
        Ok(Self { row, col })
    }
}

impl Point {
    const fn area(&self, other: Self) -> usize {
        let dx = other.row.abs_diff(self.row);
        let dy = other.col.abs_diff(self.col);
        (dx + 1) * (dy + 1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        const POINTS: usize = 10;
        const LINES: usize = POINTS / 2 - 1;
        let data = include_str!("test.txt");
        let tiles = prep::<POINTS, LINES>(data);
        assert_eq!(50, part_one(&tiles));
    }

    #[test]
    fn two() {
        const POINTS: usize = 10;
        const LINES: usize = POINTS / 2 - 1;
        let data = include_str!("test.txt");
        let tiles = prep::<POINTS, LINES>(data);
        assert_eq!(24, part_two(&tiles));
    }
}
