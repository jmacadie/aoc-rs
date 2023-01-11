#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u16 {
    for line in data.lines() {
        let (num, data) = read_line(line);
        if sue_matches_1(data) {
            return num;
        }
    }
    0
}

fn part_two(data: &str) -> u16 {
    for line in data.lines() {
        let (num, data) = read_line(line);
        if sue_matches_2(data) {
            return num;
        }
    }
    0
}

type Sue = [u8; 10];
const TARGET: Sue = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

fn read_line(line: &str) -> (u16, Sue) {
    let mut sue = [u8::MAX; 10];

    let (sue_num, data) = line.split_once(": ").unwrap();
    let sue_num = sue_num.trim_start_matches("Sue ").parse().unwrap();

    for part in data.split(", ") {
        let (key, num) = part.split_once(": ").unwrap();
        let num = num.parse().unwrap();
        match key {
            "children" => sue[0] = num,
            "cats" => sue[1] = num,
            "samoyeds" => sue[2] = num,
            "pomeranians" => sue[3] = num,
            "akitas" => sue[4] = num,
            "vizslas" => sue[5] = num,
            "goldfish" => sue[6] = num,
            "trees" => sue[7] = num,
            "cars" => sue[8] = num,
            "perfumes" => sue[9] = num,
            _ => unreachable!(),
        }
    }

    (sue_num, sue)
}

fn sue_matches_1(test: Sue) -> bool {
    for (poss, target) in test.into_iter().zip(TARGET.into_iter()) {
        if poss != u8::MAX && poss != target {
            return false;
        }
    }
    true
}

fn sue_matches_2(test: Sue) -> bool {
    for (i, poss) in test.into_iter().enumerate() {
        if poss != u8::MAX {
            match i {
                1 | 7 => {
                    if poss <= TARGET[i] {
                        return false;
                    }
                }
                3 | 6 => {
                    if poss >= TARGET[i] {
                        return false;
                    }
                }
                _ => {
                    if poss != TARGET[i] {
                        return false;
                    }
                }
            }
        }
    }
    true
}
