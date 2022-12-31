pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(Blueprint::new)
        .map(|bp| (bp, bp.optimise::<24>()))
        .map(|(bp, val)| usize::from(bp.number) * usize::from(val))
        .sum()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .take(3)
        .map(Blueprint::new)
        .map(|bp| usize::from(bp.optimise::<32>()))
        .product()
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    number: u8,
    ore_robot: RobotCost,
    clay_robot: RobotCost,
    obsidian_robot: RobotCost,
    geode_robot: RobotCost,
    ore_robot_lim: u8,
    clay_robot_lim: u8,
    obsidian_robot_lim: u8,
}

impl Blueprint {
    fn new(line: &str) -> Blueprint {
        let (number, rest) = line.split_once(':').unwrap();
        let number = number.trim_start_matches("Blueprint ").parse().unwrap();

        let mut parts = rest.split('.');

        let ore_robot = parts.next().unwrap().trim();
        let ore_robot = ore_robot
            .trim_start_matches("Each ore robot costs ")
            .trim_end_matches(" ore")
            .parse()
            .unwrap();

        let clay_robot = parts.next().unwrap().trim();
        let clay_robot = clay_robot
            .trim_start_matches("Each clay robot costs ")
            .trim_end_matches(" ore")
            .parse()
            .unwrap();

        let obsidian_robot = parts.next().unwrap().trim();
        let (obsidian_robot_ore, obsidian_robot_clay) = obsidian_robot
            .trim_start_matches("Each obsidian robot costs ")
            .split_once(" and ")
            .unwrap();
        let obsidian_robot_ore = obsidian_robot_ore.trim_end_matches(" ore").parse().unwrap();
        let obsidian_robot_clay = obsidian_robot_clay
            .trim_end_matches(" clay")
            .parse()
            .unwrap();

        let geode_robot = parts.next().unwrap().trim();
        let (geode_robot_ore, geode_robot_obsidian) = geode_robot
            .trim_start_matches("Each geode robot costs ")
            .split_once(" and ")
            .unwrap();
        let geode_robot_ore = geode_robot_ore.trim_end_matches(" ore").parse().unwrap();
        let geode_robot_obsidian = geode_robot_obsidian
            .trim_end_matches(" obsidian")
            .parse()
            .unwrap();

        let mut ore_robot_lim = std::cmp::max(ore_robot, clay_robot);
        ore_robot_lim = std::cmp::max(ore_robot_lim, obsidian_robot_ore);
        ore_robot_lim = std::cmp::max(ore_robot_lim, geode_robot_ore);

        Blueprint {
            number,
            ore_robot: RobotCost {
                ore: ore_robot,
                clay: 0,
                obsidian: 0,
            },
            clay_robot: RobotCost {
                ore: clay_robot,
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot: RobotCost {
                ore: obsidian_robot_ore,
                clay: obsidian_robot_clay,
                obsidian: 0,
            },
            geode_robot: RobotCost {
                ore: geode_robot_ore,
                clay: 0,
                obsidian: geode_robot_obsidian,
            },
            ore_robot_lim,
            clay_robot_lim: obsidian_robot_clay,
            obsidian_robot_lim: geode_robot_obsidian,
        }
    }

    fn optimise<const T: u8>(&self) -> u8 {
        self.optimise_inner::<T>(0, State::default())
    }

    fn optimise_inner<const T: u8>(&self, mut best: u8, state: State) -> u8 {
        for robot in [Robot::Geode, Robot::Obsidian, Robot::Clay, Robot::Ore] {
            if self.enough(robot, state) {
                continue;
            }
            if let Some((val, new_state)) = self.run::<T>(robot, state) {
                best = std::cmp::max(best, val);
                let rem = T - new_state.min;
                if rem > 15 || val + (rem * (rem + 1)) / 2 > best {
                    best = self.optimise_inner::<T>(best, new_state);
                }
            }
        }
        best
    }

    fn run<const T: u8>(&self, next: Robot, mut state: State) -> Option<(u8, State)> {
        let cost = self.get_pick_cost(next);
        if (cost.clay > 0 && state.clay_robot == 0)
            || (cost.obsidian > 0 && state.obsidian_robot == 0)
        {
            return None;
        }

        let mut wait = 0;
        if cost.ore > state.ore {
            let new = 1 + (cost.ore - state.ore - 1) / state.ore_robot;
            if state.min + new >= T {
                return None;
            }
            wait = std::cmp::max(wait, new);
        }
        if cost.clay > state.clay {
            let new = 1 + (cost.clay - state.clay - 1) / state.clay_robot;
            if state.min + new >= T {
                return None;
            }
            wait = std::cmp::max(wait, new);
        }
        if cost.obsidian > state.obsidian {
            let new = 1 + (cost.obsidian - state.obsidian - 1) / state.obsidian_robot;
            if state.min + new >= T {
                return None;
            }
            wait = std::cmp::max(wait, new);
        }
        state.wait(wait + 1);
        state.add(next, cost);

        Some((state.final_geodes::<T>(), state))
    }

    fn get_pick_cost(&self, pick: Robot) -> RobotCost {
        match pick {
            Robot::Ore => self.ore_robot,
            Robot::Clay => self.clay_robot,
            Robot::Obsidian => self.obsidian_robot,
            Robot::Geode => self.geode_robot,
        }
    }

    fn enough(&self, robot: Robot, state: State) -> bool {
        match robot {
            Robot::Ore => state.ore_robot >= self.ore_robot_lim,
            Robot::Clay => state.clay_robot >= self.clay_robot_lim,
            Robot::Obsidian => state.obsidian_robot >= self.obsidian_robot_lim,
            Robot::Geode => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RobotCost {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

#[derive(Debug, Clone, Copy)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct State {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: u8,
    geode_robot: u8,
    min: u8,
}

impl State {
    fn final_geodes<const T: u8>(&self) -> u8 {
        self.geode + (T - self.min) * self.geode_robot
    }

    fn wait(&mut self, mins: u8) {
        self.ore += self.ore_robot * mins;
        self.clay += self.clay_robot * mins;
        self.obsidian += self.obsidian_robot * mins;
        self.geode += self.geode_robot * mins;
        self.min += mins;
    }

    fn add(&mut self, robot: Robot, cost: RobotCost) {
        match robot {
            Robot::Ore => self.ore_robot += 1,
            Robot::Clay => self.clay_robot += 1,
            Robot::Obsidian => self.obsidian_robot += 1,
            Robot::Geode => self.geode_robot += 1,
        }
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            min: 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(33, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(56 * 62, part_two(data));
    }
}
