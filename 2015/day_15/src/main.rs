#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

fn part_one() -> i64 {
    let mut max = 0;
    for b in 2..57 {
        let d_lim = std::cmp::min(99 - b * 8 / 5, 5 * b);
        for d in 6..d_lim {
            let c_lim_min = (200 - 2 * d + b) / 7;
            let c_lim_max = std::cmp::min(5 * d, 99 - b - d);
            for c in c_lim_min..c_lim_max {
                let a: i32 = 100 - b - c - d;
                let a = u8::try_from(a).unwrap();
                let b = u8::try_from(b).unwrap();
                let c = u8::try_from(c).unwrap();
                let d = u8::try_from(d).unwrap();

                max = std::cmp::max(max, magic_number(a, b, c, d));
            }
        }
    }
    max
}

fn part_two() -> i64 {
    let mut max = 0;
    for b in 2..57 {
        let d_lim = std::cmp::min(99 - b * 8 / 5, 5 * b);
        for d in 6..d_lim {
            let c_lim_min = (200 - 2 * d + b) / 7;
            let c_lim_max = std::cmp::min(5 * d, 99 - b - d);
            for c in c_lim_min..c_lim_max {
                let a: i32 = 100 - b - c - d;

                if 3 * a + 3 * b + 8 * c + 8 * d == 500 {
                    let a = u8::try_from(a).unwrap();
                    let b = u8::try_from(b).unwrap();
                    let c = u8::try_from(c).unwrap();
                    let d = u8::try_from(d).unwrap();

                    max = std::cmp::max(max, magic_number(a, b, c, d));
                }
            }
        }
    }
    max
}

fn magic_number(a: u8, b: u8, c: u8, d: u8) -> i64 {
    let a = i64::from(a);
    let b = i64::from(b);
    let c = i64::from(c);
    let d = i64::from(d);

    let capacity = 2 * a;
    let durability = std::cmp::max(5 * b - d, 0);
    let flavour = std::cmp::max(5 * c - 3 * b - 2 * a, 0);
    let texture = std::cmp::max(5 * d - c, 0);

    capacity * durability * flavour * texture
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(0, part_one());
    }

    #[test]
    fn two() {
        assert_eq!(0, part_two());
    }
}
