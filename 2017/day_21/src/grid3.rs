use std::{fmt::Display, str::FromStr};

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
pub struct Grid3 {
    pub(crate) data: u16,
}

impl From<u16> for Grid3 {
    fn from(value: u16) -> Self {
        Self { data: value }
    }
}

impl Grid3 {
    pub fn to_index(self) -> usize {
        usize::from(self.data)
    }

    const fn ring(self) -> u16 {
        self.data & 0x00ff
    }

    const fn centre(self) -> u16 {
        self.data & 0x0100
    }

    pub fn equivalent_set(self) -> [Self; 8] {
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

    pub const fn num_on(self) -> u32 {
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
