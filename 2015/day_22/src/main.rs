#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data, 50, 500));
    println!("Part 2: {}", part_two(data, 50, 500));
}

fn part_one(data: &str, hp: u8, mana: u32) -> u32 {
    let game = GameState::new(data, hp, mana, false);
    min_win(game, u32::MAX)
}

fn part_two(data: &str, hp: u8, mana: u32) -> u32 {
    let game = GameState::new(data, hp, mana, true);
    min_win(game, u32::MAX)
}

fn min_win(game: GameState, mut best: u32) -> u32 {
    let choices = [
        PlayerMove::MagicMissile,
        PlayerMove::Drain,
        PlayerMove::Shield,
        PlayerMove::Poison,
        PlayerMove::Recharge,
    ];
    for choice in choices {
        let mut next = game;
        next.turn(choice);
        match (next.result, next.mana_spent) {
            (_, m) if m >= best => (),
            (GameResult::Lost, _) => (),
            (GameResult::Won, m) => {
                return m;
            }
            (GameResult::Pending, _) => {
                best = min_win(next, best);
            }
        }
    }
    best
}

#[derive(Debug, Clone, Copy)]
struct GameState {
    mana_spent: u32,
    hp: u8,
    mana: u32,
    boss_hp: u8,
    boss_damage: u8,
    hard_mode: bool,
    shield_turns_remaining: u8,
    poison_turns_remaining: u8,
    recharge_turns_remaining: u8,
    result: GameResult,
}

impl GameState {
    fn new(data: &str, hp: u8, mana: u32, hard_mode: bool) -> Self {
        let (boss_hp, boss_damage) = read_input(data);
        Self {
            mana_spent: 0,
            hp,
            mana,
            boss_hp,
            boss_damage,
            hard_mode,
            shield_turns_remaining: 0,
            poison_turns_remaining: 0,
            recharge_turns_remaining: 0,
            result: GameResult::Pending,
        }
    }

    fn turn(&mut self, choice: PlayerMove) {
        if self.valid_move(choice) {
            self.process_hard_mode();
            self.process_effects();
            self.player_turn(choice);
            self.process_effects();
            self.boss_turn();
        }
    }

    fn valid_move(&mut self, choice: PlayerMove) -> bool {
        match choice {
            PlayerMove::Shield => {
                if self.shield_turns_remaining > 1 {
                    self.result = GameResult::Lost;
                    return false;
                }
            }
            PlayerMove::Poison => {
                if self.poison_turns_remaining > 1 {
                    self.result = GameResult::Lost;
                    return false;
                }
            }
            PlayerMove::Recharge => {
                if self.recharge_turns_remaining > 1 {
                    self.result = GameResult::Lost;
                    return false;
                }
            }
            _ => (),
        }
        true
    }

    fn boss_turn(&mut self) {
        if self.result != GameResult::Pending {
            return;
        }
        let damage = match (self.shield_turns_remaining, self.boss_damage) {
            (0, d) => d,
            (_, d) if d > 7 => d - 7,
            (_, _) => 1,
        };
        if damage >= self.hp {
            self.result = GameResult::Lost;
        } else {
            self.hp -= damage;
        }
    }

    fn player_turn(&mut self, choice: PlayerMove) {
        match choice {
            PlayerMove::MagicMissile => {
                self.spend_mana(53);
                self.damage_boss(4);
            }
            PlayerMove::Drain => {
                self.spend_mana(73);
                self.damage_boss(2);
                self.hp += 2;
            }
            PlayerMove::Shield => {
                self.spend_mana(113);
                self.shield_turns_remaining = 6;
            }
            PlayerMove::Poison => {
                self.spend_mana(173);
                self.poison_turns_remaining = 6;
            }
            PlayerMove::Recharge => {
                self.spend_mana(229);
                self.recharge_turns_remaining = 5;
            }
        }
    }

    fn process_hard_mode(&mut self) {
        if self.hard_mode {
            if self.hp <= 1 {
                self.result = GameResult::Lost;
            } else {
                self.hp -= 1;
            }
        }
    }

    fn process_effects(&mut self) {
        self.process_shield();
        self.process_poison();
        self.process_recharge();
    }

    fn process_shield(&mut self) {
        if self.shield_turns_remaining == 0 {
            return;
        }
        self.shield_turns_remaining -= 1;
    }

    fn process_poison(&mut self) {
        if self.poison_turns_remaining == 0 {
            return;
        }
        self.damage_boss(3);
        self.poison_turns_remaining -= 1;
    }

    fn process_recharge(&mut self) {
        if self.recharge_turns_remaining == 0 {
            return;
        }
        self.mana += 101;
        self.recharge_turns_remaining -= 1;
    }

    fn spend_mana(&mut self, mana: u32) {
        if self.result != GameResult::Pending {
            return;
        }
        if mana > self.mana {
            self.result = GameResult::Lost;
        } else {
            self.mana_spent += mana;
            self.mana -= mana;
        }
    }

    fn damage_boss(&mut self, damage: u8) {
        if self.result != GameResult::Pending {
            return;
        }
        if damage >= self.boss_hp {
            self.result = GameResult::Won;
        } else {
            self.boss_hp -= damage;
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum GameResult {
    Won,
    Lost,
    Pending,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PlayerMove {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

fn read_input(data: &str) -> (u8, u8) {
    let mut lines = data.lines();
    let (Some(hp), Some(dam), None) = (lines.next(), lines.next(), lines.next()) else {
        unreachable!();
    };
    let hp = hp.strip_prefix("Hit Points: ").unwrap().parse().unwrap();
    let dam = dam.strip_prefix("Damage: ").unwrap().parse().unwrap();
    (hp, dam)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_1() {
        let data = include_str!("test.txt");
        assert_eq!(250 - 24, part_one(data, 10, 250));
    }

    #[test]
    fn one_2() {
        let data = include_str!("test2.txt");
        assert_eq!(250 - 114 + 505, part_one(data, 10, 250));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data, 10, 250));
    }
}
