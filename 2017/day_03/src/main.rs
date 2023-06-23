#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let goal: u32 = data.trim().parse().unwrap();

    let ring = ring(goal);

    ring + distance_to_middle(goal, ring)
}

fn part_two(data: &str) -> u32 {
    let goal: u32 = data.trim().parse().unwrap();

    let mut gd = Grid::new();

    while gd.get_val() < goal {
        gd.step();
    }

    gd.get_val()
}

const fn distance_to_middle(target: u32, ring: u32) -> u32 {
    let base = ring * 2;

    // The final numbers in each ring are 1, 9, 25, 49...
    // i.e. the series of squares of the odd numbers
    let ring_limit = (base + 1).pow(2);

    let distance = ring_limit - target;
    let side_distance = distance % base;

    if ring > side_distance {
        ring - side_distance
    } else {
        side_distance - ring
    }
}

fn ring(target: u32) -> u32 {
    // Get the integer square root of one less than the target
    let sqrt = int_sqrt(target - 1);

    (sqrt + 1) / 2
}

// https://en.wikipedia.org/wiki/Integer_square_root#Algorithm_using_Newton's_method
fn int_sqrt(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    // Closure to calculate start value
    let first = |x: u32| {
        let log = 31 - x.leading_zeros();
        2_u32.pow((log / 2) + 1)
    };

    // Closure to compute the next value
    let next = |x: u32| (x + (n / x)) / 2;

    // Initial estimate (must be too high)
    //let mut x0 = n / 2;
    let mut x0 = first(n);

    // Next guess
    let mut x1 = next(x0);

    while x1 < x0 {
        x0 = x1;
        x1 = next(x0);
    }

    x0
}

struct Grid {
    data: [[u32; 11]; 11],
    current: Point,
}

impl Grid {
    fn new() -> Self {
        let mut out = Self::new_empty();
        out.set_val(1);
        out
    }

    fn new_empty() -> Self {
        Self {
            data: [[0; 11]; 11],
            current: Point::default(),
        }
    }

    fn set_val(&mut self, val: u32) {
        let x = usize::try_from(self.current.0 + 5).unwrap();
        let y = usize::try_from(self.current.1 + 5).unwrap();
        self.data[y][x] = val;
    }

    fn get_val_at(&self, loc: Point) -> u32 {
        let x = usize::try_from(loc.0 + 5).unwrap();
        let y = usize::try_from(loc.1 + 5).unwrap();
        self.data[y][x]
    }

    fn get_val(&self) -> u32 {
        self.get_val_at(self.current)
    }

    fn move_next(&mut self) {
        let next_point = self.next();
        self.current = next_point;
    }

    fn step(&mut self) {
        self.move_next();
        let val = self
            .current
            .nieghbours()
            .into_iter()
            .map(|p| self.get_val_at(p))
            .sum();
        self.set_val(val);
    }

    fn next(&self) -> Point {
        let ring = std::cmp::max(self.current.0.abs(), self.current.1.abs());

        if self.current.1 == ring {
            Point(self.current.0 + 1, self.current.1)
        } else if self.current.0 == -ring {
            Point(self.current.0, self.current.1 + 1)
        } else if self.current.1 == -ring {
            Point(self.current.0 - 1, self.current.1)
        } else {
            //if self.current.0 == ring {
            Point(self.current.0, self.current.1 - 1)
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
struct Point(i8, i8);

impl Point {
    const fn nieghbours(self) -> [Self; 8] {
        [
            Self(self.0 + 1, self.1 + 1),
            Self(self.0, self.1 + 1),
            Self(self.0 - 1, self.1 + 1),
            Self(self.0 - 1, self.1),
            Self(self.0 - 1, self.1 - 1),
            Self(self.0, self.1 - 1),
            Self(self.0 + 1, self.1 - 1),
            Self(self.0 + 1, self.1),
        ]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sqrt() {
        assert_eq!(0, int_sqrt(0));
        assert_eq!(1, int_sqrt(1));
        assert_eq!(1, int_sqrt(2));
        assert_eq!(1, int_sqrt(3));
        assert_eq!(2, int_sqrt(4));
        assert_eq!(2, int_sqrt(8));
        assert_eq!(3, int_sqrt(9));
        assert_eq!(1_414, int_sqrt(2_000_000));
    }

    #[test]
    fn test_ring() {
        assert_eq!(0, ring(1));
        assert_eq!(1, ring(2));
        assert_eq!(1, ring(3));
        assert_eq!(1, ring(4));
        assert_eq!(1, ring(5));
        assert_eq!(1, ring(8));
        assert_eq!(1, ring(9));
        assert_eq!(2, ring(10));
        assert_eq!(2, ring(16));
        assert_eq!(2, ring(25));
        assert_eq!(3, ring(26));
    }

    #[test]
    fn one() {
        assert_eq!(1, part_one("2"));
        assert_eq!(2, part_one("3"));
        assert_eq!(1, part_one("4"));
        assert_eq!(2, part_one("5"));
        assert_eq!(1, part_one("6"));
        assert_eq!(2, part_one("7"));
        assert_eq!(1, part_one("8"));
        assert_eq!(2, part_one("9"));
        assert_eq!(3, part_one("10"));
    }

    #[test]
    fn new_grid() {
        let gd = Grid::new();
        assert_eq!(gd.current, Point(0, 0));
        assert_eq!(gd.get_val(), 1);
    }

    #[test]
    fn grid_next_point() {
        let mut gd = Grid::new();
        gd.move_next();
        assert_eq!(gd.current, Point(1, 0));
        gd.move_next();
        assert_eq!(gd.current, Point(1, -1));
        gd.move_next();
        assert_eq!(gd.current, Point(0, -1));
        gd.move_next();
        assert_eq!(gd.current, Point(-1, -1));
        gd.move_next();
        assert_eq!(gd.current, Point(-1, 0));
        gd.move_next();
        assert_eq!(gd.current, Point(-1, 1));
        gd.move_next();
        assert_eq!(gd.current, Point(0, 1));
        gd.move_next();
        assert_eq!(gd.current, Point(1, 1));
        gd.move_next();
        assert_eq!(gd.current, Point(2, 1));
        for _ in 0..7 {
            gd.move_next();
        }
        assert_eq!(gd.current, Point(-2, -2));
        for _ in 0..13 {
            gd.move_next();
        }
        assert_eq!(gd.current, Point(3, -2));
    }

    #[test]
    fn grid_step() {
        let mut gd = Grid::new();
        gd.step();
        assert_eq!(gd.current, Point(1, 0));
        assert_eq!(gd.get_val(), 1);
        gd.step();
        assert_eq!(gd.current, Point(1, -1));
        assert_eq!(gd.get_val(), 2);
        gd.step();
        assert_eq!(gd.current, Point(0, -1));
        assert_eq!(gd.get_val(), 4);
        for _ in 0..5 {
            gd.step();
        }
        assert_eq!(gd.current, Point(1, 1));
        assert_eq!(gd.get_val(), 25);
        for _ in 0..10 {
            gd.step();
        }
        assert_eq!(gd.current, Point(-2, 0));
        assert_eq!(gd.get_val(), 330);
    }
}
