use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ROOT: &str = "src/y_2020/d_01/";
const TARGET: i32 = 2020;

#[allow(clippy::missing_panics_doc)]
pub fn run () {
    let test_data = read_into_vec("test.txt").unwrap();
    let data = read_into_vec("input.txt").unwrap();

    // Part 1
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Test
    if let Some((a, b ,p)) = run_part_one(&test_data, TARGET) {
        assert_eq!(1721, a);
        assert_eq!(299, b);
        assert_eq!(514_579, p);
    }
    
    // Main
    if let Some((a, b, p)) = run_part_one(&data, TARGET) {
        println!("Part 1 ### val 1: {}, val 2: {}, product: {}", a, b, p);
    }

    // Part 2
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Test
    if let Some((a, b, c, p)) = run_part_two(&test_data, TARGET) {
        assert_eq!(979, a);
        assert_eq!(366, b);
        assert_eq!(675, c);
        assert_eq!(241_861_950, p);
    }
    
    // Main
    if let Some((a, b, c, p)) = run_part_two(&data, TARGET) {
        println!("Part 2 ### val 1: {}, val 2: {}, val 3: {}, product: {}", a, b, c, p);
    }
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run_part_two(data: &[i32], target: i32) -> Option<(i32, i32, i32, i32)> {
    for (i, _) in data.iter().enumerate() {
        let (_, arr) = data.split_at(i);
        if let Some((val, tail)) = arr.split_first() {
            if let Some((a, b, p)) = run_part_one(tail, target - val) {
                let p = p * val;
                return Some((*val, a, b, p));
            }
        }
    }
    None
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run_part_one(data: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    for (i, _) in data.iter().enumerate() {
        if let Some((a, b)) = match_at(data, i, target) {
            let p = a * b;
            return Some((a, b, p));
        }
    }
    None
}

fn match_at(data: &[i32], start:usize, target: i32) -> Option<(i32, i32)> {
    let (_, search) = data.split_at(start);
    if let Some((val, search)) = search.split_first() {
        for other in search {
            if val + other == target {
                return Some((*val, *other));
            }
        }
    }
    None
}

fn read_into_vec(filename: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let file = format!("{}{}", ROOT, filename);
    let len = count_lines(file.clone())?;
    let mut out = Vec::with_capacity(len);

    let lines = read_lines(file)?;
    for line in lines.flatten() {
        let i = line.parse::<i32>()?;
        out.push(i);
    }
    Ok(out)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_lines<P>(filename: P) -> io::Result<usize>
where P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    Ok(lines.count())
}

