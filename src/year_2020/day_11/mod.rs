#![warn(unused_imports, dead_code)]
use crate::common::file::read_lines;
use crate::common::map_2d;

const ROOT: &str = "src/year_2020/day_11/";

pub fn run() {
    run_part_one();
    run_part_two();
}

fn run_part_one() {
    println!("Running Day 11 2020, Part 1!");
    let test = WaitingRoom::new("test.txt");
    for (p, st) in &test.seats {
        println!("{:?} - {:?}", p, st);
    }
}

fn run_part_two() {
    println!("Running Day 11 2020, Part 2!");
}

#[derive(Debug)]
enum SeatType {
    Floor,
    Vacant,
    Occupied,
}

struct WaitingRoom {
    seats: map_2d::Map<SeatType>,
}

impl WaitingRoom {
    fn new(filename: &str) -> Self {
        let file = format!("{}{}", ROOT, filename);
        let lines = read_lines(file).unwrap();
        let mut floor_data = Vec::new();
        for line in lines.flatten() {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => row.push(SeatType::Floor),
                    'L' => row.push(SeatType::Vacant),
                    '#' => row.push(SeatType::Occupied),
                    _ => unreachable!(),
                }
            }
            floor_data.push(row);
        }
        let seats = map_2d::Map::new(floor_data);
        WaitingRoom { seats }
    }
}
