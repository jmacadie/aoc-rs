#![warn(clippy::all, clippy::pedantic)]
use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data, 100));
    println!("Part 2: {}", part_two(data, 100));
}

fn part_one(data: &str, hp: u8) -> u8 {
    let boss = get_boss(data);
    min_stats_to_win(hp, boss)
        .into_iter()
        .map(|(damage, armour)| min_cost(damage, armour))
        .min()
        .unwrap()
}

fn part_two(data: &str, hp: u8) -> u8 {
    let boss = get_boss(data);
    min_stats_to_win(hp, boss)
        .into_iter()
        .filter(|&(_, armour)| armour > 0)
        .map(|(damage, armour)| max_cost(damage, armour - 1))
        .max()
        .unwrap()
}

type Item = [u8; 3];

const WEAPONS: [Item; 5] = [
    [8, 4, 0],  // Dagger
    [10, 5, 0], // Shortsword
    [25, 6, 0], // Warhammer
    [40, 7, 0], // Longsword
    [74, 8, 0], // Greataxe
];

const ARMOUR: [Item; 5] = [
    [13, 0, 1],  // Leather
    [31, 0, 2],  // Chainmail
    [53, 0, 3],  // Splintmail
    [75, 0, 4],  // Bandemail
    [102, 0, 5], // Platemail
];

const RINGS: [Item; 6] = [
    [25, 1, 0],  // Damage +1
    [50, 2, 0],  // Damage +2
    [100, 3, 0], // Damage +3
    [20, 0, 1],  // Defence +1
    [40, 0, 2],  // Defence +2
    [80, 0, 3],  // Defence +2
];

fn min_cost(target_damage: u8, target_armour: u8) -> u8 {
    cost_iterator(target_damage, target_armour).min().unwrap()
}

fn max_cost(target_damage: u8, target_armour: u8) -> u8 {
    cost_iterator(target_damage, target_armour).max().unwrap()
}

fn cost_iterator(target_damage: u8, target_armour: u8) -> impl Iterator<Item = u8> {
    let armour_with_none = ARMOUR.into_iter().chain(std::iter::once(Item::default()));
    let rings_with_none = RINGS
        .into_iter()
        .chain(std::iter::once(Item::default()))
        .chain(std::iter::once(Item::default()));
    WEAPONS
        .into_iter()
        .cartesian_product(armour_with_none)
        .cartesian_product(rings_with_none.combinations(2))
        .filter(move |((weapon, armour), rings)| {
            let comb_dam = weapon[1] + rings[0][1] + rings[1][1];
            let comb_arm = armour[2] + rings[0][2] + rings[1][2];
            comb_dam == target_damage && comb_arm == target_armour
        })
        .map(|((weapon, armour), rings)| weapon[0] + armour[0] + rings[0][0] + rings[1][0])
}

fn get_boss(data: &str) -> (u8, u8, u8) {
    let mut lines = data.lines();
    let hp = get_stat(lines.next(), "Hit Points: ");
    let dam = get_stat(lines.next(), "Damage: ");
    let arm = get_stat(lines.next(), "Armor: ");
    (hp, dam, arm)
}

fn get_stat(line: Option<&str>, prefix: &'static str) -> u8 {
    line.unwrap().strip_prefix(prefix).unwrap().parse().unwrap()
}

fn turns_to_kill(hp: u8, damage: u8, armour: u8) -> u8 {
    let dph = if armour >= damage { 1 } else { damage - armour };
    if hp % dph == 0 {
        hp / dph
    } else {
        hp / dph + 1
    }
}

fn min_armour_to_win(you: (u8, u8), boss: (u8, u8, u8)) -> u8 {
    let (b_hp, b_dam, b_arm) = boss;
    let (hp, dam) = you;
    let you_kill_boss_in = turns_to_kill(b_hp, dam, b_arm);
    (0..)
        .into_iter()
        .find(|&arm| turns_to_kill(hp, b_dam, arm) >= you_kill_boss_in)
        .unwrap()
}

fn min_stats_to_win(you: u8, boss: (u8, u8, u8)) -> [(u8, u8); 10] {
    let mut min_stats = [(0, 0); 10];
    for ((min_dam, min_arm), dam) in min_stats.iter_mut().zip((4..=13).into_iter()) {
        *min_dam = dam;
        *min_arm = min_armour_to_win((you, dam), boss);
    }
    min_stats
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data, 8));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data, 8));
    }

    #[test]
    fn turns() {
        assert_eq!(13, turns_to_kill(100, 8, 0));
        assert_eq!(15, turns_to_kill(104, 8, 1));
    }
}
