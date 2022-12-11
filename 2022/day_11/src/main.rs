use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> u64 {
    let (mut items, operations, tests, next) = parse_monkeys(data);
    let lim: u64 = tests.iter().filter(|&v| v > &0).product();
    let mut inspections = [0_u64; 8];
    for _ in 0..20 {
        inspect_round(
            &mut items,
            &mut inspections,
            operations,
            tests,
            next,
            3,
            lim,
        );
    }
    inspections
        .into_iter()
        .sorted_unstable_by_key(|&v| std::cmp::Reverse(v))
        .take(2)
        .product()
}

fn part_two(data: &'static str) -> u64 {
    let (mut items, operations, tests, next) = parse_monkeys(data);
    let lim: u64 = tests.iter().filter(|&v| v > &0).product();
    let mut inspections = [0_u64; 8];
    for _ in 0..10_000 {
        inspect_round(
            &mut items,
            &mut inspections,
            operations,
            tests,
            next,
            1,
            lim,
        );
    }
    inspections
        .into_iter()
        .sorted_unstable_by_key(|&v| std::cmp::Reverse(v))
        .take(2)
        .product()
}

fn inspect_round(
    items: &mut Items,
    inspections: &mut [u64; 8],
    operations: [&str; 8],
    tests: [u64; 8],
    next: [(usize, usize); 8],
    divisor: u64,
    limit: u64,
) {
    for monkey in 0_usize..8 {
        for item_num in 0_usize..36 {
            if items[monkey][item_num] == 0 {
                break;
            }
            inspections[monkey] += 1;
            let new = update_worry(operations[monkey], items[monkey][item_num], divisor) % limit;
            if test_worry(tests[monkey], new) {
                pass_item(items, (monkey, item_num), next[monkey].0, new);
            } else {
                pass_item(items, (monkey, item_num), next[monkey].1, new);
            }
        }
    }
}

fn pass_item(items: &mut Items, from: (usize, usize), to: usize, item: u64) {
    let mut to_pos = 0_usize;
    while items[to][to_pos] != 0 {
        to_pos += 1;
    }
    items[to][to_pos] = item;
    items[from.0][from.1] = 0;
}

fn update_worry(operation: &str, item: u64, divisor: u64) -> u64 {
    let mut operation_parts = operation.split(' ');
    let (Some("old"), Some(operator), Some(lhs), None) = (operation_parts.next(), operation_parts.next(), operation_parts.next(), operation_parts.next()) else {
        panic!("Invalid operation strinf: {operation}");
    };
    let new_worry = match (operator, lhs) {
        ("+", "old") => item + item,
        ("+", val) => item + val.parse::<u64>().unwrap(),
        ("*", "old") => item * item,
        ("*", val) => item * val.parse::<u64>().unwrap(),
        _ => unreachable!(),
    };
    new_worry / divisor
}

fn test_worry(test: u64, worry: u64) -> bool {
    worry % test == 0
}

type Items = [[u64; 36]; 8];

fn parse_monkeys(data: &'static str) -> (Items, [&'static str; 8], [u64; 8], [(usize, usize); 8]) {
    let mut items: Items = [[0; 36]; 8];
    let mut operations: [&str; 8] = [""; 8];
    let mut tests: [u64; 8] = [0; 8];
    let mut next: [(usize, usize); 8] = [(0, 0); 8];
    let parts = data.split("\n\n");
    for (idx, monkey) in parts.enumerate() {
        let (func, test, pass) = parse_monkey(monkey, &mut items);
        operations[idx] = func;
        tests[idx] = test;
        next[idx] = pass;
    }
    (items, operations, tests, next)
}

fn parse_monkey(data: &'static str, items: &mut Items) -> (&'static str, u64, (usize, usize)) {
    let mut monkey: usize = 0;
    let mut operation: &str = "";
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
                operation = op.trim();
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
