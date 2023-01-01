pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> String {
    let initial = data
        .lines()
        .map(|l| l.as_bytes())
        .fold([0; 20], acc_numbers);
    let norm = normalise(initial);
    print_snafu(norm)
}

fn acc_numbers(mut array: [i32; 20], number: &[u8]) -> [i32; 20] {
    for (i, v) in number.iter().rev().enumerate() {
        let delta = match v {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => unreachable!(),
        };
        array[i] += delta;
    }
    array
}

fn normalise(mut number: [i32; 20]) -> [i32; 20] {
    let mut carry = 0;
    for elem in number.iter_mut() {
        let (c, r) = normalise_one(*elem + carry);
        *elem = r;
        carry = c;
    }
    number
}

fn normalise_one(count: i32) -> (i32, i32) {
    let rem = (count + 2).rem_euclid(5) - 2;
    let carry = (count - rem) / 5;
    (carry, rem)
}

fn print_snafu(number: [i32; 20]) -> String {
    let mut started = false;
    let mut out = String::new();
    for &elem in number.iter().rev() {
        if elem != 0 {
            started = true;
        }
        if started {
            let c = match elem {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            };
            out.push(c);
        }
    }
    out
}

fn part_two(_data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!("2=-1=0", part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
