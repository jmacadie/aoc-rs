use crate::common::file::read_lines;

const ROOT: &str = "src/year_2020/day_08/";

pub fn run() {
    let test = load_instructions("test.txt");
    let main = load_instructions("input.txt");

    run_part_one(&test, &main);
    run_part_two(&test, &main);
}

fn run_part_one(test_ins: &Instructions, main_ins: &Instructions) {
    let mut test = Programme::new(test_ins);
    test.run(0);
    assert_eq!(5, test.value());

    let mut main = Programme::new(main_ins);
    main.run(0);
    println!("Part 1: Accumulator gets to {}", main.value());
}

fn run_part_two(test_ins: &Instructions, main_ins: &Instructions) {
    let mut val = find_valid_programme(test_ins);
    assert_eq!(8, val);

    val = find_valid_programme(main_ins);
    println!("Part 2: Found valid programme, terminaing with {}", val);
}

fn find_valid_programme(ins: &Instructions) -> i32 {
    for i in 0.. {
        let mut prog = Programme::new(ins);
        prog.run(i);
        if !prog.looping {
            return prog.value();
        }
    }
    0
}

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

type Instructions = Vec<Instruction>;

fn load_instructions(filename: &str) -> Instructions {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut instructions = Vec::new();
    for line in lines.flatten() {
        let mut parts = line.split(' ');
        let op = parts.next().unwrap();
        let val = parts.next().unwrap().parse().unwrap();
        let instruction = match op {
            "acc" => Instruction::Acc(val),
            "jmp" => Instruction::Jmp(val),
            "nop" => Instruction::Nop(val),
            _ => unreachable!(),
        };
        instructions.push(instruction);
    }
    instructions
}

struct Programme<'a> {
    instructions: &'a Instructions,
    accumulator: i32,
    line: i32,
    switch_statements: i32,
    visited: Vec<i32>,
    looping: bool,
}

impl<'a> Programme<'a> {
    fn new(instructions: &'a Instructions) -> Self {
        Programme {
            instructions,
            accumulator: 0,
            line: 1,
            switch_statements: 0,
            visited: Vec::new(),
            looping: false,
        }
    }

    fn value(&self) -> i32 {
        self.accumulator
    }

    fn run(&mut self, switch: i32) {
        while !self.looping && !self.at_end() {
            self.step(switch);
        }
    }

    fn at_end(&self) -> bool {
        let line: usize = self.line.try_into().unwrap();
        line > self.instructions.len()
    }

    fn step(&mut self, switch: i32) {
        self.visited.push(self.line);
        match *self.get_instruction() {
            Instruction::Acc(val) => {
                self.accumulator += val;
                self.line += 1;
            }
            Instruction::Jmp(val) => {
                self.switch_statements += 1;
                if self.switch_statements == switch {
                    self.line += 1;
                } else {
                    self.line += val;
                }
            }
            Instruction::Nop(val) => {
                self.switch_statements += 1;
                if self.switch_statements == switch {
                    self.line += val;
                } else {
                    self.line += 1;
                }
            }
        };
        self.check_loop();
    }

    fn get_instruction(&self) -> &Instruction {
        let line: usize = self.line.try_into().unwrap();
        &self.instructions[line - 1]
    }

    fn check_loop(&mut self) {
        if self.visited.contains(&self.line) {
            self.looping = true;
        }
    }
}
