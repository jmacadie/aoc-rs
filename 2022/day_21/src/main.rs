use std::collections::{HashMap, HashSet};

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

fn part_two(data: &'static str) -> u64 {
    let mut monkeys = read_data(data);
    reorder_monkeys(&mut monkeys);
    let root = monkeys.get("humn").unwrap();
    eval(root, &monkeys)
}

fn reorder_monkeys(data: &mut Monkeys) {
    let mut monkey = "humn";
    let mut moved = HashSet::new();
    while monkey != "root" {
        moved.insert(monkey);
        monkey = rewrite_monkey(data, monkey, &moved);
    }
}

fn rewrite_monkey(
    data: &mut Monkeys,
    monkey: &'static str,
    moved: &HashSet<&'static str>,
) -> &'static str {
    let target = MonkeyCalcElem::Monkey(monkey);
    for (&next, &next_calc) in data.iter() {
        if moved.contains(next) {
            continue;
        }
        match next_calc {
            MonkeyCalc::Add((a, b)) => {
                if a == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), b));
                        } else {
                            *x = MonkeyCalc::Sub((MonkeyCalcElem::Monkey(next), b));
                        }
                    }
                    return next;
                } else if b == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), a));
                        } else {
                            *x = MonkeyCalc::Sub((MonkeyCalcElem::Monkey(next), a));
                        }
                    }
                    return next;
                }
            }
            MonkeyCalc::Sub((a, b)) => {
                if a == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), b));
                        } else {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Monkey(next), b));
                        }
                    }
                    return next;
                } else if b == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), a));
                        } else {
                            *x = MonkeyCalc::Sub((a, MonkeyCalcElem::Monkey(next)));
                        }
                    }
                    return next;
                }
            }
            MonkeyCalc::Mul((a, b)) => {
                if a == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), b));
                        } else {
                            *x = MonkeyCalc::Div((MonkeyCalcElem::Monkey(next), b));
                        }
                    }
                    return next;
                } else if b == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), a));
                        } else {
                            *x = MonkeyCalc::Div((MonkeyCalcElem::Monkey(next), a));
                        }
                    }
                    return next;
                }
            }
            MonkeyCalc::Div((a, b)) => {
                if a == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), b));
                        } else {
                            *x = MonkeyCalc::Mul((MonkeyCalcElem::Monkey(next), b));
                        }
                    }
                    return next;
                } else if b == target {
                    if let Some(x) = data.get_mut(monkey) {
                        if next == "root" {
                            *x = MonkeyCalc::Add((MonkeyCalcElem::Number(0), a));
                        } else {
                            *x = MonkeyCalc::Div((a, MonkeyCalcElem::Monkey(next)));
                        }
                    }
                    return next;
                }
            }
            MonkeyCalc::Num(_) => (),
        }
    }
    unreachable!();
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

#[derive(Debug, Clone, Copy)]
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
            MonkeyCalc::Add(Self::split(s, '+'))
        } else if s.contains('-') {
            MonkeyCalc::Sub(Self::split(s, '-'))
        } else if s.contains('*') {
            MonkeyCalc::Mul(Self::split(s, '*'))
        } else if s.contains('/') {
            MonkeyCalc::Div(Self::split(s, '/'))
        } else {
            MonkeyCalc::Num(s.parse().unwrap())
        }
    }

    fn split(s: &'static str, op: char) -> (MonkeyCalcElem, MonkeyCalcElem) {
        let pattern = format!(" {op} ");
        let Some((a, b)) = s.split_once(&pattern) else {
            unreachable!();
        };
        (MonkeyCalcElem::new(a), MonkeyCalcElem::new(b))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        assert_eq!(301, part_two(data));
    }
}
