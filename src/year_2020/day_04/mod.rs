use crate::common::file::read_lines;

const ROOT: &str = "src/year_2020/day_04/";

pub fn run() {
    run_part_one();
    run_part_two();
}

enum CheckType {
    Fields,
    Full,
}

fn run_part_one() {
    let method = CheckType::Fields;
    assert_eq!(2, count_valid("test.txt", &method));
    println!(
        "Part 1: {} valid passports",
        count_valid("input.txt", &method)
    );
}

fn run_part_two() {
    let method = CheckType::Full;
    assert_eq!(0, count_valid("test_invalid.txt", &method));
    assert_eq!(4, count_valid("test_valid.txt", &method));
    println!(
        "Part 2: {} valid passports",
        count_valid("input.txt", &method)
    );
}

fn count_valid(filename: &str, method: &CheckType) -> i32 {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut fields = [false; 8];
    let mut count: i32 = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            if is_valid(&fields) {
                count += 1;
            }
            fields = [false; 8];
        } else {
            let parts = line.split(' ');
            for part in parts {
                let mut key_val = part.split(':');
                let key = key_val.next().unwrap();
                match method {
                    CheckType::Fields => mark_key(key, &mut fields),
                    CheckType::Full => {
                        let val = key_val.next().unwrap();
                        validate_key(key, val, &mut fields);
                    }
                };
            }
        }
    }
    count
}

fn is_valid(fields: &[bool]) -> bool {
    fields[0] && fields[1] && fields[2] && fields[3] && fields[4] && fields[5] && fields[6]
}

fn validate_key(key: &str, val: &str, fields: &mut [bool]) {
    match key {
        "byr" => {
            fields[0] = validate_year(val, 1920, 2002);
        }
        "iyr" => {
            fields[1] = validate_year(val, 2010, 2020);
        }
        "eyr" => {
            fields[2] = validate_year(val, 2020, 2030);
        }
        "hgt" => {
            fields[3] = validate_height(val);
        }
        "hcl" => {
            fields[4] = validate_hair_colour(val);
        }
        "ecl" => {
            fields[5] = validate_eye_colour(val);
        }
        "pid" => {
            fields[6] = validate_pid(val);
        }
        "cid" => {
            fields[7] = true;
        }
        _ => {
            unreachable!();
        }
    }
}

fn validate_num(val: &str, len: usize, min: i32, max: i32) -> bool {
    if val.len() != len {
        return false;
    }
    for c in val.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    let val_num: i32 = val.parse().unwrap();
    (val_num >= min) && (val_num <= max)
}

fn validate_year(val: &str, min: i32, max: i32) -> bool {
    validate_num(val, 4, min, max)
}

fn validate_height(val: &str) -> bool {
    let (num, suffix) = val.split_at(val.len() - 2);
    match suffix {
        "cm" => validate_num(num, 3, 150, 193),
        "in" => validate_num(num, 2, 59, 76),
        _ => false,
    }
}

fn validate_hair_colour(val: &str) -> bool {
    if val.len() != 7 {
        return false;
    }
    let mut chars = val.chars();
    if chars.next().unwrap() != '#' {
        return false;
    }
    for c in chars {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

fn validate_pid(val: &str) -> bool {
    if val.len() != 9 {
        return false;
    }
    for c in val.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

fn validate_eye_colour(val: &str) -> bool {
    matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn mark_key(key: &str, fields: &mut [bool]) {
    match key {
        "byr" => {
            fields[0] = true;
        }
        "iyr" => {
            fields[1] = true;
        }
        "eyr" => {
            fields[2] = true;
        }
        "hgt" => {
            fields[3] = true;
        }
        "hcl" => {
            fields[4] = true;
        }
        "ecl" => {
            fields[5] = true;
        }
        "pid" => {
            fields[6] = true;
        }
        "cid" => {
            fields[7] = true;
        }
        _ => {
            unreachable!();
        }
    }
}
