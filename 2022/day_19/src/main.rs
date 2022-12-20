pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    /*for line in data.lines() {
        let bp = read_line(line);
        let out = optimise(bp, &[], 0, State::default());
        println!("{out:?}");
    }*/
    data.lines()
        .map(read_line)
        .map(|bp| (bp, optimise(bp, &[], 0, State::default())))
        .map(|(bp, val)| usize::from(bp.number) * usize::from(val))
        .sum()
}

fn part_two(_data: &str) -> usize {
    0
}

fn optimise(bp: Blueprint, picks: &[Robots], mut best: u8, state: State) -> u8 {
    let mut curr = picks.to_vec();

    for robot in [Robots::Geode, Robots::Obsidian, Robots::Clay, Robots::Ore] {
        curr.push(robot);
        if let Some((val, new_state)) = run(bp, &curr, state) {
            if val > best {
                best = val;
            }
            let time = 24 - new_state.min;
            if time > 15 || new_state.geode + (time * (time + 1)) / 2 > best {
                best = optimise(bp, &curr, best, new_state);
            }
        }
        curr.pop();
    }

    best
}

fn run(bp: Blueprint, picks: &[Robots], mut state: State) -> Option<(u8, State)> {
    let mut num =
        (state.ore_robot + state.clay_robot + state.obsidian_robot + state.geode_robot - 1).into();
    let mut making;
    let mut save_state = State::default();
    let mut pick = get_next_pick(picks, num);
    let mut cost = get_pick_cost(pick, bp);

    while state.min < 24 {
        making = have_resources(cost, state.ore, state.clay, state.obsidian);

        state.ore += state.ore_robot;
        state.clay += state.clay_robot;
        state.obsidian += state.obsidian_robot;
        state.geode += state.geode_robot;
        state.min += 1;

        if making && state.min < 23 {
            match pick {
                Some(Robots::Ore) => state.ore_robot += 1,
                Some(Robots::Clay) => state.clay_robot += 1,
                Some(Robots::Obsidian) => state.obsidian_robot += 1,
                Some(Robots::Geode) => state.geode_robot += 1,
                None => unreachable!(),
            }
            state.ore -= cost.unwrap().ore;
            state.clay -= cost.unwrap().clay;
            state.obsidian -= cost.unwrap().obsidian;
            num += 1;
            pick = get_next_pick(picks, num);
            cost = get_pick_cost(pick, bp);
            if pick.is_none() {
                save_state = state;
            }
        }
    }
    if num >= picks.len() {
        return Some((state.geode, save_state));
    }
    None
}

fn have_resources(cost: Option<RobotCost>, ore: u8, clay: u8, obsidian: u8) -> bool {
    if let Some(cost) = cost {
        return ore >= cost.ore && clay >= cost.clay && obsidian >= cost.obsidian;
    }
    false
}

fn get_next_pick(picks: &[Robots], num: usize) -> Option<Robots> {
    picks.get(num).copied()
}

fn get_pick_cost(pick: Option<Robots>, bp: Blueprint) -> Option<RobotCost> {
    match pick? {
        Robots::Ore => Some(bp.ore_robot),
        Robots::Clay => Some(bp.clay_robot),
        Robots::Obsidian => Some(bp.obsidian_robot),
        Robots::Geode => Some(bp.geode_robot),
    }
}

fn read_line(line: &str) -> Blueprint {
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
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    number: u8,
    ore_robot: RobotCost,
    clay_robot: RobotCost,
    obsidian_robot: RobotCost,
    geode_robot: RobotCost,
}

#[derive(Debug, Clone, Copy)]
struct RobotCost {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

#[derive(Debug, Clone, Copy)]
enum Robots {
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
        assert_eq!(0, part_two(data));
    }
}
