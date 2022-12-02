#![warn(unused_imports, dead_code)]

use crate::common::file::read_lines;
use std::str::Split;

const ROOT: &str = "src/year_2022/day_02/";

pub fn run() {
    run_part_one();
    run_part_two();
}

fn run_part_one() {
    assert_eq!(15, play_game("test.txt", false));
    println!("Part 1: Game score is {}", play_game("input.txt", false));
}

fn run_part_two() {
    assert_eq!(12, play_game("test.txt", true));
    println!("Part 2: Game score is {}", play_game("input.txt", true));
}

fn play_game(filename: &str, from_result: bool) -> i32 {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut score = 0_i32;
    for line in lines.flatten() {
        let mut s = line.split(' ');
        let opp = Move::new(get_char(&mut s));
        let you = if from_result {
            Move::from_result(get_char(&mut s), opp)
        } else {
            Move::new(get_char(&mut s))
        };
        score += total_score(opp, you);
    }
    score
}

fn get_char(split: &mut Split<char>) -> char {
    split.next().unwrap().chars().next().unwrap()
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn new(move_str: char) -> Self {
        match move_str {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissor,
            _ => unreachable!(),
        }
    }

    fn from_result(res_str: char, other: Self) -> Self {
        match res_str {
            'X' => other.beats_move(),
            'Y' => other,
            'Z' => other.beaten_by(),
            _ => unreachable!(),
        }
    }

    fn beats_move(self) -> Self {
        match self {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        }
    }

    fn beaten_by(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        }
    }

    fn beats(self, other: Self) -> bool {
        other == self.beats_move()
    }
}

fn total_score(opp: Move, you: Move) -> i32 {
    shape_score(you) + game_score(opp, you)
}

fn shape_score(your_move: Move) -> i32 {
    match your_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3,
    }
}

fn game_score(opp: Move, you: Move) -> i32 {
    if opp == you {
        3
    } else if you.beats(opp) {
        6
    } else {
        0
    }
}
