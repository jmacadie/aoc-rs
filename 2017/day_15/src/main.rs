#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn main() {
    let data = include_str!("input.txt");
    let (a, b) = get_seeds(data);
    let (a_vals, b_vals) = gen_vals(a, b);
    println!("Part 1: {}", part_one(&a_vals, &b_vals));
    println!("Part 2: {}", part_two(&a_vals, &b_vals));
}

const A_FACTOR: u64 = 16_807;
const B_FACTOR: u64 = 48_271;
const MOD: u64 = 2_147_483_647;
const LIM: usize = 40_000_000;

fn get_seeds(data: &str) -> (u64, u64) {
    let mut lines = data.lines();
    let a_seed = lines
        .next()
        .unwrap()
        .split_once(" with ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let b_seed = lines
        .next()
        .unwrap()
        .split_once(" with ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    (a_seed, b_seed)
}

fn gen_vals(a_seed: u64, b_seed: u64) -> (Vec<u64>, Vec<u64>) {
    let a_vals = Arc::new(Mutex::new(Vec::with_capacity(LIM)));
    let b_vals = Arc::new(Mutex::new(Vec::with_capacity(LIM)));

    let cloned = Arc::clone(&a_vals);
    let thread_a = thread::spawn(move || {
        let mut vals = cloned.lock().unwrap();
        let mut a = a_seed;
        for _ in 0..LIM {
            a = (a * A_FACTOR) % MOD;
            vals.push(a);
        }
    });

    let cloned = Arc::clone(&b_vals);
    let thread_b = thread::spawn(move || {
        let mut vals = cloned.lock().unwrap();
        let mut b = b_seed;
        for _ in 0..LIM {
            b = (b * B_FACTOR) % MOD;
            vals.push(b);
        }
    });

    let _ = thread_a.join();
    let _ = thread_b.join();

    let a_vals = Arc::into_inner(a_vals).unwrap().into_inner().unwrap();
    let b_vals = Arc::into_inner(b_vals).unwrap().into_inner().unwrap();

    (a_vals, b_vals)
}

fn part_one(a_vals: &[u64], b_vals: &[u64]) -> u16 {
    a_vals.iter().zip(b_vals.iter()).fold(0, |acc, (a, b)| {
        if lower_16_bits_same(*a, *b) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_two(a_vals: &[u64], b_vals: &[u64]) -> u16 {
    a_vals
        .iter()
        .filter(|&&a| divisible_by_4(a))
        .zip(b_vals.iter().filter(|&&b| divisible_by_8(b)))
        .take(5_000_000)
        .fold(0, |acc, (a, b)| {
            if lower_16_bits_same(*a, *b) {
                acc + 1
            } else {
                acc
            }
        })
}

const fn lower_16_bits_same(mut a: u64, mut b: u64) -> bool {
    a <<= 64 - 16;
    b <<= 64 - 16;
    a ^ b == 0
}

const fn divisible_by_4(a: u64) -> bool {
    a << (64 - 2) == 0
}

const fn divisible_by_8(a: u64) -> bool {
    a << (64 - 3) == 0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (a, b) = get_seeds(data);
        let (a_vals, b_vals) = gen_vals(a, b);
        assert_eq!(588, part_one(&a_vals, &b_vals));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (a, b) = get_seeds(data);
        let (a_vals, b_vals) = gen_vals(a, b);
        assert_eq!(309, part_two(&a_vals, &b_vals));
    }
}
