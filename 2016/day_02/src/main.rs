#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let mut kp = Keypad::default();
    for line in data.lines() {
        kp.process_line(line);
    }
    kp.code
}

fn part_two(data: &str) -> String {
    let mut kp = Keypad2::default();
    for line in data.lines() {
        kp.process_line(line);
    }
    kp.code
}

struct Keypad2 {
    row: u8,
    col: u8,
    code: String,
}

impl Default for Keypad2 {
    fn default() -> Self {
        Self {
            row: 3,
            col: 1,
            code: String::new(),
        }
    }
}

impl Keypad2 {
    fn next(&mut self, instruction: char) {
        match (self.row, self.col, instruction) {
            // No-ops first
              (3, 1, 'U' | 'D' | 'L')  // 5
            | (2, 2, 'U' | 'L')        // 2
            | (1, 3, 'U' | 'L' | 'R')  // 1
            | (2, 4, 'U' | 'R')        // 4
            | (3, 5, 'U' | 'D' | 'R')  // 9
            | (4, 4, 'D' | 'R')        // C
            | (5, 3, 'D' | 'L' | 'R')  // D
            | (4, 2, 'D' | 'L') => (), // A
            (_, _, 'U') => self.row -= 1,
            (_, _, 'D') => self.row += 1,
            (_, _, 'L') => self.col -= 1,
            (_, _, 'R') => self.col += 1,
            _ => unreachable!(),
        }
    }

    fn process_line(&mut self, line: &str) {
        for instruction in line.chars() {
            self.next(instruction);
        }
        self.code.push(self.number());
    }

    fn number(&self) -> char {
        match (self.row, self.col) {
            (1, 3) => '1',
            (2, 2) => '2',
            (2, 3) => '3',
            (2, 4) => '4',
            (3, 1) => '5',
            (3, 2) => '6',
            (3, 3) => '7',
            (3, 4) => '8',
            (3, 5) => '9',
            (4, 2) => 'A',
            (4, 3) => 'B',
            (4, 4) => 'C',
            (5, 3) => 'D',
            _ => unreachable!(),
        }
    }
}

struct Keypad {
    row: u8,
    col: u8,
    code: u32,
}

impl Default for Keypad {
    fn default() -> Self {
        Self {
            row: 2,
            col: 2,
            code: 0,
        }
    }
}

impl Keypad {
    fn next(&mut self, instruction: char) {
        match instruction {
            'U' => self.row = std::cmp::max(self.row - 1, 1),
            'D' => self.row = std::cmp::min(self.row + 1, 3),
            'L' => self.col = std::cmp::max(self.col - 1, 1),
            'R' => self.col = std::cmp::min(self.col + 1, 3),
            _ => unreachable!(),
        }
    }

    fn process_line(&mut self, line: &str) {
        for instruction in line.chars() {
            self.next(instruction);
        }
        self.code *= 10;
        self.code += self.number();
    }

    fn number(&self) -> u32 {
        match (self.row, self.col) {
            (1, 1) => 1,
            (1, 2) => 2,
            (1, 3) => 3,
            (2, 1) => 4,
            (2, 2) => 5,
            (2, 3) => 6,
            (3, 1) => 7,
            (3, 2) => 8,
            (3, 3) => 9,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(1985, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!("5DB3", part_two(data));
    }
}
