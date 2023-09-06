use crate::grid2::Grid2;
use std::{fmt::Display, str::FromStr};

/// 4x4 Grid (AKA 4 x 2x2 Grids)
///
/// This is it's own struct so we can parse in the rules that map a 3x3 into a 4x4.
/// However, under the hood I'm simply treating this as an array of 4 `Grid2`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Grid2x2 {
    pub(crate) data: [Grid2; 4],
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
