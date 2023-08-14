#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let pipes = data
        .lines()
        .map(|l| l.split_once(" <-> ").unwrap().1)
        .collect::<Vec<&str>>();
    let mut tracker = FoundHash::new();
    expand_group(0, &mut tracker, &pipes);
    tracker.count
}

fn part_two(data: &str) -> usize {
    let pipes = data
        .lines()
        .map(|l| l.split_once(" <-> ").unwrap().1)
        .collect::<Vec<&str>>();
    let mut tracker = FoundHash::new();
    let mut last = 0;
    let mut group_count = 0;
    while let Some(val) = tracker.next_not_found(last) {
        expand_group(val, &mut tracker, &pipes);
        group_count += 1;
        last = val;
    }
    group_count
}

fn expand_group(from: usize, tracker: &mut FoundHash, pipes: &[&str]) {
    for to in pipes[from]
        .split(", ")
        .map(|val| val.parse::<usize>().unwrap())
    {
        if tracker.is_found(to) {
            continue;
        }
        tracker.mark_found(to);
        expand_group(to, tracker, pipes);
    }
}

struct FoundHash {
    data: [u128; 20],
    count: usize,
}

impl FoundHash {
    const fn new() -> Self {
        Self {
            data: [0; 20],
            count: 0,
        }
    }

    const fn is_found(&self, val: usize) -> bool {
        let col = val / 100;
        let posn = val % 100;
        self.data[col] >> posn & 1 == 1
    }

    fn mark_found(&mut self, val: usize) {
        let col = val / 100;
        let posn = val % 100;
        self.data[col] |= 1 << posn;
        self.count += 1;
    }

    fn next_not_found(&self, start: usize) -> Option<usize> {
        let start_col = start / 100;
        let start_posn = start % 100;
        let mut temp = self.data[start_col] >> start_posn;

        for posn in start_posn..100 {
            if temp & 1 == 0 {
                return Some(start_col * 100 + posn);
            }
            temp >>= 1;
        }

        for col in start_col + 1..20 {
            temp = self.data[col];
            for posn in 0..100 {
                if temp & 1 == 0 {
                    return Some(col * 100 + posn);
                }
                temp >>= 1;
            }
        }
        None
    }
}
