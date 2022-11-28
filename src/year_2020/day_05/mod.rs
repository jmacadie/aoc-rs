use crate::common::file::read_lines;

const ROOT: &str = "src/year_2020/day_05/";

pub fn run() {
    run_part_one();
    run_part_two();
}

fn run_part_one() {
    assert_eq!(820, find_highest("test.txt"));
    let max_seat_id = find_highest("input.txt");
    println!("Part 1: Highest seat number is {}", max_seat_id);
}

fn run_part_two() {
    let arr = get_sorted_seats("input.txt");
    let mut iter = arr.into_iter();
    let mut last = iter.next().unwrap();
    for i in iter {
        last += 1;
        if i > last {
            println!("Part 2: first missing seat is {}", last);
            return;
        }
    }
}

fn find_highest(filename: &str) -> i32 {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut max = 0;
    for line in lines.flatten() {
        let (_, _, seat_id) = convert_seat(&line);
        if seat_id > max {
            max = seat_id;
        }
    }
    max
}

fn convert_seat(pass: &str) -> (i32, i32, i32) {
    let (row, col) = pass.split_at(7);

    let row_bin: String = row
        .chars()
        .map(|c| if c == 'B' { '1' } else { '0' })
        .collect();
    let row_dec = i32::from_str_radix(&row_bin, 2).unwrap();

    let col_bin: String = col
        .chars()
        .map(|c| if c == 'R' { '1' } else { '0' })
        .collect();
    let col_dec = i32::from_str_radix(&col_bin, 2).unwrap();

    let seat_id = row_dec * 8 + col_dec;
    (row_dec, col_dec, seat_id)
}

fn get_sorted_seats(filename: &str) -> Vec<i32> {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut out = Vec::new();
    for line in lines.flatten() {
        let (_, _, seat_id) = convert_seat(&line);
        out.push(seat_id);
    }
    out.sort_unstable();
    out
}
