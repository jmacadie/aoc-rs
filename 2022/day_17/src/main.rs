use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &'static str) -> usize {
    let mut b = Board::new(data);
    for (num, &block) in BLOCKS.iter().cycle().enumerate() {
        if num == 2022 {
            return b.height;
        }
        b.drop_block(block);
    }
    0
}

fn part_two(data: &'static str) -> usize {
    const TARGET: usize = 1_000_000_000_000;

    let mut states = States::new();
    let mut start_cycles: usize = 0;
    let mut rep_cycles: usize = 1;
    let mut rep_height: usize = 0;
    let mut skip: usize = 0;

    let mut b = Board::new(data);
    for (num, &block) in BLOCKS.iter().cycle().enumerate() {
        if b.jet_index < 20 {
            states.add(b.jet_index, block, num, b.height);
            if let Some((cycles, height)) = states.check() {
                start_cycles = num;
                rep_cycles = cycles;
                rep_height = height;
                skip = num % BLOCKS.len();
                break;
            }
        }
        b.drop_block(block);
    }

    let residual_target = TARGET - start_cycles;
    let reps = residual_target / rep_cycles;
    let last_cycles = residual_target % rep_cycles;

    for (num, &block) in BLOCKS.iter().cycle().skip(skip).enumerate() {
        if num == last_cycles {
            return b.height + reps * rep_height;
        }
        b.drop_block(block);
    }
    0
}

struct Board {
    height: usize,
    blocks: [u8; 1_000],
    top: usize,
    jets: &'static [u8],
    jet_index: usize,
}

#[derive(Debug, Clone, Copy)]
struct State {
    instruction_index: usize,
    next: Block,
    number: usize,
    height: usize,
}

impl State {
    fn default() -> Self {
        Self {
            instruction_index: 0,
            next: Block::default(),
            number: 0,
            height: 0,
        }
    }

    fn new(instruction_index: usize, next: Block, number: usize, height: usize) -> Self {
        Self {
            instruction_index,
            number,
            next,
            height,
        }
    }

    fn same(&self, previous: &State) -> bool {
        self.instruction_index == previous.instruction_index && self.next == previous.next
    }

    fn diff(&self, previous: &Self) -> (usize, usize) {
        (self.number - previous.number, self.height - previous.height)
    }
}

struct States {
    data: [State; 100],
    size: usize,
}

impl States {
    fn new() -> Self {
        Self {
            data: [State::default(); 100],
            size: 0,
        }
    }

    fn add(&mut self, instruction_index: usize, next: Block, number: usize, height: usize) {
        self.data[self.size] = State::new(instruction_index, next, number, height);
        self.size += 1;
    }

    fn check(&self) -> Option<(usize, usize)> {
        if self.size < 2 {
            return None;
        }
        let test = self.data[self.size - 1];
        for counter in 0..self.size - 1 {
            let previous = self.data[counter];
            if test.same(&previous) {
                return Some(test.diff(&previous));
            }
        }
        None
    }
}

type Block = [u8; 4];

const HORIZONTAL: Block = [0b11110, 0, 0, 0];
const CROSS: Block = [0b01000, 0b11100, 0b01000, 0];
const REV_L: Block = [0b11100, 0b00100, 0b00100, 0];
const VERTICAL: Block = [0b10000, 0b10000, 0b10000, 0b10000];
const SQUARE: Block = [0b11000, 0b11000, 0, 0];

const BLOCKS: [Block; 5] = [HORIZONTAL, CROSS, REV_L, VERTICAL, SQUARE];
const MASKS: [u8; 7] = [0b1000000, 0b100000, 0b10000, 0b1000, 0b100, 0b10, 0b1];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

trait TetrisRow: Sized {
    fn fits(&self, space: Self) -> bool;
    fn shift_right(&self) -> Option<Self>;
    fn shift_left(&self) -> Option<Self>;
    fn add(&self, space: Self) -> Self;
}

impl TetrisRow for u8 {
    fn fits(&self, space: u8) -> bool {
        self & space == 0
    }

