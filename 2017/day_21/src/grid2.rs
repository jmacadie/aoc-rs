use std::{fmt::Display, str::FromStr};

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
pub struct Grid2 {
    pub(crate) data: u8,
}

impl Grid2 {
    pub fn equivalent_set(self) -> [Self; 4] {
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

    pub fn to_index(self) -> usize {
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
