use crate::common::file::read_lines;

const ROOT: &str = "src/year_2020/day_10/";

pub fn run() {
    let test = Adapters::new("test.txt");
    let main = Adapters::new("input.txt");

    run_part_one(&test, &main);
    run_part_two(&test, &main);
}

fn run_part_one(test: &Adapters, main: &Adapters) {
    assert_eq!(220, test.output());

    let val = main.output();
    println!("Part 1: 1 jolt steps * 3 jolt steps = {}", val);
}

fn run_part_two(test: &Adapters, main: &Adapters) {
    assert_eq!(19_208, test.adapter_permutations());

    println!(
        "Part 2: {} adapter_permutations",
        main.adapter_permutations()
    );
}

struct Adapters {
    list: Vec<i32>,
}

impl Adapters {
    fn new(filename: &str) -> Self {
        let file = format!("{}{}", ROOT, filename);
        let lines = read_lines(file).unwrap();
        let mut list = Vec::new();
        for line in lines.flatten() {
            list.push(line.parse().unwrap());
        }
        list.sort_unstable();
        list.push(list.last().unwrap() + 3);
        Adapters { list }
    }

    fn adapter_permutations(&self) -> u64 {
        let groups = self.get_step_groups();
        groups
            .iter()
            .filter(|(a, _)| a == &1)
            .map(|(_, b)| Self::group_permutations(*b))
            .product()
    }

    fn group_permutations(num: i32) -> u64 {
        match num {
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            5 => 13,
            6 => 26,
            _ => unreachable!(),
        }
    }

    fn get_step_groups(&self) -> Vec<(i32, i32)> {
        let mut steps = Vec::new();
        let mut last = 0;
        let mut curr_group = (0, 0);
        for next in &self.list {
            let step = next - last;
            if step == curr_group.0 {
                curr_group.1 += 1;
            } else {
                steps.push(curr_group);
                curr_group = (step, 1);
            }
            last = *next;
        }
        steps.push(curr_group);
        steps
    }

    fn get_steps(&self) -> Vec<i32> {
        let mut steps = vec![0, 0, 0];
        let mut last = 0;
        for next in &self.list {
            let step: usize = (next - last - 1).try_into().unwrap();
            steps[step] += 1;
            last = *next;
        }
        steps
    }

    fn output(&self) -> i32 {
        let steps = self.get_steps();
        steps[0] * steps[2]
    }
}
