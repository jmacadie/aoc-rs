use crate::common::file::read_lines;
use std::io;

const ROOT: &str = "src/year_2020/day_02/";

enum PasswordPolicy {
    Policy1 {
        min: i32,
        max: i32,
        character: char,
    },
    Policy2 {
        first: i32,
        second: i32,
        character: char,
    },
}

struct Password {
    password: String,
    policy: PasswordPolicy,
}

impl Password {
    fn new(input: &str, policy_type: i32) -> Self {
        // Initial split of input
        let mut parts = input.split(' ');
        let nums_part = parts.next().unwrap();
        let char_part = parts.next().unwrap();
        let pword_part = parts.next().unwrap();

        // Parse out min and max
        let mut nums = nums_part.split('-');
        let one: i32 = nums.next().unwrap().parse().unwrap();
        let two: i32 = nums.next().unwrap().parse().unwrap();

        // Parse out Password char
        let character = char_part.trim_end_matches(':').chars().next().unwrap();

        // Convert the password into a String
        let password = pword_part.to_owned();

        let policy = match policy_type {
            1 => PasswordPolicy::Policy1 {
                min: one,
                max: two,
                character,
            },
            _ => PasswordPolicy::Policy2 {
                first: one,
                second: two,
                character,
            },
        };
        // return the struct
        Password { password, policy }
    }

    fn is_valid(&self) -> bool {
        match self.policy {
            PasswordPolicy::Policy1 {
                min,
                max,
                character,
            } => {
                let count = self.password.chars().filter(|c| c == &character).count();
                let count: i32 = count.try_into().unwrap();
                count >= min && count <= max
            }
            PasswordPolicy::Policy2 {
                first,
                second,
                character,
            } => {
                let first: usize = first.try_into().unwrap();
                let second: usize = second.try_into().unwrap();
                let mut chars = self.password.chars();
                let first_char = chars.nth(first - 1).unwrap();
                let second_char = chars.nth(second - first - 1).unwrap();
                (first_char == character && second_char != character)
                    || (first_char != character && second_char == character)
            }
        }
    }
}

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    let test = count_valid("test.txt", 1).unwrap();
    assert_eq!(2, test);
    let main = count_valid("input.txt", 1).unwrap();
    println!("Part 1: {}", main);
}

fn part_two() {
    let test = count_valid("test.txt", 2).unwrap();
    assert_eq!(1, test);
    let main = count_valid("input.txt", 2).unwrap();
    println!("Part 2: {}", main);
}

fn count_valid(filename: &str, policy_type: i32) -> io::Result<i32> {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file)?;
    let mut count: i32 = 0;
    for line in lines.flatten() {
        if Password::new(&line, policy_type).is_valid() {
            count += 1;
        }
    }
    Ok(count)
}
