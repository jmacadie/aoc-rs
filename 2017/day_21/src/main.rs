#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

const START: u16 = 0b0000_0000_0101_1110;

pub fn main() {
    let data = include_str!("input.txt");
    let grid_map = GridMap::new(data);
    println!("Part 1: {}", part_one(&grid_map));
    println!("Part 2: {}", part_two(&grid_map));
}

fn part_one(grid_map: &GridMap) -> u32 {
    let start: Grid3 = START.into();

    grid_map
        .get_3x3(start)
        .iter()
        .flat_map(|g| grid_map.get_2x3(*g))
        .map(Grid3::num_on)
        .sum()
}

fn part_two(grid_map: &GridMap) -> u32 {
    fn expand(data: &[(Grid3, u32)], grid_map: &GridMap) -> Vec<(Grid3, u32)> {
        let mut output = Vec::with_capacity(data.len() * 9);
        for (grid, found) in &data
            .iter()
            .flat_map(|(g, c)| grid_map.get_3x3(*g).into_iter().map(move |gi| (gi, c)))
            .sorted_unstable_by_key(|&(g, _)| g)
            .group_by(|&(g, _)| g)
        {
            let count = found.map(|(_, c)| c).sum();
            output.push((grid, count));
        }
        output
    }

    let mut grids = vec![(START.into(), 1)];
    for _ in 0..6 {
        grids = expand(&grids, grid_map);
    }

    grids.into_iter().map(|(g, c)| c * g.num_on()).sum()
}

#[derive(Debug)]
struct GridMap {
    g2_g3: [Grid3; 16],
    g3_g2x2: [Grid2x2; 512],    // 1 iteration
    g3_g2x3: [[Grid3; 4]; 512], // 2 iterations
    g3_g3x2: [[Grid2; 9]; 512], // n/a
    g3_g3x3: [[Grid3; 9]; 512], // 3 iterations
}

impl GridMap {
    fn new(data: &str) -> Self {
        let mut g2_g3 = [Grid3::default(); 16];
        let mut g3_g2x2 = [Grid2x2::default(); 512];

        // Read in the explicit written data first
        for (from, to) in data.lines().map(|l| l.split_once(" => ").unwrap()) {
            if from.len() == 5 {
                // 2x2 to 3x3 map
                let from: Grid2 = from.parse().unwrap();
                let to: Grid3 = to.parse().unwrap();

                for g in from.equivalent_set() {
                    g2_g3[g.to_index()] = to;
                }
            } else {
                // 3x3 to 4x4 map
                let from: Grid3 = from.parse().unwrap();
                let to: Grid2x2 = to.parse().unwrap();

                for g in from.equivalent_set() {
                    g3_g2x2[g.to_index()] = to;
                }
            }
        }

        // Now add the implied mappings for the next two steps
        // Written as three steps as g3x2 -> g2x3 is actually just carving up the same bitset into
        // more copies of smaller grids
        let g3_g2x3 = [[Grid3::default(); 4]; 512];
        let g3_g3x2 = [[Grid2::default(); 9]; 512];
        let g3_g3x3 = [[Grid3::default(); 9]; 512];

        let mut output = Self {
            g2_g3,
            g3_g2x2,
            g3_g2x3,
            g3_g3x2,
            g3_g3x3,
        };

        output.gen_2x3();
        output.gen_3x2();
        output.gen_3x3();

        output
    }

    fn get_2x3(&self, source: Grid3) -> [Grid3; 4] {
        self.g3_g2x3[source.to_index()]
    }

    fn get_3x3(&self, source: Grid3) -> [Grid3; 9] {
        self.g3_g3x3[source.to_index()]
    }

    fn gen_2x3(&mut self) {
        for (next, prev) in self.g3_g2x3.iter_mut().zip(self.g3_g2x2.iter()) {
            for (inner_next, inner_prev) in next.iter_mut().zip(prev.data.iter()) {
                *inner_next = self.g2_g3[inner_prev.to_index()];
            }
        }
    }

