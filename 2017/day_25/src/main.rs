#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    let mut machine = TuringMachine::new(data);
    machine.run();
    machine.tape.diagnostic_checksum()
}

struct TuringMachine {
    rules: Rules,
    state: State,
    tape: Tape,
    steps: u32,
}

impl TuringMachine {
    fn new(data: &str) -> Self {
        let rules: Rules = data.parse().unwrap();
        let state = rules.start_state;
        Self {
            rules,
            state,
            tape: Tape::new(),
            steps: 0,
        }
    }

    fn step(&mut self) {
        let rule = self.rules.data[self.state.to_index()].unwrap();
        let instruction = if self.tape.current {
            rule.instructions[1]
        } else {
            rule.instructions[0]
        };
        self.tape.write(instruction.write);
        self.tape.rotate(instruction.next_position);
        self.state = instruction.next_state;
        self.steps += 1;
    }

    fn run(&mut self) {
        while self.steps < self.rules.checksum_after {
            self.step();
        }
    }
}

#[derive(Debug)]
struct Tape {
    current: bool,
    left: Vec<bool>,
    right: Vec<bool>,
}

impl Tape {
    fn new() -> Self {
        Self {
            current: false,
            left: Vec::with_capacity(1000),
            right: Vec::with_capacity(1000),
        }
    }

    fn write(&mut self, val: bool) {
        self.current = val;
    }

    fn rotate(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                self.right.push(self.current);
                self.current = self.left.pop().unwrap_or(false);
            }
            Direction::Right => {
                self.left.push(self.current);
                self.current = self.right.pop().unwrap_or(false);
            }
        }
    }

    fn diagnostic_checksum(&self) -> usize {
        let right = self.right.iter().filter(|&&v| v).count();
        let left = self.left.iter().filter(|&&v| v).count();
        if self.current {
            right + left + 1
        } else {
            right + left
        }
    }
}

#[derive(Debug)]
struct Rules {
    start_state: State,
    checksum_after: u32,
    data: [Option<Rule>; 6],
}

impl FromStr for Rules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [None; 6];
        let Some((init, rules_data)) = s.split_once("\n\n") else {
            return Err(format!(
                "Cannot split initial information from rules text: {s}"
            ));
        };

        let mut init_parts = init.lines();
        let (Some(start_state), Some(checksum_after), None) =
            (init_parts.next(), init_parts.next(), init_parts.next())
        else {
            return Err(format!(
                "Cannot split the initial rules information into two lines: {init}"
            ));
        };

        let start_state = match start_state
            .trim_start_matches("Begin in state ")
            .trim_end_matches('.')
        {
            "A" => State::A,
            "B" => State::B,
            "C" => State::C,
            "D" => State::D,
            "E" => State::E,
            "F" => State::F,
            _ => {
                return Err(format!("Cannot convert {start_state} to a valid state"));
            }
        };

        let checksum_after = checksum_after
            .trim_start_matches("Perform a diagnostic checksum after ")
            .trim_end_matches(" steps.")
            .parse()
            .map_err(|_| format!("Cannot convert {checksum_after} into a number"))?;

        for (data, part) in data.iter_mut().zip(rules_data.split("\n\n")) {
            *data = Some(part.parse()?);
        }
        Ok(Self {
            start_state,
            checksum_after,
            data,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Rule {
    instructions: [Instruction; 2],
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split_str(input: &str, line_count: usize) -> Option<(&str, &str)> {
            let mut newline_count = 0;
            let mut newline_chars = 1;

            for (index, character) in input.char_indices() {
                match character {
                    '\n' => newline_count += 1,
                    '\r' => {
                        if let Some((_, '\n')) = input[index + 1..].char_indices().next() {
                            newline_chars = 2;
                        }
                        newline_count += 1;
                    }
                    _ => {}
                }
                if newline_count == line_count {
                    return Some((&input[..index], &input[(index + newline_chars)..]));
                }
            }
            None // Not enough lines to split
        }

        let Some((_, body)) = split_str(s, 1) else {
            return Err(format!("Cannot split the first line off {s}"));
        };

        let Some((rule_0, rule_1)) = split_str(body, 4) else {
            return Err(format!("Cannot split into two block of 4 lines: {body}"));
        };

        let Some((header_0, body_0)) = split_str(rule_0, 1) else {
            return Err(format!("Cannot split header from: {rule_0}"));
        };

        if header_0 != "  If the current value is 0:" {
            return Err(format!(
                "Rule 0 doesn't have the expected header: {header_0}"
            ));
        }

        let rule_0: Instruction = body_0.parse()?;

        let Some((header_1, body_1)) = split_str(rule_1, 1) else {
            return Err(format!("Cannot split header from: {rule_1}"));
        };

        if header_1 != "  If the current value is 1:" {
            return Err(format!(
                "Rule 1 doesn't have the expected header: {header_1}"
            ));
        }

        let rule_1: Instruction = body_1.parse()?;

        Ok(Self {
            instructions: [rule_0, rule_1],
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    write: bool,
    next_position: Direction,
    next_state: State,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.lines();
        let (Some(write), Some(next_position), Some(next_state), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(format!("Expecting instruction to be in three parts: {s}"));
        };
        let val = write
            .trim_start_matches("    - Write the value ")
            .trim_end_matches('.');
        if val.len() != 1 {
            return Err(format!("Cannot parse rule into a write command: {write}"));
        }
        let write = match val {
            "0" => false,
            "1" => true,
            _ => {
                return Err(format!("Cannot convert {write} into 0 or 1"));
            }
        };
        let next_position = next_position.parse()?;
        let next_state = next_state.parse()?;
        Ok(Self {
            write,
            next_position,
            next_state,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s
            .trim_start_matches("    - Move one slot to the ")
            .trim_end_matches('.');
        match val {
            "right" => Ok(Self::Right),
            "left" => Ok(Self::Left),
            _ => Err(format!("Cannot parse rule into a valid direction: {s}")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl State {
    const fn to_index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::F => 5,
        }
    }
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s
            .trim_start_matches("    - Continue with state ")
            .trim_end_matches('.');
        if val.len() != 1 {
            return Err(format!("Cannot parse rule into a state: {s}"));
        }
        match val {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            _ => Err(format!("Cannot parse rule into a valid state: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(3, part_one(data));
    }
}
