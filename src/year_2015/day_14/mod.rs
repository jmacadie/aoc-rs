use crate::common::file::read_lines;

const ROOT: &str = "src/year_2015/day_14/";

pub fn run() {
    run_part_one();
    run_part_two();
}

fn run_part_one() {
    println!("Running Day 14 2015, Part 1!");
}

/*fn run_part_one(test: &Thing, main: &Thing) {
    assert_eq!(220, test.output());
    println!("Part 1: Thing output is {}", main.output());
}*/

fn run_part_two() {
    println!("Running Day 14 2015, Part 2!");
}

/*

fn new(filename: &str) -> Vec<i32> {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut list = Vec::new();
    for line in lines.flatten() {
        list.push(line.parse().unwrap());
    }
    list.sort_unstable();
    list.push(list.last().unwrap() + 3);
    list
}
*/
