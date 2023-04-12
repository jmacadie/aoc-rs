#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {:0>17b}", part_one(data, 272));
    println!("Part 2: {:0>17b}", part_two(data, 35_651_584));
}

fn part_one(data: &str, data_size: u32) -> u32 {
    generate_checksum(data, data_size)
}

fn part_two(data: &str, data_size: u32) -> u32 {
    generate_checksum(data, data_size)
}

fn generate_checksum(data: &str, data_size: u32) -> u32 {
    let (count, window_size) = get_checksum_params(data_size);
    let mut cs = Checksum::new(data, window_size);
    let mut out = 0;
    for _ in 0..count {
        out <<= 1;
        if cs.next() & 1 == 0 {
            out |= 1;
        };
    }
    out
}

const fn get_checksum_params(mut target: u32) -> (u32, u32) {
    let mut window_size = 1;
    while (target & 1) == 0 {
        window_size <<= 1;
        target /= 2;
    }
    (target, window_size)
}

#[derive(Debug)]
struct Checksum {
    output_window: u32,
    roots: Roots,
    seperators: Seperators,
}

impl Checksum {
    fn new(init: &str, output_window: u32) -> Self {
        Self {
            output_window,
            roots: Roots::new(init),
            seperators: Seperators::new(),
        }
    }

    fn next(&mut self) -> u32 {
        let (root_count, seperators) = self.roots.next(self.output_window);
        let sep_count = self.seperators.next(seperators);
        root_count + sep_count
    }
}

#[derive(Debug)]
struct Roots {
    data: u64,
    len: u32,
    position: u32,
    ones: u32,
}

impl Roots {
    fn new(init: &str) -> Self {
        let init = init.trim();
        let mut a = u64::from_str_radix(init, 2).unwrap();
        let len = u32::try_from(init.len()).unwrap();
        let b = !a << (64 - len) >> (64 - 2 * len - 1);
        a = Self::reverse(a);
        a >>= 64 - len;
        let data = a | b;
        Self {
            data,
            len: 2 * (len + 1),
            position: 0,
            ones: data.count_ones(),
        }
    }

    const fn reverse(mut n: u64) -> u64 {
        let mut rev = 0;
        let mut bits = 63;
        while n != 0 {
            rev |= (n & 1) << bits;
            n >>= 1;
            bits -= 1;
        }
        rev
    }

    fn next(&mut self, window_size: u32) -> (u32, u32) {
        let cycles = window_size / self.len;
        let full_cycle_count = self.ones * cycles;

        let start = self.position;
        self.position += window_size;
        self.position %= self.len;

        let seperators = 2 * cycles
            + match (
                start <= self.position,
                self.same_partition(start, self.position),
            ) {
                (true, true) => 0,
                (_, false) => 1,
                (false, true) => 2,
            };

        let mut temp = self.data;
        let min = std::cmp::min(start, self.position);
        let max = std::cmp::max(start, self.position);
        temp >>= min;
        temp <<= 64 - max + min;
        let mut part_cycle_count = temp.count_ones();

        if start > self.position {
            part_cycle_count = self.ones - part_cycle_count;
        }
        (part_cycle_count + full_cycle_count, seperators)
    }

    const fn same_partition(&self, a: u32, b: u32) -> bool {
        let partition = self.len / 2;
        (a < partition && b < partition) || (a >= partition && b >= partition)
    }
}

#[derive(Debug)]
struct Seperators {
    position: u32,
    last_value: u32,
}

impl Seperators {
    const fn new() -> Self {
        Self {
            position: 0,
            last_value: 0,
        }
    }

    fn next(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        self.position += n;
        let last = self.last_value;
        self.last_value = Self::dragon_curve(self.position);
        self.last_value - last
    }

    fn dragon_curve(n: u32) -> u32 {
        if n == 1 {
            return 0;
        }
        if Self::is_2n(n) {
            return (n - 2) / 2;
        }
        if Self::is_2n(n + 1) {
            return (n - 1) / 2;
        }
        let lim = Self::next_2n(n) - 1;
        let lim_val = (lim - 1) / 2;
        let excess = lim - n;
        let excess_val = excess - Self::dragon_curve(excess);
        lim_val - excess_val
    }

    const fn is_2n(n: u32) -> bool {
        n.count_ones() == 1
    }

    const fn next_2n(mut n: u32) -> u32 {
        let mut out = 1;
        while n != 0 {
            out <<= 1;
            n >>= 1;
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0b01100, part_one(data, 20));
    }
}
