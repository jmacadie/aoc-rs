#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    let f = Factory::new(data);
    solve(&f)
}

fn part_two(data: &'static str) -> usize {
    let mut f = Factory::new(data);
    // Add the extra generators and chips ;)
    f.floors[0] = (f.floors[0].0 | 0b0000_0000_0011_1100).into();
    solve(&f)
}

/// A* algorithm applied to successive copies of `Factory`
fn solve(f: &Factory) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    visited.insert((f.elevator, f.summary));
    for m in f.valid_moves() {
        heap.push(m);
    }

    while let Some(f) = heap.pop() {
        if f.distance() == 0 {
            return f.step;
        }
        if visited.contains(&(f.elevator, f.summary)) {
            continue;
        }
        visited.insert((f.elevator, f.summary));
        for m in f.valid_moves() {
            heap.push(m);
        }
    }
    0
}

/// The state of the factory, after a series of steps.
/// Holds state information so that the A* algorithm (in solve) can
/// be applied to various copies of this type
#[derive(Debug, PartialEq, Eq)]
struct Factory {
    elevator: usize,
    floors: [Floor; 4],
    summary: [[u8; 4]; 4],
    step: usize,
    heuristic: usize,
}

impl Factory {
    fn new(data: &'static str) -> Self {
        let mut temp: [(&str, [usize; 2]); 8] = [("", [0; 2]); 8];
        for (i, line) in data.lines().enumerate() {
            let (_, info) = line.trim_end_matches('.').split_once(" contains ").unwrap();
            if info == "nothing relevant" {
                continue;
            }
            let level = i + 1;
            for part in info.split(", ") {
                let part = part.trim_start_matches("and ");
                if let Some((a, b)) = part.split_once(" and ") {
                    Self::parse_part(a, level, &mut temp);
                    Self::parse_part(b, level, &mut temp);
                } else {
                    Self::parse_part(part, level, &mut temp);
                }
            }
        }
        let floors = std::array::from_fn(|i| Self::build_floor(&temp, i + 1));
        let mut out = Self {
            elevator: 0,
            floors,
            summary: [[0; 4]; 4],
            step: 0,
            heuristic: 0,
        };
        out.compute_hueristic();
        out.compute_summary();
        out
    }

    fn build_floor(data: &[(&'static str, [usize; 2])], level: usize) -> Floor {
        let mut temp = 0;
        for (_, vals) in data.iter() {
            temp <<= 1;
            if vals[0] == level {
                temp |= 1;
            }
            temp <<= 1;
            if vals[1] == level {
                temp |= 1;
            }
        }
        temp.into()
    }

    fn parse_part(part: &'static str, level: usize, data: &mut [(&'static str, [usize; 2])]) {
        let (mut elem, part_type) = part
            .trim_start_matches("a ")
            .trim_start_matches("an ")
            .split_once(' ')
            .unwrap();
        match part_type {
            "generator" => (),
            "microchip" => {
                elem = elem.trim_end_matches("-compatible");
            }
            _ => unreachable!(),
        }
        for (e, v) in data {
            if e == &elem || e == &"" {
                *e = elem;
                match part_type {
                    "generator" => v[0] = level,
                    "microchip" => v[1] = level,
                    _ => unreachable!(),
                }
                break;
            }
        }
    }

    fn valid_moves(&self) -> Vec<Self> {
        let current_floor = self.floors[self.elevator];
        let moves_from = current_floor.all_valid_moves_from();
        let mut moves = Vec::new();

        // Move up
        if self.elevator < 3 {
            let target_floor = self.floors[self.elevator + 1];
            moves.extend(
                moves_from
                    .iter()
                    .filter(|&&m| target_floor.valid_move_to(m))
                    .map(|f| ElevatorMove::from((self.elevator + 1, *f)))
                    .map(|m| self.move_elevator(m)),
            );
        }

        // Move down
        if self.elevator > 0 {
            let target_floor = self.floors[self.elevator - 1];
            moves.extend(
                moves_from
                    .iter()
                    .filter(|&&m| target_floor.valid_move_to(m))
                    .map(|f| ElevatorMove::from((self.elevator - 1, *f)))
                    .map(|m| self.move_elevator(m)),
            );
        }

        moves
    }

    fn distance(&self) -> usize {
        self.floors
            .iter()
            .rev()
            .enumerate()
            .map(|(i, f)| i * usize::try_from(f.0.count_ones()).unwrap())
            .sum()
    }

    fn compute_hueristic(&mut self) {
        self.heuristic = self.step + self.distance() / 2;
    }

    fn compute_summary(&mut self) {
        for (s, f) in self.summary.iter_mut().zip(self.floors.iter()) {
            *s = f.summarise();
        }
    }

    fn move_elevator(&self, m: ElevatorMove) -> Self {
        let mut floors = self.floors;
        for (i, f) in floors.iter_mut().enumerate() {
            match i {
                _ if i == self.elevator => *f = f.move_from(m.moved),
                _ if i == m.move_to => *f = f.move_to(m.moved),
                _ => (),
            };
        }
        let mut out = Self {
            elevator: m.move_to,
            floors,
            summary: [[0; 4]; 4],
            step: self.step + 1,
            heuristic: 0,
        };
        out.compute_hueristic();
        out.compute_summary();
        out
    }
}

