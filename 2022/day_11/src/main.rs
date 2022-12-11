use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> u64 {
    let data = parse_monkeys(data);
    let inspections = solve::<20, 3>(data);
    inspections
        .into_iter()
        .sorted_unstable_by_key(|&v| std::cmp::Reverse(v))
        .take(2)
        .product()
}

fn part_two(data: &str) -> u64 {
    let data = parse_monkeys(data);
    let inspections = solve::<10_000, 1>(data);
    inspections
        .into_iter()
        .sorted_unstable_by_key(|&v| std::cmp::Reverse(v))
        .take(2)
        .product()
}

fn solve<const ROUNDS: usize, const DIVISOR: u64>(mut data: Data) -> [u64; 8] {
    let mut inspections = [0_u64; 8];
    let lim: u64 = data.tests.iter().filter(|&v| v > &0).product();
    for _ in 0..ROUNDS {
        for (monkey, item) in inspections.iter_mut().enumerate() {
            for item_num in 0_usize..36 {
                if data.items[monkey][item_num] == 0 {
                    break;
                }
                *item += 1;
                let new = (update_worry(data.operations[monkey], data.items[monkey][item_num]) / DIVISOR) % lim;
                if test_worry(data.tests[monkey], new) {
                    pass_item(&mut data.items, (monkey, item_num), data.next[monkey].0, new);
                } else {
                    pass_item(&mut data.items, (monkey, item_num), data.next[monkey].1, new);
                }
            }
        }
    }
    inspections
}

fn pass_item(items: &mut Items, from: (usize, usize), to: usize, item: u64) {
    let mut to_pos = 0_usize;
    while items[to][to_pos] != 0 {
        to_pos += 1;
    }
    items[to][to_pos] = item;
    items[from.0][from.1] = 0;
}

fn update_worry(operation: Operation, item: u64) -> u64 {
    match operation {
        Operation::Add(x) => item + x,
        Operation::Mul(x) => item * x,
        Operation::Sqr => item * item,
    }
}

fn test_worry(test: u64, worry: u64) -> bool {
    worry % test == 0
}

type Items = [[u64; 36]; 8];

#[derive(Clone, Copy)]
enum Operation {
    Add(u64),
    Mul(u64),
    Sqr,
}

struct Data {
    items: [[u64; 36]; 8],
    operations: [Operation; 8],
    tests: [u64; 8],
    next: [(usize, usize); 8],
}

fn parse_monkeys(data: &str) -> Data {
    let mut items: Items = [[0; 36]; 8];
    let mut operations: [Operation; 8] = [Operation::Mul(1); 8];
    let mut tests: [u64; 8] = [0; 8];
    let mut next: [(usize, usize); 8] = [(0, 0); 8];
    let parts = data.split("\n\n");
    for (idx, monkey) in parts.enumerate() {
        let (func, test, pass) = parse_monkey(monkey, &mut items);
        operations[idx] = func;
        tests[idx] = test;
        next[idx] = pass;
    }
    Data { items, operations, tests, next }
}

fn parse_monkey(data: &str, items: &mut Items) -> (Operation, u64, (usize, usize)) {
    let mut monkey: usize = 0;
    let mut operation: Operation = Operation::Mul(1);
    let mut test: u64 = 0;
    let mut pass_true: usize = 0;
    let mut pass_false: usize = 0;
    for line in data.lines() {
        let mut parts = line.trim_start().split(':');
        let (Some(header), Some(data), None) = (parts.next(), parts.next(), parts.next()) else {
            panic!("Input line not well formatted: {line}");
        };
        match header {
            "Starting items" => {
                let item_parts = data.split(',');
                for (idx, thing) in item_parts.enumerate() {
                    items[monkey][idx] = thing.trim().parse().unwrap();
                }
            }
            "Operation" => {
                let mut data_parts = data.trim().split('=');
                let (Some("new "), Some(op), None) = (data_parts.next(), data_parts.next(), data_parts.next()) else {
                    panic!("Operation line is not well formatted: {data}");
                };
                let mut operation_parts = op.trim().split(' ');
                let (Some("old"), Some(operator), Some(lhs), None) = (operation_parts.next(), operation_parts.next(), operation_parts.next(), operation_parts.next()) else {
                    panic!("Invalid operation string: {op}");
                };
                operation = match (operator, lhs) {
                    ("+", "old") => Operation::Mul(2),
                    ("+", val) => Operation::Add(val.parse::<u64>().unwrap()),
                    ("*", "old") => Operation::Sqr,
                    ("*", val) => Operation::Mul(val.parse::<u64>().unwrap()),
                    _ => unreachable!(),
                };
            }
            "Test" => {
                let mut data_parts = data.trim().split(' ');
                let (Some("divisible"), Some("by"), Some(val), None) = (data_parts.next(), data_parts.next(), data_parts.next(), data_parts.next()) else {
                    panic!("Test line is not well formatted: {data}");
                };
                test = val.parse().unwrap();
            }
            "If true" => {
                let mut data_parts = data.trim().split(' ');
                let (Some("throw"), Some("to"), Some("monkey"), Some(val), None) = (data_parts.next(), data_parts.next(), data_parts.next(), data_parts.next(), data_parts.next()) else {
                    panic!("If true line is not well formatted: {data}");
                };
                pass_true = val.parse().unwrap();
            }
            "If false" => {
                let mut data_parts = data.trim().split(' ');
                let (Some("throw"), Some("to"), Some("monkey"), Some(val), None) = (data_parts.next(), data_parts.next(), data_parts.next(), data_parts.next(), data_parts.next()) else {
                    panic!("If false line is not well formatted: {data}");
                };
                pass_false = val.parse().unwrap();
            }
            _ => {
                let mut header_parts = header.split(' ');
                match (header_parts.next(), header_parts.next()) {
                    (Some("Monkey"), Some(val)) => monkey = val.parse().unwrap(),
                    _ => unreachable!(),
                }
            }
        }
    }
    (operation, test, (pass_true, pass_false))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(10_605, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(2_713_310_158, part_two(data));
    }
}
