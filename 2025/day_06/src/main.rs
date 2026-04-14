#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let mut v: Vec<Vec<_>> = data
        .lines()
        .map(|l| l.split_ascii_whitespace().collect())
        .collect();
    let operators = v.pop().unwrap();
    let mut result = 0;

    for (i, &o) in operators.iter().enumerate() {
        let nums = v.iter().map(|inner| inner[i].parse::<usize>().unwrap());
        result += match o {
            "+" => nums.sum(),
            "*" => nums.product(),
            _ => 0,
        }
    }
    result
}

fn part_two(data: &str) -> usize {
    let mut v: Vec<_> = data.lines().collect();
    let operators = v.pop().unwrap();
    let mut result = 0;
    let mut int = 0;
    let mut add = true;

    for (i, o) in operators.chars().enumerate() {
        match o {
            '+' => {
                add = true;
                int = 0;
            }
            '*' => {
                add = false;
                int = 1;
            }
            ' ' => {}
            _ => unreachable!(),
        }
        let num = v
            .iter()
            .map(|inner| inner.chars().nth(i).unwrap_or(' '))
            .fold(0, |acc, x| match x {
                ' ' => acc,
                val => acc * 10 + usize::try_from(val.to_digit(10).unwrap()).unwrap(),
            });
        match (num, add) {
            (0, _) => {
                result += int;
            }
            (val, true) => int += val,
            (val, false) => int *= val,
        }
    }
    result + int
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(4_277_556, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(3_263_827, part_two(data));
    }
}
