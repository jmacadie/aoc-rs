use itertools::Itertools;

pub fn run() {
    let test_data = include_str!("test.txt");
    let data = include_str!("input.txt");
    run_part_one(test_data, data);
    run_part_two(test_data, data);
}

fn run_part_one(test_data: &str, data: &str) {
    println!("Running Day 3 2022, Part 1!");
    assert_eq!(157, count_priorties(test_data));
    println!("Part 1: {} total priority", count_priorties(data));
}

fn run_part_two(test_data: &str, data: &str) {
    assert_eq!(70, count_badges(test_data));
    println!("Part 2: {} total badges priority", count_badges(data));
}

fn count_priorties(data: &str) -> i32 {
    data.trim_end_matches('\n')
        .lines()
        .map(split_in_half)
        .map(|(a, b)| find_common((a, b)).unwrap_or('a'))
        .map(priority)
        .sum()
}

fn count_badges(data: &str) -> i32 {
    let mut sum = 0;
    for lines in &data.trim_end_matches('\n').lines().chunks(3) {
        for (a, b, c) in lines.tuples() {
            let z = find_common_3((a, b, c)).unwrap();
            sum += priority(z);
        }
    }
    sum
}

fn find_common_3((a, b, c): (&str, &str, &str)) -> Option<char> {
    for a_c in a.chars() {
        for b_c in b.chars() {
            if a_c == b_c {
                for c_c in c.chars() {
                    if c_c == a_c {
                        return Some(a_c);
                    }
                }
            }
        }
    }
    None
}
fn split_in_half(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

fn find_common((a, b): (&str, &str)) -> Option<char> {
    for a_c in a.chars() {
        for b_c in b.chars() {
            if a_c == b_c {
                return Some(a_c);
            }
        }
    }
    None
}

fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}
