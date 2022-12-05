use stacks::{CraneMove, Stacks};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data)?);
    println!("Part 2: {}", part_two(data)?);

    Ok(())
}

fn part_one(data: &str) -> color_eyre::Result<String> {
    let mut data = data.split("\n\n");
    let (crates, moves) = (data.next().unwrap(), data.next().unwrap());
    let mut crates = Stacks::new(crates);
    for step in moves.lines() {
        let step = step.parse::<CraneMove>()?;
        crates.crane_move_one(step);
    }
    Ok(crates.get_heads())
}

fn part_two(data: &str) -> color_eyre::Result<String> {
    let mut data = data.split("\n\n");
    let (crates, moves) = (data.next().unwrap(), data.next().unwrap());
    let mut crates = Stacks::new(crates);
    for step in moves.lines() {
        let step = step.parse::<CraneMove>()?;
        crates.crane_move_all(step);
    }
    Ok(crates.get_heads())
}

mod stacks {
    use itertools::Itertools;
    use std::str::FromStr;

    type Boxes = [[u8; 60]; 9];
    type Heads = [usize; 9];

    pub(crate) struct Stacks {
        boxes: Boxes,
        heads: Heads,
    }

    impl Stacks {
        pub(crate) fn new(input: &str) -> Self {
            let mut boxes = [[0_u8; 60]; 9];
            Self::init_load(&mut boxes, input);
            let heads = Self::find_heads(&boxes);
            Self { boxes, heads }
        }

        pub(crate) fn crane_move_one(&mut self, data: CraneMove) {
            let mut moves = data.move_num;

            while moves > 0 {
                self.move_top_box(data.from, data.to);
                moves -= 1;
            }
        }

        pub(crate) fn crane_move_all(&mut self, data: CraneMove) {
            let mut moves = data.move_num;
            let mut from_row = self.heads[data.from] - moves;

            while moves > 0 {
                self.move_one_box(data.from, from_row, data.to);
                moves -= 1;
                from_row += 1;
            }
            self.heads[data.from] -= data.move_num;
        }

        pub(crate) fn get_heads(&self) -> String {
            let mut out = String::with_capacity(9);
            for pile in 0..9 {
                if self.heads[pile] == 0 {
                    out.push(' ');
                } else {
                    out.push(self.boxes[pile][self.heads[pile] - 1] as char);
                }
            }
            out.trim().to_owned()
        }

        fn move_top_box(&mut self, from: usize, to: usize) {
            self.move_one_box(from, self.heads[from] - 1, to);
            self.heads[from] -= 1;
        }

        fn move_one_box(&mut self, from: usize, from_row: usize, to: usize) {
            self.boxes[to][self.heads[to]] = self.boxes[from][from_row];
            self.boxes[from][from_row] = 0;
            self.heads[to] += 1;
        }

        fn init_load(boxes: &mut Boxes, input: &str) {
            for (row, line) in input.lines().rev().skip(1).enumerate() {
                for (pile, mut part) in line.bytes().chunks(4).into_iter().enumerate() {
                    boxes[pile][row] = match part.nth(1) {
                        Some(value @ b'A'..=b'Z') => value,
                        Some(b' ') => 0_u8,
                        _ => unreachable!(),
                    };
                }
            }
        }

        fn find_heads(boxes: &Boxes) -> Heads {
            let mut heads = Heads::default();
            let mut row: usize;
            for pile in 0..9 {
                row = 0;
                while boxes[pile][row] != 0 {
                    row += 1;
                }
                heads[pile] = row;
            }
            heads
        }
    }

    pub(crate) struct CraneMove {
        move_num: usize,
        from: usize,
        to: usize,
    }

    impl FromStr for CraneMove {
        type Err = color_eyre::Report;

        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let mut parts = line.split(' ');
            let (Some("move"), Some(m), Some("from"), Some(f), Some("to"), Some (t), None) = (parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) else {
                return Err(color_eyre::eyre::eyre!("expected 'move x from y to z', got '{line}'"));
            };

            let move_num = m
                .parse::<usize>()
                .map_err(|_| color_eyre::eyre::eyre!("could not convert move {m} into a usize"))?;
            let from = f
                .parse::<usize>()
                .map_err(|_| color_eyre::eyre::eyre!("could not convert from {f} into a usize"))?;
            let to = t
                .parse::<usize>()
                .map_err(|_| color_eyre::eyre::eyre!("could not convert to {t} into a usize"))?;

            Ok(Self {
                move_num,
                from: from - 1,
                to: to - 1,
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!("CMZ", part_one(data).unwrap());
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!("MCD", part_two(data).unwrap());
    }
}