    // Same bit set but sliced up into 9 grids of 2x2 rather than 4 grids of 3x3
    // There's no other way round (that I can think of) than to individually allocate the source
    // bits to the target bit locations.
    // Sometimes we get a quick win where we can allocate runs of bits
    fn gen_3x2(&mut self) {
        for (next, prev) in self.g3_g3x2.iter_mut().zip(self.g3_g2x3.iter()) {
            let mut ring: u16;

            // Top left
            ring = prev[0].data >> 4 & 0x000c; // 8, 7 -> 4, 3
            ring |= prev[0].data >> 7 & 0x0002; // 9 -> 2
            ring |= prev[0].data & 0x0001; // 1 -> 1
            next[0] = ring.into();

            // Top middle
            ring = prev[0].data >> 2 & 0x0008; // 6 -> 4
            ring |= prev[1].data >> 5 & 0x0004; // 8 -> 3
            ring |= prev[1].data << 1 & 0x0002; // 1 -> 2
            ring |= prev[0].data >> 4 & 0x0001; // 5 -> 1
            next[1] = ring.into();

            // Top right
            ring = prev[1].data >> 3 & 0x000e; // 7, 6, 5 -> 4, 3, 2
            ring |= prev[1].data >> 8 & 0x0001; // 9 -> 1
            next[2] = ring.into();

            // Middle left
            ring = prev[0].data << 2 & 0x0008; // 2 -> 4
            ring |= prev[0].data & 0x0004; // 3 -> 3
            ring |= prev[2].data >> 5 & 0x0002; // 7 -> 2
            ring |= prev[2].data >> 7 & 0x0001; // 8 -> 1
            next[3] = ring.into();

            // Middle middle
            ring = prev[0].data & 0x0008; // 4 -> 4
            ring |= prev[1].data << 1 & 0x0004; // 2 -> 3
            ring |= prev[3].data >> 6 & 0x0002; // 8 -> 2
            ring |= prev[2].data >> 5 & 0x0001; // 6 -> 1
            next[4] = ring.into();

            // Middle right
            ring = prev[1].data << 1 & 0x0008; // 3 -> 4
            ring |= prev[1].data >> 1 & 0x0004; // 4 -> 3
            ring |= prev[3].data >> 4 & 0x0002; // 6 -> 2
            ring |= prev[3].data >> 6 & 0x0001; // 7 -> 1
            next[5] = ring.into();

            // Bottom left
            ring = prev[2].data << 3 & 0x0008; // 1 -> 4
            ring |= prev[2].data >> 6 & 0x0004; // 9 -> 3
            ring |= prev[2].data >> 1 & 0x0003; // 3, 2 -> 2, 1
            next[6] = ring.into();

            // Bottom middle
            ring = prev[2].data >> 1 & 0x0008; // 5 -> 4
            ring |= prev[3].data << 2 & 0x0004; // 1 -> 3
            ring |= prev[3].data & 0x0002; // 2 -> 2
            ring |= prev[2].data >> 3 & 0x0001; // 4 -> 1
            next[7] = ring.into();

            // Bottom right
            ring = prev[3].data >> 5 & 0x0008; // 9 -> 4
            ring |= prev[3].data >> 2 & 0x0007; // 5, 4, 3 -> 3, 2, 1
            next[8] = ring.into();
        }
    }

    fn gen_3x3(&mut self) {
        for (next, prev) in self.g3_g3x3.iter_mut().zip(self.g3_g3x2.iter()) {
            for (inner_next, inner_prev) in next.iter_mut().zip(prev.iter()) {
                *inner_next = self.g2_g3[inner_prev.to_index()];
            }
        }
    }
}

/// 2x2 Grid
///
/// A simple wrapper over a u8. We actually only needed a u4, but one doesn't exist so have used a
/// u8 and ignore the top 4 bits
///
/// The 2x2 grid can be on or off in each of its four locations. We use a 1 to indicate that the
/// location is 'on' and 0 to indicate 'off'.
///
/// The mapping of physical location to u8 bits is as follows:
/// 1 2
/// 3 4  ->  0 0 0 0 1 2 4 3
///
/// This order is used to make it easier to rotate the bits
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Grid2 {
    data: u8,
}

