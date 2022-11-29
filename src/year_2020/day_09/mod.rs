use crate::common::file::read_lines;

const ROOT: &str = "src/year_2020/day_09/";

pub fn run() {
    let test = Transmission::new("test.txt", 5);
    let main = Transmission::new("input.txt", 25);

    run_part_one(&test, &main);
    run_part_two(&test, &main);
}

fn run_part_one(test_trans: &Transmission, main_trans: &Transmission) {
    assert_eq!(127, test_trans.find_first_invalid());

    let val = main_trans.find_first_invalid();
    println!("Part 1: First invalid number is {}", val);
}

fn run_part_two(test_trans: &Transmission, main_trans: &Transmission) {
    assert_eq!(62, test_trans.find_encryption_weakness());

    let val = main_trans.find_encryption_weakness();
    println!("Part 2: Encryption weakness found - {}", val);
}

struct Transmission {
    data: Vec<u64>,
    window: usize,
}

impl Transmission {
    fn new(filename: &str, window: usize) -> Transmission {
        let file = format!("{}{}", ROOT, filename);
        let lines = read_lines(file).unwrap();
        let mut data = Vec::new();
        for line in lines.flatten() {
            data.push(line.parse().unwrap());
        }
        Transmission { data, window }
    }

    fn find_encryption_weakness(&self) -> u64 {
        let target = self.find_first_invalid();
        for position in 0.. {
            if let Some(slice) = self.encryption_weakness_at(position, target) {
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return min + max;
            }
        }
        0
    }

    fn encryption_weakness_at(&self, position: usize, target: u64) -> Option<&[u64]> {
        let mut sum = 0_u64;
        for end in position.. {
            sum += self.data[end];
            if sum > target {
                return None;
            }
            if sum == target {
                return Some(&self.data[position..end]);
            }
        }
        None
    }

    fn find_first_invalid(&self) -> u64 {
        for position in self.window..self.data.len() {
            if !self.valid_at(position) {
                return self.data[position];
            }
        }
        0
    }

    fn valid_at(&self, position: usize) -> bool {
        assert!(position >= self.window, "Position cannot be in preamble");
        let target = self.data[position];
        let start = position - self.window;
        let source = &self.data[start..position];
        Self::find_sum_pair(target, source)
    }

    fn find_sum_pair(target: u64, source: &[u64]) -> bool {
        for (i, val_1) in source.iter().enumerate() {
            for val_2 in source[i + 1..].iter() {
                if val_1 + val_2 == target {
                    return true;
                }
            }
        }
        false
    }
}
