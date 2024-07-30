#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use stack::Stack;

#[allow(clippy::missing_panics_doc)]
pub fn main() {
    const N: usize = 1493;
    let data = include_str!("input.txt");
    let s: Stack<N> = data.parse().unwrap();
    println!("Part 1: {}", part_one::<N>(&s));
    println!("Part 2: {}", part_two::<N>(&s));
}

fn part_one<const N: usize>(s: &Stack<N>) -> usize {
    s.disintegratable()
}

fn part_two<const N: usize>(s: &Stack<N>) -> usize {
    s.all_fall()
}

mod stack {
    use super::brick::Brick;
    use super::orientation::Orientation;
    use std::{collections::VecDeque, str::FromStr};

    #[derive(Default)]
    struct Adjacent {
        data: [Option<usize>; 5],
    }

    impl Adjacent {
        fn add(&mut self, brick: usize) {
            *self.data.iter_mut().find(|d| d.is_none()).unwrap() = Some(brick);
        }
    }

    impl Adjacent {
        fn iter(&self) -> impl Iterator<Item = usize> + '_ {
            self.data.iter().filter_map(|d| *d)
        }
    }

    pub struct Stack<const N: usize> {
        above: [Adjacent; N],
        below: [Adjacent; N],
    }

    impl<const N: usize> Stack<N> {
        fn fall(bricks: &mut [Brick; N]) {
            let mut surface = [[0; 10]; 10];
            for b in bricks {
                let x = b.x();
                let y = b.y();
                let length = b.len();
                match b.axis {
                    Orientation::X => {
                        let top = surface[y].iter().skip(x).take(length).max().unwrap() + 1;
                        surface[y]
                            .iter_mut()
                            .skip(x)
                            .take(length)
                            .for_each(|s| *s = top);
                        b.fall(top);
                    }
                    Orientation::Y => {
                        let top = surface
                            .iter()
                            .skip(y)
                            .take(length)
                            .map(|row| row[x])
                            .max()
                            .unwrap()
                            + 1;
                        surface
                            .iter_mut()
                            .skip(y)
                            .take(length)
                            .for_each(|row| row[x] = top);
                        b.fall(top);
                    }
                    Orientation::Z => {
                        let top = surface[y][x] + 1;
                        surface[y][x] = top + u16::from(b.length) - 1;
                        b.fall(top);
                    }
                };
            }
        }

        fn build_adjacent_graph(
            bricks: &[Brick; N],
            above: &mut [Adjacent; N],
            below: &mut [Adjacent; N],
        ) {
            bricks.iter().enumerate().for_each(|(idx_upper, upper)| {
                bricks
                    .iter()
                    .take(idx_upper)
                    .enumerate()
                    .rev()
                    .for_each(|(idx_lower, lower)| {
                        if upper.touching(*lower) {
                            below[idx_upper].add(idx_lower);
                            above[idx_lower].add(idx_upper);
                        }
                    });
            });
        }

        pub fn disintegratable(&self) -> usize {
            self.above
                .iter()
                .filter(|above| {
                    above
                        .iter()
                        .all(|nieghbour| self.below[nieghbour].iter().count() > 1)
                })
                .count()
        }

        fn direct_descendants(&self, source: usize) -> usize {
            let mut affected = Affected::<N>::new();
            affected.insert(source);

            let mut frontier = VecDeque::new();
            frontier.extend(self.above[source].iter());

            while let Some(next) = frontier.pop_front() {
                if self.below[next].iter().all(|node| affected.contains(node)) {
                    frontier.extend(self.above[next].iter());
                    affected.insert(next);
                }
            }

            affected.len() - 1
        }

        pub fn all_fall(&self) -> usize {
            (0..N).map(|node| self.direct_descendants(node)).sum()
        }
    }

    impl<const N: usize> FromStr for Stack<N> {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut bricks = std::array::from_fn(|_| Brick::default());
            for (b, l) in bricks.iter_mut().zip(s.lines()) {
                *b = l.parse()?;
            }
            bricks.sort_unstable();
            let mut above = std::array::from_fn(|_| Adjacent::default());
            let mut below = std::array::from_fn(|_| Adjacent::default());
            Self::fall(&mut bricks);
            Self::build_adjacent_graph(&bricks, &mut above, &mut below);
            Ok(Self { above, below })
        }
    }

    struct Affected<const N: usize> {
        data: [bool; N],
        count: usize,
    }

    impl<const N: usize> Affected<N> {
        const fn new() -> Self {
            Self {
                data: [false; N],
                count: 0,
            }
        }

        fn insert(&mut self, node: usize) {
            if !self.data[node] {
                self.data[node] = true;
                self.count += 1;
            }
        }

        const fn contains(&self, node: usize) -> bool {
            self.data[node]
        }

        const fn len(&self) -> usize {
            self.count
        }
    }
}

