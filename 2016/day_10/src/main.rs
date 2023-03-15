#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    let mut f = Factory::new(data);
    f.run(data);
    println!("Part 1: {}", part_one(&f, 61, 17));
    println!("Part 2: {}", part_two(&f));
}

fn part_one(f: &Factory, chip1: usize, chip2: usize) -> usize {
    let bot = f.get_comparing_bot(chip1.into(), chip2.into()).unwrap();
    bot.id.0
}

fn part_two(f: &Factory) -> usize {
    f.outputs
        .iter()
        .take(3)
        .map(|&chip| chip.unwrap().0)
        .product()
}

#[derive(Debug, Clone, Copy)]
struct Factory {
    robots: [Robot; 210],
    outputs: [Option<Microchip>; 21],
    processing_stack: [RobotNum; 10],
    processing_count: usize,
}

impl Factory {
    fn new(instructions: &str) -> Self {
        let mut robots = std::array::from_fn(|i| Robot::new(i.into()));
        for line in instructions.lines() {
            let (first, rest) = line.split_once(' ').unwrap();
            match first {
                "bot" => {
                    let (bot_num, rest) = rest.split_once(' ').unwrap();
                    let rest = rest.trim_start_matches("gives low to ");
                    let (low, rest) = rest.split_once(" and ").unwrap();
                    let high = rest.trim_start_matches("high to ");
                    let bot_num: usize = bot_num.parse().unwrap();

                    let robot = robots.get_mut(bot_num).unwrap();
                    robot.high = high.parse().unwrap();
                    robot.low = low.parse().unwrap();
                }
                // Initial microchips being handed out
                // Ignore as we'll allocate these in a second pass
                "value" => (),
                _ => unreachable!(),
            }
        }
        Self {
            robots,
            outputs: [None; 21],
            processing_stack: [0.into(); 10],
            processing_count: 0,
        }
    }

    fn give_initial(&mut self, instuctions: &str) {
        for line in instuctions.lines() {
            let (first, rest) = line.split_once(' ').unwrap();
            match first {
                "bot" => (), // initial passing rules: ignore as already done
                "value" => {
                    let (chip, rest) = rest.split_once(' ').unwrap();
                    let bot = rest.trim_start_matches("goes to ");

                    let chip: Microchip = chip.parse::<usize>().unwrap().into();
                    let bot: Destination = bot.parse().unwrap();

                    self.pass_microchip(chip, bot);
                }
                _ => unreachable!(),
            }
        }
    }

    fn pass_microchip(&mut self, chip: Microchip, dest: Destination) {
        match dest {
            Destination::Robot(n) => {
                //let bot = self.robots.get_mut(n.0).unwrap();
                if self.robots[n.0].recieve_microchip(chip) {
                    self.processing_stack[self.processing_count] = n;
                    self.processing_count += 1;
                }
            }
            Destination::Output(n) => {
                self.outputs[n.0] = Some(chip);
            }
        }
    }

    fn run_iteration(&mut self) {
        // Grab & then reset unprocessed counter down to zero
        let count = self.processing_count;
        self.processing_count = 0;

        for num in 0..count {
            let bot_num = self.processing_stack[num];
            let bot = self.robots[bot_num.0];

            let high_chip = bot.high().unwrap();
            let high_dest = bot.high;
            self.pass_microchip(high_chip, high_dest);

            let low_chip = bot.low().unwrap();
            let low_dest = bot.low;
            self.pass_microchip(low_chip, low_dest);
        }
    }

    fn run(&mut self, data: &str) {
        self.give_initial(data);

        while self.processing_count != 0 {
            self.run_iteration();
        }
    }

    fn get_comparing_bot(&self, chip1: Microchip, chip2: Microchip) -> Option<Robot> {
        self.robots
            .iter()
            .find(|&bot| bot.compares(chip1) && bot.compares(chip2))
            .copied()
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    id: RobotNum,
    microchips: [Option<Microchip>; 2],
    high: Destination,
    low: Destination,
}

impl Robot {
    fn new(id: RobotNum) -> Self {
        Self {
            id,
            microchips: [None, None],
            high: Destination::Output(0.into()),
            low: Destination::Output(0.into()),
        }
    }

    fn recieve_microchip(&mut self, chip: Microchip) -> bool {
        if self.microchips[0].is_none() {
            self.microchips[0] = Some(chip);
            false
        } else {
            self.microchips[1] = Some(chip);
            true
        }
    }

    fn high(&self) -> Option<Microchip> {
        self.microchips[1]
            .and_then(|m1| self.microchips[0].map(|m0| (m0, m1)))
            .map(|(m0, m1)| std::cmp::max(m0, m1))
    }

    fn low(&self) -> Option<Microchip> {
        self.microchips[1]
            .and_then(|m1| self.microchips[0].map(|m0| (m0, m1)))
            .map(|(m0, m1)| std::cmp::min(m0, m1))
    }

    fn compares(&self, chip: Microchip) -> bool {
        self.microchips[0] == Some(chip) || self.microchips[1] == Some(chip)
    }
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Robot(RobotNum),
    Output(OutputBin),
}

impl FromStr for Destination {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((destination_type, num)) = s.split_once(' ') else {
            return Err("bad format".into());
        };
        let num: usize = num.parse()?;
        match destination_type {
            "output" => Ok(Self::Output(num.into())),
            "bot" => Ok(Self::Robot(num.into())),
            _ => Err("bad format".into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Microchip(usize);
impl From<usize> for Microchip {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OutputBin(usize);
impl From<usize> for OutputBin {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RobotNum(usize);
impl From<usize> for RobotNum {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let mut f = Factory::new(data);
        f.run(data);
        assert_eq!(2, part_one(&f, 5, 2));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let mut f = Factory::new(data);
        f.run(data);
        assert_eq!(30, part_two(&f));
    }
}