impl Grid2 {
    fn equivalent_set(self) -> [Self; 4] {
        let mut ring = self.data;
        let mut output = [Self::default(); 4];
        for grid in &mut output {
            *grid = ring.into();
            ring = Self::rotate(ring);
        }
        output
    }

    const fn rotate(ring: u8) -> u8 {
        let lsb = ring & 0x0001;
        ring >> 1 | lsb << 3
    }

    fn to_index(self) -> usize {
        usize::from(self.data)
    }
}

impl From<u8> for Grid2 {
    fn from(value: u8) -> Self {
        Self { data: value }
    }
}

impl From<u16> for Grid2 {
    fn from(value: u16) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        (value as u8).into()
    }
}

impl FromStr for Grid2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_char(char: u8) -> Result<u8, String> {
            match char {
                b'#' => Ok(1),
                b'.' => Ok(0),
                _ => Err(format!(
                    "The charcter {} is expected to be one of '#' or '.' only",
                    String::from_utf8(vec![char]).map_err(|_| "Invalid character: {char}")?
                )),
            }
        }

        // Get the parts, split by '/'
        let mut parts = s.split('/');
        let (Some(top), Some(bot), None) = (parts.next(), parts.next(), parts.next()) else {
            return Err(format!("{s} is not split into two parts, divided by '/'"));
        };
        if top.len() != 2 {
            return Err(format!(
                "The top row should only be two characters long: {top}"
            ));
        }
        if bot.len() != 2 {
            return Err(format!(
                "The bottom row should only be two characters long: {bot}"
            ));
        }

        // Switch to byte arrays
        let top = top.as_bytes();
        let bot = bot.as_bytes();

        // Declare the output
        let mut output = 0;

        // Add the top row
        for i in top.iter() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        // Add the bottom row
        for i in bot.iter().rev() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        Ok(output.into())
    }
}

impl Display for Grid2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_bit(f: &mut std::fmt::Formatter<'_>, data: u8) -> std::fmt::Result {
            match data & 1 {
                1 => write!(f, "#")?,
                0 => write!(f, ".")?,
                _ => unreachable!(),
            }
            Ok(())
        }
        write_bit(f, self.data >> 3)?;
        write_bit(f, self.data >> 2)?;
        writeln!(f)?;
        write_bit(f, self.data)?;
        write_bit(f, self.data >> 1)?;
        writeln!(f)?;
        Ok(())
    }
}

/// 4x4 Grid (AKA 4 x 2x2 Grids)
///
/// This is it's own struct so we can parse in the rules that map a 3x3 into a 4x4.
/// However, under the hood I'm simply treating this as an array of 4 `Grid2`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Grid2x2 {
    data: [Grid2; 4],
}