mod brick {
    use super::location::Location;
    use super::orientation::Orientation;
    use std::{cmp::Ordering, fmt::Display, str::FromStr};

    #[derive(Default, Clone, Copy)]
    pub struct Brick {
        loc: Location,
        pub axis: Orientation,
        pub length: u8,
    }

    impl Brick {
        pub fn x(self) -> usize {
            self.loc.x()
        }

        pub fn y(self) -> usize {
            self.loc.y()
        }

        pub const fn z(self) -> u16 {
            self.loc.z()
        }

        pub fn len(self) -> usize {
            self.length.into()
        }

        pub fn fall(&mut self, to: u16) {
            self.loc.fall(to);
        }

        pub fn touching(self, lower: Self) -> bool {
            match (self.axis, lower.axis) {
                (Orientation::X, Orientation::X) => {
                    self.z() == lower.z() + 1
                        && self.y() == lower.y()
                        && self.x() < lower.x() + lower.len()
                        && lower.x() < self.x() + self.len()
                }
                (Orientation::X, Orientation::Y) => {
                    self.z() == lower.z() + 1
                        && self.y() >= lower.y()
                        && self.y() < lower.y() + lower.len()
                        && lower.x() >= self.x()
                        && lower.x() < self.x() + self.len()
                }
                (Orientation::X, Orientation::Z) => {
                    self.z() == lower.z() + u16::from(lower.length)
                        && self.y() == lower.y()
                        && lower.x() >= self.x()
                        && lower.x() < self.x() + self.len()
                }
                (Orientation::Y, Orientation::X) => {
                    self.z() == lower.z() + 1
                        && self.x() >= lower.x()
                        && self.x() < lower.x() + lower.len()
                        && lower.y() >= self.y()
                        && lower.y() < self.y() + self.len()
                }
                (Orientation::Y, Orientation::Y) => {
                    self.z() == lower.z() + 1
                        && self.x() == lower.x()
                        && self.y() < lower.y() + lower.len()
                        && lower.y() < self.y() + self.len()
                }
                (Orientation::Y, Orientation::Z) => {
                    self.z() == lower.z() + u16::from(lower.length)
                        && self.x() == lower.x()
                        && lower.y() >= self.y()
                        && lower.y() < self.y() + self.len()
                }
                (Orientation::Z, Orientation::X) => {
                    self.z() == lower.z() + 1
                        && self.x() >= lower.x()
                        && self.x() < lower.x() + lower.len()
                        && lower.y() == self.y()
                }
                (Orientation::Z, Orientation::Y) => {
                    self.z() == lower.z() + 1
                        && self.y() >= lower.y()
                        && self.y() < lower.y() + lower.len()
                        && lower.x() == self.x()
                }
                (Orientation::Z, Orientation::Z) => {
                    self.x() == lower.x()
                        && self.y() == lower.y()
                        && self.z() == lower.z() + u16::from(lower.length)
                }
            }
        }
    }

