#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.trim().split(',').map(hash).sum()
}

fn part_two(data: &str) -> usize {
    let mut fac = Facility::new(data);
    fac.run();
    fac.focussing_power()
}

struct Facility<'a> {
    boxes: [Box<'a>; 256],
    instructions: Vec<Instruction<'a>>,
}

impl<'a> Facility<'a> {
    fn new(data: &'a str) -> Self {
        let boxes = std::array::from_fn(|i| Box::new(i + 1));
        let mut instructions = Vec::new();
        instructions.extend(data.trim().split(',').map(Instruction::new));
        Self {
            boxes,
            instructions,
        }
    }

    fn run(&mut self) {
        self.instructions.iter().for_each(|i| match i.operation {
            Op::Add => self.boxes[i.box_num].add(i.label, i.lens.unwrap()),
            Op::Remove => self.boxes[i.box_num].remove(i.label),
        });
    }

    fn focussing_power(&self) -> usize {
        self.boxes.iter().map(Box::focussing_power).sum()
    }
}

struct Instruction<'a> {
    label: &'a str,
    box_num: usize,
    operation: Op,
    lens: Option<usize>,
}

impl<'a> Instruction<'a> {
    fn new(instruction: &'a str) -> Self {
        if let Some((label, lens)) = instruction.split_once('=') {
            let lens = lens.parse::<usize>().unwrap();
            let box_num = hash(label);
            Self {
                label,
                box_num,
                operation: Op::Add,
                lens: Some(lens),
            }
        } else {
            let label = instruction.trim_end_matches('-');
            let box_num = hash(label);
            Self {
                label,
                box_num,
                operation: Op::Remove,
                lens: None,
            }
        }
    }
}
enum Op {
    Add,
    Remove,
}

struct Box<'a> {
    number: usize,
    lenses: Vec<Lens<'a>>,
}

impl<'a> Box<'a> {
    const fn new(number: usize) -> Self {
        Self {
            number,
            lenses: Vec::new(),
        }
    }

    fn add(&mut self, label: &'a str, focal_length: usize) {
        if let Some(idx) = self.lenses.iter().position(|l| l.label == label) {
            self.lenses[idx].focal_length = focal_length;
        } else {
            self.lenses.push(Lens::new(label, focal_length));
        }
    }

    fn remove(&mut self, label: &'a str) {
        if let Some(idx) = self.lenses.iter().position(|l| l.label == label) {
            self.lenses.remove(idx);
        }
    }

    fn focussing_power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, l)| self.number * l.focal_length * (i + 1))
            .sum()
    }
}

struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

impl<'a> Lens<'a> {
    const fn new(label: &'a str, focal_length: usize) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, &inp| ((acc + usize::from(inp)) * 17) % 256)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(1320, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(145, part_two(data));
    }
}