impl FromStr for Grid2x2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_char(char: u8) -> Result<u8, String> {
            match char {
                b'#' => Ok(1),
                b'.' => Ok(0),
                _ => Err(format!(
                    "The charcter {} is expected to be one of '#' or '.' only",
                    String::from_utf8(vec![char]).map_err(|_| "Invalid character: {char}")?
                )),
            }
        }

        // Get the parts, split by '/'
        let mut parts = s.split('/');
        let (Some(top), Some(mid1), Some(mid2), Some(bot), None) = (parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) else {
            return Err(format!("{s} is not split into four parts, divided by '/'"));
        };
        if top.len() != 4 {
            return Err(format!(
                "The top row should only be four characters long: {top}"
            ));
        }
        if mid1.len() != 4 {
            return Err(format!(
                "The second row should only be four characters long: {mid1}"
            ));
        }
        if mid2.len() != 4 {
            return Err(format!(
                "The third row should only be four characters long: {mid2}"
            ));
        }
        if bot.len() != 4 {
            return Err(format!(
                "The bottom row should only be four characters long: {bot}"
            ));
        }

        // Switch to byte arrays
        let top = top.as_bytes();
        let mid1 = mid1.as_bytes();
        let mid2 = mid2.as_bytes();
        let bot = bot.as_bytes();

        // TOP LEFT 2x2 grid

        // Declare the output
        let mut output = 0;

        // Add the top row
        for i in top.iter().take(2) {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        // Add the bottom row
        for i in mid1.iter().take(2).rev() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        let top_left = output.into();

        // TOP RIGHT 2x2 grid

        // Clear the output
        output = 0;

        // Add the top row
        for i in top.iter().skip(2).take(2) {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        // Add the bottom row
        for i in mid1.iter().skip(2).take(2).rev() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        let top_right = output.into();

        // BOTTOM LEFT 2x2 grid

        // Clear the output
        output = 0;

        // Add the top row
        for i in mid2.iter().take(2) {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        // Add the bottom row
        for i in bot.iter().take(2).rev() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        let bottom_left = output.into();

        // BOTTOM RIGHT 2x2 grid

        // Clear the output
        output = 0;

        // Add the top row
        for i in mid2.iter().skip(2).take(2) {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        // Add the bottom row
        for i in bot.iter().skip(2).take(2).rev() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        let bottom_right = output.into();

        Ok(Self {
            data: [top_left, top_right, bottom_left, bottom_right],
        })
    }
}

impl Display for Grid2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_bit(f: &mut std::fmt::Formatter<'_>, data: u8) -> std::fmt::Result {
            match data & 1 {
                1 => write!(f, "#")?,
                0 => write!(f, ".")?,
                _ => unreachable!(),
            }
            Ok(())
        }
        write_bit(f, self.data[0].data >> 3)?;
        write_bit(f, self.data[0].data >> 2)?;
        write!(f, " | ")?;
        write_bit(f, self.data[1].data >> 3)?;
        write_bit(f, self.data[1].data >> 2)?;
        writeln!(f)?;
        write_bit(f, self.data[0].data)?;
        write_bit(f, self.data[0].data >> 1)?;
        write!(f, " | ")?;
        write_bit(f, self.data[1].data)?;
        write_bit(f, self.data[1].data >> 1)?;
        writeln!(f)?;
        writeln!(f, "-- + --")?;
        write_bit(f, self.data[2].data >> 3)?;
        write_bit(f, self.data[2].data >> 2)?;
        write!(f, " | ")?;
        write_bit(f, self.data[3].data >> 3)?;
        write_bit(f, self.data[3].data >> 2)?;
        writeln!(f)?;
        write_bit(f, self.data[2].data)?;
        write_bit(f, self.data[2].data >> 1)?;
        write!(f, " | ")?;
        write_bit(f, self.data[3].data)?;
        write_bit(f, self.data[3].data >> 1)?;
        writeln!(f)?;
        Ok(())
    }
}

/// 3x3 Grid
///
/// A simple wrapper over a u16. We actually only needed a u9, but one doesn't exist so have used a
/// u16 and ignore the top 7 bits
///
/// The 3x3 grid can be on or off in each of its nine locations. We use a 1 to indicate that the
/// location is 'on' and 0 to indicate 'off'.
///
/// The mapping of physical location to u16 bits is as follows:
/// 1 2 3
/// 4 5 6
/// 7 8 9  ->  0 0 0 0 0 0 0 5 1 2 3 6 9 8 7 4
///
/// This order is used to make it easier to rotate the bits, which is needed for out folding and
/// rotating
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, PartialOrd, Ord)]
struct Grid3 {
    data: u16,
}

impl From<u16> for Grid3 {
    fn from(value: u16) -> Self {
        Self { data: value }
    }
}

impl Grid3 {
    fn to_index(self) -> usize {
        usize::from(self.data)
    }

    const fn ring(self) -> u16 {
        self.data & 0x00ff
    }

    const fn centre(self) -> u16 {
        self.data & 0x0100
    }

    fn equivalent_set(self) -> [Self; 8] {
        let mut ring = self.ring();
        let centre = self.centre();

        let mut output = [Self::default(); 8];
        for grid in output.iter_mut().take(4) {
            *grid = (ring | centre).into();
            ring = Self::rotate(ring);
        }

        ring = Self::reverse(ring);
        for grid in output.iter_mut().skip(4).take(4) {
            *grid = (ring | centre).into();
            ring = Self::rotate(ring);
        }

        output
    }

