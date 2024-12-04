#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    0
}

fn part_two(_data: &str) -> usize {
    0
}

struct Grid {
    data: &'static [u8],
    dimension: u16,
}

impl Grid {
    const fn new(data: &'static str, dimension: u16) -> Self {
        Self { data: data.as_bytes(), dimension }
    }

    const fn loc(&self, index: u16) -> (u16, u16) {
        let row = index / (self.dimension + 1);
        let col = index % (self.dimension + 1);
        (row, col)
    }

    const fn room(&self, loc: (u16, u16), dir: Direction) -> bool {
        match dir {
            Direction::Up => loc.0 >= 3,
            Direction::UpRight => loc.0 >= 3 && loc.1 <= self.dimension - 4,
            Direction::Right => loc.1 <= self.dimension - 4,
            Direction::DownRight => loc.0 <= self.dimension - 4 && loc.1 <= self.dimension - 4,
            Direction::Down => loc.0 <= self.dimension - 4,
            Direction::DownLeft => loc.0 <= self.dimension - 4 && loc.1 >= 3,
            Direction::Left => loc.1 >= 3,
            Direction::UpLeft => loc.0 >= 3 && loc.1 >= 3,
        }
    }

    fn get()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn all() -> [Self; 8] {
        [Self::Up, Self::UpRight, Self::Right, Self::DownRight, Self::Down, Self::DownLeft, Self::Left, Self::UpLeft]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
