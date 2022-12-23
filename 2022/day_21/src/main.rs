use std::collections::HashMap;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> u64 {
    let monkeys = read_data(data);
    let root = monkeys.get("root").unwrap();
    eval(root, &monkeys)
}

fn part_two(_data: &str) -> usize {
    0
}

fn eval(calc: &MonkeyCalc, data: &Monkeys) -> u64 {
    match calc {
        MonkeyCalc::Num(val) => *val,
        MonkeyCalc::Add((a, b)) => get_monkey_number(a, data) + get_monkey_number(b, data),
        MonkeyCalc::Sub((a, b)) => get_monkey_number(a, data) - get_monkey_number(b, data),
        MonkeyCalc::Mul((a, b)) => get_monkey_number(a, data) * get_monkey_number(b, data),
        MonkeyCalc::Div((a, b)) => get_monkey_number(a, data) / get_monkey_number(b, data),
    }
}

fn get_monkey_number(e: &MonkeyCalcElem, data: &Monkeys) -> u64 {
    match e {
        MonkeyCalcElem::Number(val) => *val,
        MonkeyCalcElem::Monkey(monkey) => eval(data.get(monkey).unwrap(), data),
    }
}

fn read_data(data: &'static str) -> Monkeys {
    let mut monkeys = HashMap::new();
    for line in data.lines() {
        let (monkey, calc) = read_line(line);
        monkeys.insert(monkey, calc);
    }
    monkeys
}

fn read_line(line: &'static str) -> (&'static str, MonkeyCalc) {
    let Some((monkey, calc)) = line.split_once(": ") else {
        unreachable!();
    };
    (monkey, MonkeyCalc::new(calc))
}

type Monkeys = HashMap<&'static str, MonkeyCalc>;

#[derive(Debug)]
enum MonkeyCalc {
    Num(u64),
    Add((MonkeyCalcElem, MonkeyCalcElem)),
    Sub((MonkeyCalcElem, MonkeyCalcElem)),
    Mul((MonkeyCalcElem, MonkeyCalcElem)),
    Div((MonkeyCalcElem, MonkeyCalcElem)),
}

impl MonkeyCalc {
    fn new(s: &'static str) -> Self {
        if s.contains('+') {
            let Some((a, b)) = s.split_once(" + ") else {
                unreachable!();
            };
            let a = MonkeyCalcElem::new(a);
            let b = MonkeyCalcElem::new(b);
            MonkeyCalc::Add((a, b))
        } else if s.contains('-') {
            let Some((a, b)) = s.split_once(" - ") else {
                unreachable!();
            };
            let a = MonkeyCalcElem::new(a);
            let b = MonkeyCalcElem::new(b);
            MonkeyCalc::Sub((a, b))
        } else if s.contains('*') {
            let Some((a, b)) = s.split_once(" * ") else {
                unreachable!();
            };
            let a = MonkeyCalcElem::new(a);
            let b = MonkeyCalcElem::new(b);
            MonkeyCalc::Mul((a, b))
        } else if s.contains('/') {
            let Some((a, b)) = s.split_once(" / ") else {
                unreachable!();
            };
            let a = MonkeyCalcElem::new(a);
            let b = MonkeyCalcElem::new(b);
            MonkeyCalc::Div((a, b))
        } else {
            let a = s.parse().unwrap();
            MonkeyCalc::Num(a)
        }
    }
}

#[derive(Debug)]
enum MonkeyCalcElem {
    Number(u64),
    Monkey(&'static str),
}

impl MonkeyCalcElem {
    fn new(s: &'static str) -> Self {
        match s.parse::<u64>() {
            Ok(v) => MonkeyCalcElem::Number(v),
            Err(_) => MonkeyCalcElem::Monkey(s),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(152, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
