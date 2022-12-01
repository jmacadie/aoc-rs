#![warn(unused_imports, dead_code)]
use crate::common::file::read_lines;

const ROOT: &str = "src/year_2022/day_01/";

pub fn run() {
    run_part_one();
    run_part_two();
}

fn run_part_one() {
    assert_eq!(24_000, find_max("test.txt"));
    println!("Part 1: {}", find_max("input.txt"));
}

fn run_part_two() {
    assert_eq!(45_000, find_max3("test.txt"));
    println!("Part 2: {}", find_max3("input.txt"));
}

fn find_max(filename: &str) -> i32 {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut max = 0_i32;
    let mut count = 0_i32;
    for line in lines.flatten() {
        if line.is_empty() {
            if count > max {
                max = count;
            }
            count = 0;
        } else {
            count += line.parse::<i32>().unwrap();
        }
    }
    max
}

fn find_max3(filename: &str) -> i32 {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut max = [0_i32; 3];
    let mut count = 0_i32;
    for line in lines.flatten() {
        if line.is_empty() {
            if count <= max[2] {
                ();
            } else if count <= max[1] {
                max[2] = count;
            } else if count <= max[0] {
                max[2] = max[1];
                max[1] = count;
            } else {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = count;
            }
            count = 0;
        } else {
            count += line.parse::<i32>().unwrap();
        }
    }
    max[2] + max[1] + max[0]
}