    impl Display for Brick {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{0} {1}{2}", self.loc, self.length, self.axis)?;
            Ok(())
        }
    }

    impl FromStr for Brick {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            fn coords(s: &str) -> Result<(u8, u8, u16), String> {
                let mut parts = s.split(',');
                let (Some(x), Some(y), Some(z), None) =
                    (parts.next(), parts.next(), parts.next(), parts.next())
                else {
                    return Err(format!("split co-ordinates {s} into three with commas"));
                };
                let x = x.parse().map_err(|_| format!("convert {x} to a number"))?;
                let y = y.parse().map_err(|_| format!("convert {y} to a number"))?;
                let z = z.parse().map_err(|_| format!("convert {z} to a number"))?;
                Ok((x, y, z))
            }
            let (start, end) = s.split_once('~').ok_or("split on a tilda")?;
            let (x1, y1, z1) = coords(start)?;
            let (x2, y2, z2) = coords(end)?;
            let loc = (x1, y1, z1).into();
            match (x1.cmp(&x2), y1.cmp(&y2), z1.cmp(&z2)) {
                (Ordering::Less, Ordering::Equal, Ordering::Equal) => Ok(Self {
                    loc,
                    axis: Orientation::X,
                    length: x2 - x1 + 1,
                }),
                (Ordering::Equal, Ordering::Less, Ordering::Equal) => Ok(Self {
                    loc,
                    axis: Orientation::Y,
                    length: y2 - y1 + 1,
                }),
                (Ordering::Equal, Ordering::Equal, Ordering::Less) => {
                    let length = u8::try_from(z2 - z1 + 1)
                        .map_err(|_| format!("vertical length {z1} - {z2} doesn't fit in a u8"))?;
                    Ok(Self {
                        loc,
                        axis: Orientation::Z,
                        length,
                    })
                }
                (Ordering::Equal, Ordering::Equal, Ordering::Equal) => Ok(Self {
                    loc,
                    axis: Orientation::Z,
                    length: 1,
                }),
                (_, _, _) => unreachable!(),
            }
        }
    }

    impl PartialEq for Brick {
        fn eq(&self, other: &Self) -> bool {
            self.loc == other.loc
        }
    }

    impl Eq for Brick {}

    impl PartialOrd for Brick {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Brick {
        fn cmp(&self, other: &Self) -> Ordering {
            self.loc.cmp(&other.loc)
        }
    }
}

mod location {
    use std::fmt::Display;

    #[derive(Default, Clone, Copy)]
    pub struct Location {
        plane: u8,
        vertical: u16,
    }

    impl Location {
        pub fn x(self) -> usize {
            usize::from(self.plane >> 4)
        }

        pub fn y(self) -> usize {
            usize::from(self.plane & 0x0f)
        }

        pub const fn z(self) -> u16 {
            self.vertical
        }

        pub fn fall(&mut self, to: u16) {
            self.vertical = to;
        }
    }

    impl From<(u8, u8, u16)> for Location {
        fn from(value: (u8, u8, u16)) -> Self {
            let plane = (value.0 << 4) | (0x0f & value.1);
            Self {
                plane,
                vertical: value.2,
            }
        }
    }

    impl Display for Location {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let x = self.plane >> 4;
            let y = self.plane & 0x0f;
            write!(f, "({x}, {y}, {0})", self.vertical)?;
            Ok(())
        }
    }

    impl PartialEq for Location {
        fn eq(&self, other: &Self) -> bool {
            self.vertical == other.vertical
        }
    }

    impl Eq for Location {}

    impl PartialOrd for Location {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Location {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.vertical.cmp(&other.vertical)
        }
    }
}

mod orientation {
    use std::fmt::Display;

    #[derive(Default, Clone, Copy)]
    pub enum Orientation {
        #[default]
        X,
        Y,
        Z,
    }

    impl Display for Orientation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::X => write!(f, "x")?,
                Self::Y => write!(f, "y")?,
                Self::Z => write!(f, "z")?,
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        const N: usize = 7;
        let data = include_str!("test.txt");
        let s: Stack<N> = data.parse().unwrap();
        assert_eq!(5, part_one::<N>(&s));
    }

    #[test]
    fn two() {
        const N: usize = 7;
        let data = include_str!("test.txt");
        let s: Stack<N> = data.parse().unwrap();
        assert_eq!(7, part_two::<N>(&s));
    }
}