    /// Takes the lower eight bits of a u16 and rotates them around
    ///
    /// For example:
    /// 0 0 0 0 0 0 0 0 7 6 5 4 3 2 1 0
    ///
    /// goes to:
    /// 0 0 0 0 0 0 0 0 1 0 7 6 5 4 3 2
    ///                 * *
    ///                     > > > > > >
    /// \             /
    ///   upper eight
    ///  bits, ignore
    const fn rotate(ring: u16) -> u16 {
        let lsb = ring & 0x0003;
        ring >> 2 | lsb << 6
    }

    /// Keeps the eighth bit in place but reverses the bits from 0 to 7
    /// This is needed for the folds, which are the combination of
    /// one reverse and a number of rotations
    ///
    /// For example:
    /// 0 0 0 0 0 0 0 0 7 6 5 4 3 2 1 0
    ///
    /// goes to:
    /// 0 0 0 0 0 0 0 0 7 0 1 2 3 4 5 6
    ///                 *
    ///                   < < < | > > >
    /// \             /
    ///   upper eight
    ///  bits, ignore
    fn reverse(ring: u16) -> u16 {
        let mut temp = ring;
        let mut reversed = ring >> 7;
        for _ in 0..7 {
            reversed <<= 1;
            reversed |= temp & 1;
            temp >>= 1;
        }
        reversed
    }

    const fn num_on(self) -> u32 {
        self.data.count_ones()
    }
}

impl FromStr for Grid3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_char(char: u8) -> Result<u16, String> {
            match char {
                b'#' => Ok(1),
                b'.' => Ok(0),
                _ => Err(format!(
                    "The charcter {} is expected to be one of '#' or '.' only",
                    String::from_utf8(vec![char]).map_err(|_| "Invalid character: {char}")?
                )),
            }
        }

        // Get the parts, split by '/'
        let mut parts = s.split('/');
        let (Some(top), Some(mid), Some(bot), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
            return Err(format!("{s} is not split into three parts, divided by '/'"));
        };
        if top.len() != 3 {
            return Err(format!(
                "The top row should only be three characters long: {top}"
            ));
        }
        if mid.len() != 3 {
            return Err(format!(
                "The middle row should only be three characters long: {mid}"
            ));
        }
        if bot.len() != 3 {
            return Err(format!(
                "The bottom row should only be three characters long: {bot}"
            ));
        }

        // Switch to byte arrays
        let top = top.as_bytes();
        let mid = mid.as_bytes();
        let bot = bot.as_bytes();

        // Put the centre on first
        let mut output = parse_char(mid[1])?;

        // Now add the top row
        for i in top.iter() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        // Now add the right middle char
        output <<= 1;
        output |= parse_char(mid[2])?;

        // Now add the bottom row
        for i in bot.iter().rev() {
            output <<= 1;
            output |= parse_char(*i)?;
        }

        // Finally, add the left middle char
        output <<= 1;
        output |= parse_char(mid[0])?;

        Ok(output.into())
    }
}

impl Display for Grid3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_bit(f: &mut std::fmt::Formatter<'_>, data: u16) -> std::fmt::Result {
            match data & 1 {
                1 => write!(f, "#")?,
                0 => write!(f, ".")?,
                _ => unreachable!(),
            }
            Ok(())
        }
        write_bit(f, self.data >> 7)?;
        write_bit(f, self.data >> 6)?;
        write_bit(f, self.data >> 5)?;
        writeln!(f)?;
        write_bit(f, self.data)?;
        write_bit(f, self.data >> 8)?;
        write_bit(f, self.data >> 4)?;
        writeln!(f)?;
        write_bit(f, self.data >> 1)?;
        write_bit(f, self.data >> 2)?;
        write_bit(f, self.data >> 3)?;
        writeln!(f)?;
        Ok(())
    }
}
