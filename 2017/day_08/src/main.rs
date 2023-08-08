#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::HashMap, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    let (reg, max) = genenerate_registers(data);
    println!("Part 1: {}", part_one(&reg));
    println!("Part 2: {}", part_two(max));
}

fn part_one(registers: &Registers) -> i32 {
    *registers.values().max().unwrap()
}

const fn part_two(max: i32) -> i32 {
    max
}

fn genenerate_registers(data: &str) -> (Registers<'_>, i32) {
    let mut registers = Registers::new();
    let mut max_val = 0_i32;
    for line in data.lines() {
        let instr = Instruction::from_str(line).unwrap();
        if condition_met(instr.condition, &registers) {
            let val = registers
                .entry(instr.register)
                .and_modify(|v| *v += instr.amount)
                .or_insert(instr.amount);
            max_val = std::cmp::max(*val, max_val);
        }
    }
    (registers, max_val)
}

fn condition_met(condition: Condition, registers: &Registers) -> bool {
    let &val = registers.get(condition.register).unwrap_or(&0);
    match condition.op {
        Operation::GreaterThan => val > condition.value,
        Operation::GreaterOrEqual => val >= condition.value,
        Operation::Equal => val == condition.value,
        Operation::NotEqual => val != condition.value,
        Operation::LessOrEqual => val <= condition.value,
        Operation::LessThan => val < condition.value,
    }
}

type Registers<'a> = HashMap<&'a str, i32>;

#[derive(Clone, Copy, Debug)]
struct Instruction<'a> {
    register: &'a str,
    amount: i32,
    condition: Condition<'a>,
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a str) -> Result<Self, String> {
        let (first, condition) = s
            .split_once(" if ")
            .ok_or(format!("'{s}' is expceted to be bisected by an 'if'"))?;
        let (register, amount) = first.split_once(' ').ok_or(format!(
            "'{first}' should be of the format 'register' 'operation'"
        ))?;
        let (direction, value) = amount
            .split_once(' ')
            .ok_or_else(|| "'amount' should be of the format 'inc/dec' 'amount'".to_string())?;
        let value: i32 = value
            .parse()
            .map_err(|_| format!("'{value}' cannot be parsed into an i32"))?;
        let amount = match direction {
            "inc" => value,
            "dec" => -value,
            _ => return Err(format!("'{direction}' is not of the form 'inc' or 'dec'")),
        };
        let condition = Condition::from_string(condition)?;
        Ok(Self {
            register,
            amount,
            condition,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Condition<'a> {
    register: &'a str,
    op: Operation,
    value: i32,
}

impl<'a> Condition<'a> {
    fn from_string(s: &'a str) -> Result<Self, String> {
        let mut parts = s.split_ascii_whitespace();
        let (Some(register), Some(op), Some(value), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else
            { return Err(format!("'{s}' is not in the expected format of 'register' 'op' 'value'")) };
        let op = op.parse()?;
        let value = value
            .parse()
            .map_err(|_| format!("{value} cannot be parsed into an i32"))?;
        Ok(Self {
            register,
            op,
            value,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    GreaterThan,
    GreaterOrEqual,
    Equal,
    NotEqual,
    LessOrEqual,
    LessThan,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::GreaterThan),
            ">=" => Ok(Self::GreaterOrEqual),
            "==" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            "<=" => Ok(Self::LessOrEqual),
            "<" => Ok(Self::LessThan),
            _ => Err(format!("'{s}' is not a comparison operator")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (reg, _max) = genenerate_registers(data);
        assert_eq!(1, part_one(&reg));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (_reg, max) = genenerate_registers(data);
        assert_eq!(10, part_two(max));
    }
}