    fn shift_right(&self) -> Option<Self> {
        if 1 & self == 1 {
            return None;
        }
        Some(self >> 1)
    }

    fn shift_left(&self) -> Option<Self> {
        if 0b01000000 & self == 0b01000000 {
            return None;
        }
        Some(self << 1)
    }

    fn add(&self, space: Self) -> Self {
        self | space
    }
}

trait TetrisBlock {
    fn fits(&self, space: &[u8]) -> bool;
    fn shift_right(&self) -> Self;
    fn shift_left(&self) -> Self;
    fn add(&self, space: &mut [u8]);
}

impl TetrisBlock for Block {
    fn fits(&self, space: &[u8]) -> bool {
        for (&block_row, &row) in self.iter().zip(space.iter()) {
            if !block_row.fits(row) {
                return false;
            }
        }
        true
    }

    fn shift_right(&self) -> Self {
        let mut out = [0; 4];
        for (source, new) in self.iter().zip(out.iter_mut()) {
            if let Some(row) = source.shift_right() {
                *new = row;
            } else {
                return *self;
            }
        }
        out
    }

    fn shift_left(&self) -> Self {
        let mut out = [0; 4];
        for (source, new) in self.iter().zip(out.iter_mut()) {
            if let Some(row) = source.shift_left() {
                *new = row;
            } else {
                return *self;
            }
        }
        out
    }

    fn add(&self, space: &mut [u8]) {
        for (row, &block) in space.iter_mut().zip(self.iter()) {
            *row = row.add(block);
        }
    }
}

impl Board {
    fn new(instructions: &'static str) -> Self {
        let mut blocks = [0; 1_000];
        blocks[0] = 0b1111111;
        Board {
            height: 0,
            blocks,
            top: 0,
            jets: instructions.trim_end().as_bytes(),
            jet_index: 0,
        }
    }

    fn drop_block(&mut self, block: Block) {
        let next = self.first_jets(block);
        self.step_and_jet(next);
        self.update_top();
        self.clean_blocks();
    }

    fn first_jets(&mut self, block: Block) -> Block {
        let mut out = block;
        for _ in 0..4 {
            match self.next_direction() {
                Direction::Right => out = out.shift_right(),
                Direction::Left => out = out.shift_left(),
            }
        }
        out
    }

    fn step_and_jet(&mut self, mut block: Block) {
        let mut row = self.top;
        while block.fits(&self.blocks[row..row + 4]) {
            let next = match self.next_direction() {
                Direction::Right => block.shift_right(),
                Direction::Left => block.shift_left(),
            };
            if next.fits(&self.blocks[row..row + 4]) {
                block = next;
            }
            row -= 1;
        }
        block.add(&mut self.blocks[row + 1..row + 5]);
    }

    fn update_top(&mut self) {
        let temp = self.top;
        while self.blocks[self.top + 1] != 0 {
            self.top += 1;
        }
        self.height += self.top - temp;
    }

    fn clean_blocks(&mut self) {
        const SHIFT: usize = 50;

        if self.top < 995 {
            return;
        }
        let shift = self.top - SHIFT;
        self.top = SHIFT;
        for i in 0..SHIFT + 1 {
            self.blocks[i] = self.blocks[i + shift];
        }
        for i in SHIFT + 1..1_000 {
            self.blocks[i] = 0;
        }
    }

    fn next_direction(&mut self) -> Direction {
        if self.first_index() {
            self.jet_index = 0;
        }
        self.jet_index += 1;
        match self.jets[self.jet_index - 1] {
            b'>' => Direction::Right,
            b'<' => Direction::Left,
            _ => unreachable!(),
        }
    }

    fn first_index(&self) -> bool {
        self.jet_index == 0 || self.jet_index == self.jets.len()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..=self.top).rev() {
            write!(f, "|")?;
            for mask in MASKS {
                if mask & self.blocks[row] == 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(3068, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(1_514_285_714_288, part_two(data));
    }
}