impl Display for Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, fl) in self.floors.iter().enumerate().rev() {
            if i == self.elevator {
                write!(f, "E -> ")?;
            } else {
                write!(f, "     ")?;
            }
            writeln!(f, "{fl}")?;
        }
        writeln!(f, "Step: {}", self.step)?;
        writeln!(f, "Distance: {}", self.distance())?;
        writeln!(f, "Heuristic: {}", self.heuristic)?;
        Ok(())
    }
}

impl Ord for Factory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heuristic
            .cmp(&self.heuristic)
            .then_with(|| self.step.cmp(&other.step))
    }
}

impl PartialOrd for Factory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
struct ElevatorMove {
    moved: Floor,
    move_to: usize,
}

impl From<(usize, Floor)> for ElevatorMove {
    fn from(value: (usize, Floor)) -> Self {
        Self {
            moved: value.1,
            move_to: value.0,
        }
    }
}

/// Floor
///
/// Data type that is really just a u16 under the hood.
/// This is used to represent each floor of the factory.
/// The u16 is split into 8 pairs of bits, with each pair representing
/// a different element from the input data.
/// The bits of the pairs represnt the Generator and Microchip respectively
/// A 1 indicates that the element generator or chip is present on the floor,
/// and a 0 not present
///
/// For example, taking floor 1 of the question example:
///    .  HM .  LM ...(nothing else)
///    Hydr  Lith
///    G  M  G  M
///    0  1  0  1  ...(pad with zeroes to end of u16)
///
/// So this floor is represented as `0b0101_0000_0000_0000`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Floor(u16);

impl From<u16> for Floor {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Floor {
    fn valid(self) -> bool {
        let mut unprotected_chips = self.0 & !(self.0 >> 1);
        for _ in 0..8 {
            if unprotected_chips & 1 == 1 {
                let mut generators = self.0 >> 1;
                for _ in 0..8 {
                    if generators & 1 == 1 {
                        return false;
                    }
                    generators >>= 2;
                }
                return true;
            }
            unprotected_chips >>= 2;
        }
        true
    }

    fn move_to(self, moved: Self) -> Self {
        (self.0 | moved.0).into()
    }

    fn move_from(self, moved: Self) -> Self {
        (self.0 & !moved.0).into()
    }

    fn valid_move_to(self, moved: Self) -> bool {
        self.move_to(moved).valid()
    }

    fn valid_move_from(self, moved: Self) -> bool {
        self.move_from(moved).valid()
    }

    /// Finds all possible moves from this floor that pick either one
    /// or a pair of items out of all the items that are currently on
    /// the floor.
    ///
    /// Is a little more clever than just the combinations as you can only
    /// take a chip and generator pair if they are from the matching element.
    /// Otherwise double moves can only contain two chips or two generators.
    /// You can see this as on the currenr floor the chip must be shielded from
    /// generator by its matching generator (it is the only way for the current
    /// floor to be valid). However after the move the chip cannot be shielded
    /// as its generator hasn't been moved but at least one other generator
    /// will be present (the one you're moving)
    fn all_moves_from(self) -> Vec<Self> {
        let mut all = Vec::new();

        // Loop through all 8 generator / chip pairs
        for i in (0..8).rev() {
            self.add_from_base(i, true, &mut all);
            self.add_from_base(i, false, &mut all);
        }
        all
    }

    /// Helper function for the `all_moves_from` function
    ///
    /// Works starting from a base element offset and then checks for all successive
    /// items to see if they can form valid pairs to be moved.
    ///
    /// Since a single move is always an option the base item alone is added as a move as
    /// well.
    ///
    /// If the base item is not present then by definition there are no combinations that can be
    /// formed from it, so quit out at the start.
    fn add_from_base(self, pair: i8, generator: bool, all: &mut Vec<Self>) {
        // Get the base element as a bit mask
        let base_offset = if generator { 2 * pair + 1 } else { 2 * pair };
        let base_mask = 0b1 << base_offset;

        // Quit out if the base element is not present
        if self.0 & base_mask == 0 {
            return;
        }

        // Just add this element
        all.push(base_mask.into());

        // If a generator, look next door for the matching chip
        if generator {
            let chip_mask = 0b1 << (base_offset - 1);
            if self.0 & chip_mask != 0 {
                let num = base_mask | chip_mask;
                all.push(num.into());
            }
        }

        // Add any sucessive elements of the same type
        let mut offset = base_offset - 2;
        while offset > 0 {
            let mask = 0b1 << offset;
            if self.0 & mask != 0 {
                let num = base_mask | mask;
                all.push(num.into());
            }
            offset -= 2;
        }
    }

    fn all_valid_moves_from(self) -> Vec<Self> {
        self.all_moves_from()
            .into_iter()
            .filter(|&f| self.valid_move_from(f))
            .collect()
    }

    fn summarise(self) -> [u8; 4] {
        let mut pairs = 0;
        let mut generators = 0;
        let mut microchips = 0;
        let mut blanks = 0;
        let mut temp = self.0;
        for _ in 0..8 {
            match temp & 0b11 {
                0b11 => pairs += 1,
                0b10 => generators += 1,
                0b01 => microchips += 1,
                0b00 => blanks += 1,
                _ => unreachable!(),
            }
            temp >>= 2;
        }
        [pairs, generators, microchips, blanks]
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.0;
        for _ in 0..8 {
            if (temp & 0x8000) == 0 {
                write!(f, "  ")?;
            } else {
                write!(f, "G ")?;
            }
            temp <<= 1;
            if (temp & 0x8000) == 0 {
                write!(f, "  ")?;
            } else {
                write!(f, "M ")?;
            }
            temp <<= 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(11, part_one(data));
    }
}
