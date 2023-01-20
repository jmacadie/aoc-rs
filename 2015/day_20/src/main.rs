#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    println!("Part 1: {}", part_one(29_000_000 / 10));
    println!("Part 2: {}", part_two(29_000_000 / 11));
}

fn part_one(lim: u32) -> u32 {
    const TRIANGLE_BASES: [u32; 4] = [1, 2, 2_u32.pow(2) * 3, 2_u32.pow(3) * 3_u32.pow(2) * 5_u32];
    let base = TRIANGLE_BASES
        .iter()
        .rev()
        .map(|b| (b, b.checked_pow(2)))
        .find(|(_, sq)| sq.is_some() && sq.unwrap() < lim)
        .map(|(b, _)| b)
        .unwrap();
    let min = lim / 5; // should really be a scaled with lim but numbers have to be way bigger to go
                       // over five. Five being the ratio of a number to the sum of it's factors

    for i in min / base.. {
        let root = base * i;
        let sum = sum_of_factors(root);
        if sum > lim {
            return root;
        }
    }
    0
}

fn part_two(lim: u32) -> u32 {
    const TRIANGLE_BASES: [u32; 4] = [1, 2, 2_u32.pow(2) * 3, 2_u32.pow(3) * 3_u32.pow(2) * 5_u32];
    let base = TRIANGLE_BASES
        .iter()
        .rev()
        .map(|b| (b, b.checked_pow(2)))
        .find(|(_, sq)| sq.is_some() && sq.unwrap() < lim)
        .map(|(b, _)| b)
        .unwrap();
    let min = lim / 5; // should really be a scaled with lim but numbers have to be way bigger to go
                       // over five. Five being the ratio of a number to the sum of it's factors

    for i in min / base.. {
        let root = base * i;
        let sum = sum_of_factors_less_then_50(root);
        if sum > lim {
            return root;
        }
    }
    0
}

const PRIMES: [u32; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
const NUMS_TO_50: [[u32; 15]; 50] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 1
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 2
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 3
    [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 4
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 5
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 6
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 7
    [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 8
    [0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 9
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 10
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 11
    [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 12
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 13
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 14
    [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 15
    [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 16
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0], // 17
    [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 18
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], // 19
    [2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 20
    [0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 21
    [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 22
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], // 23
    [3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 24
    [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 25
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 26
    [0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 27
    [2, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 28
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], // 29
    [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 30
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 31
    [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 32
    [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 33
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0], // 34
    [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 35
    [2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 36
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], // 37
    [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], // 38
    [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 39
    [3, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 40
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], // 41
    [1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 42
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0], // 43
    [2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 44
    [0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 45
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], // 46
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], // 47
    [4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 48
    [0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 49
    [1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 50
];

fn prime_factors(root: u32) -> Option<[u32; 15]> {
    let mut test = root;
    let mut factors = [0; 15];
    while test > 1 {
        let factor =
            PRIMES
                .iter()
                .enumerate()
                .find_map(|(i, p)| if test % p == 0 { Some(i) } else { None })?;
        factors[factor] += 1;
        test /= PRIMES[factor];
    }
    Some(factors)
}

fn sum_of_factors(root: u32) -> u32 {
    let factors = prime_factors(root);
    if let Some(factors) = factors {
        PRIMES
            .iter()
            .zip(factors.iter())
            .map(|(p, f)| (p.pow(f + 1) - 1) / (p - 1))
            .product()
    } else {
        0
    }
}

fn sum_of_factors_less_then_50(root: u32) -> u32 {
    if let Some(factors) = prime_factors(root) {
        NUMS_TO_50
            .iter()
            .filter(|&num| factor_of(&factors, num))
            .map(|num| {
                num.iter()
                    .zip(PRIMES.iter())
                    .fold(root, |res, (n, p)| res / p.pow(*n))
            })
            .sum()
    } else {
        0
    }
}

fn factor_of(num: &[u32; 15], factor: &[u32; 15]) -> bool {
    !num.iter().zip(factor.iter()).any(|(n, f)| n < f)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn factors() {
        assert_eq!([1, 1, 0, 0, 0], prime_factors(6).unwrap()[..5]);
        assert_eq!([2, 1, 1, 0, 0], prime_factors(60).unwrap()[..5]);
        assert_eq!([3, 2, 1, 0, 0], prime_factors(360).unwrap()[..5]);
        assert_eq!([1, 1, 0, 1, 0], prime_factors(42).unwrap()[..5]);
        assert_eq!([5, 3, 3, 1, 0], prime_factors(756_000).unwrap()[..5]);
        assert_eq!([6, 3, 1, 1, 1], prime_factors(665_280).unwrap()[..5]);
    }

    #[test]
    fn sum_factors() {
        assert_eq!(12, sum_of_factors(6));
        assert_eq!(60, sum_of_factors(24));
        assert_eq!(96, sum_of_factors(42));
        assert_eq!(252, sum_of_factors(96));
        assert_eq!(3_144_960, sum_of_factors(756_000));
    }

    #[test]
    fn sum_factors_50() {
        assert_eq!(12, sum_of_factors_less_then_50(6));
        assert_eq!(60, sum_of_factors_less_then_50(24));
        assert_eq!(251, sum_of_factors_less_then_50(96));
        assert_eq!(2_583_070, sum_of_factors_less_then_50(665_280));
    }

    #[test]
    fn one() {
        assert_eq!(42, part_one(95));
    }

    #[test]
    fn two() {
        assert_eq!(72, part_two(167));
    }
}
