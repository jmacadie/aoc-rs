#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<140>(data));
    println!("Part 2: {}", part_two::<140>(data));
}

fn part_one<const N: usize>(data: &'static str) -> usize {
    let m = OwnedMap::<N>::new(data);
    m.count_single_moveable()
}

fn part_two<const N: usize>(data: &str) -> usize {
    let mut m = OwnedMap::<N>::new(data);
    m.count_all_moveable()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.row.cmp(&other.row) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
        }
    }
}

impl Point {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

struct OwnedMap<const N: usize> {
    data: [[bool; N]; N],
    points: [[Point; N]; N],
}

impl<const N: usize> OwnedMap<N> {
    fn new(s: &str) -> Self {
        let data = std::array::from_fn(|row| {
            std::array::from_fn(|col| s.as_bytes()[row * (N + 1) + col] == b'@')
        });
        let points = std::array::from_fn(|row| std::array::from_fn(|col| Point::new(row, col)));
        Self { data, points }
    }

    const fn all_points(&self) -> &[Point] {
        // This is safe as `self.points` is a nested array that is guaranteed to be laid out
        // contigously in memory. We are flattening the nested array and returning a slice
        // over the points data
        unsafe { std::slice::from_raw_parts(self.points.as_ptr().cast::<Point>(), N * N) }
    }

    fn count_single_moveable(&self) -> usize {
        self.count_moveable_layer(self.all_points()).0
    }

    fn count_all_moveable(&mut self) -> usize {
        let mut count = 0;
        let (mut layer_count, mut frontier, mut data) =
            self.count_moveable_layer(self.all_points());

        while layer_count > 0 {
            self.data = data;
            count += layer_count;
            (layer_count, frontier, data) = self.count_moveable_layer(&frontier);
        }
        count
    }

    fn count_moveable_layer(&self, frontier: &[Point]) -> (usize, Vec<Point>, [[bool; N]; N]) {
        let mut next_data = self.data;
        let mut next_frontier = Vec::new();

        let count = frontier
            .iter()
            .filter(|&p| {
                self.val(p) && Self::neighbours(p).filter(|n| self.val(n)).nth(3).is_none()
            })
            .fold(0, |count, p| {
                next_data[p.row][p.col] = false;
                next_frontier.extend(Self::neighbours(p).filter(|n| self.val(n)));
                count + 1
            });

        // Sort and unique the frontier to save counting points twice (or more)
        next_frontier.sort_unstable();
        next_frontier.dedup();

        (count, next_frontier, next_data)
    }

    const fn val(&self, posn: &Point) -> bool {
        self.data[posn.row][posn.col]
    }

    fn neighbours(posn: &Point) -> impl Iterator<Item = Point> {
        match *posn {
            Point { row: 0, col: 0 } => [
                // top-left corner
                Some(Point::new(0, 1)),
                Some(Point::new(1, 0)),
                Some(Point::new(1, 1)),
                None,
                None,
                None,
                None,
                None,
            ],
            Point { row: 0, col: c } if c == N - 1 => [
                // top-right corner
                Some(Point::new(0, c - 1)),
                Some(Point::new(1, c - 1)),
                Some(Point::new(1, c)),
                None,
                None,
                None,
                None,
                None,
            ],
            Point { row: r, col: 0 } if r == N - 1 => [
                // bottom-left corner
                Some(Point::new(r - 1, 0)),
                Some(Point::new(r - 1, 1)),
                Some(Point::new(r, 1)),
                None,
                None,
                None,
                None,
                None,
            ],
            Point { row: r, col: c } if r == N - 1 && c == N - 1 => [
                // bottom-right corner
                Some(Point::new(r - 1, c - 1)),
                Some(Point::new(r - 1, c)),
                Some(Point::new(r, c - 1)),
                None,
                None,
                None,
                None,
                None,
            ],
            Point { row: 0, col: c } => [
                // top
                Some(Point::new(0, c - 1)),
                Some(Point::new(1, c - 1)),
                Some(Point::new(1, c)),
                Some(Point::new(1, c + 1)),
                Some(Point::new(0, c + 1)),
                None,
                None,
                None,
            ],
            Point { row: r, col: c } if r == N - 1 => [
                // bottom
                Some(Point::new(r, c - 1)),
                Some(Point::new(r - 1, c - 1)),
                Some(Point::new(r - 1, c)),
                Some(Point::new(r - 1, c + 1)),
                Some(Point::new(r, c + 1)),
                None,
                None,
                None,
            ],
            Point { row: r, col: 0 } => [
                // left
                Some(Point::new(r - 1, 0)),
                Some(Point::new(r - 1, 1)),
                Some(Point::new(r, 1)),
                Some(Point::new(r + 1, 1)),
                Some(Point::new(r + 1, 0)),
                None,
                None,
                None,
            ],
            Point { row: r, col: c } if c == N - 1 => [
                // right
                Some(Point::new(r - 1, c)),
                Some(Point::new(r - 1, c - 1)),
                Some(Point::new(r, c - 1)),
                Some(Point::new(r + 1, c - 1)),
                Some(Point::new(r + 1, c)),
                None,
                None,
                None,
            ],
            Point { row: r, col: c } => [
                Some(Point::new(r - 1, c - 1)),
                Some(Point::new(r - 1, c)),
                Some(Point::new(r - 1, c + 1)),
                Some(Point::new(r, c + 1)),
                Some(Point::new(r + 1, c + 1)),
                Some(Point::new(r + 1, c)),
                Some(Point::new(r + 1, c - 1)),
                Some(Point::new(r, c - 1)),
            ],
        }
        .into_iter()
        .flatten()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(13, part_one::<10>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(43, part_two::<10>(data));
    }
}
