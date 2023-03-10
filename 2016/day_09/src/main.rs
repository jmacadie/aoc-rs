#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    decompress(data.as_bytes(), false)
}

fn part_two(data: &str) -> usize {
    decompress(data.as_bytes(), true)
}

fn decompress(mut data: &[u8], recursive: bool) -> usize {
    let mut decompressed_length = 0;

    while !data.is_empty() {
        let Some((first, rest)) = data.split_first() else {
            unreachable!();
        };
        match first {
            b'(' => {
                // Found a marker, parse it out
                let (marker, rest) = split_once_slice(rest, &b')').unwrap();
                let (rep_len, rep_count) = split_once_slice(marker, &b'x').unwrap();
                let rep_len: usize = std::str::from_utf8(rep_len).unwrap().parse().unwrap();
                let rep_count: usize = std::str::from_utf8(rep_count).unwrap().parse().unwrap();

                // Add the decompressed total (length * repeat count)
                let len = if recursive {
                    decompress(&rest[..rep_len], true)
                } else {
                    rep_len
                };
                decompressed_length += len * rep_count;

                // move data slice on so we ignore the repeat pattern
                // it has been accoutned for above
                data = &rest[rep_len..];
            }
            b' ' | b'\t' | b'\n' | b'\r' => data = rest, // whitespace: ignore
            _ => {
                // Anything else, increment the decompressed count and move on
                decompressed_length += 1;
                data = rest;
            }
        }
    }
    decompressed_length
}

fn split_once_slice<'a, T: Eq + PartialEq>(
    slice: &'a [T],
    separator: &'_ T,
) -> Option<(&'a [T], &'a [T])> {
    slice
        .iter()
        .position(|x| x == separator)
        .map(|pos| (&slice[..pos], &slice[pos + 1..]))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(6, part_one("ADVENT"));
        assert_eq!(7, part_one("A(1x5)BC"));
        assert_eq!(9, part_one("(3x3)XYZ"));
        assert_eq!(11, part_one("A(2x2)BCD(2x2)EFG"));
        assert_eq!(6, part_one("(6x1)(1x3)A"));
        assert_eq!(18, part_one("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn two() {
        assert_eq!(9, part_two("(3x3)XYZ"));
        assert_eq!(20, part_two("X(8x2)(3x3)ABCY"));
        assert_eq!(241_920, part_two("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(
            445,
            part_two("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }
}
