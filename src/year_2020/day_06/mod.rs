use crate::common::file::read_lines;

const ROOT: &str = "src/year_2020/day_06/";

pub fn run() {
    run_part_one();
    run_part_two();
}

fn run_part_one() {
    let method = CountType::Or;
    assert_eq!(11, count_questions("test.txt", &method));
    println!("Part 1: Question count = {}", count_questions("input.txt", &method));
}

fn run_part_two() {
    let method = CountType::And;
    assert_eq!(6, count_questions("test.txt", &method));
    println!("Part 2: Question count = {}", count_questions("input.txt", &method));
}

enum CountType {
    Or,
    And,
}

// TODO: This is a mess!
fn count_questions(filename: &str, method: &CountType) -> i32 {
    let file = format!("{}{}", ROOT, filename);
    let lines = read_lines(file).unwrap();
    let mut questions = [false; 26];
    let mut count: i32 = 0;
    let mut first = true;
    for line in lines.flatten() {
        if line.is_empty() {
            count += count_group(&questions);
            questions = [false; 26];
            first = true;
        } else {
            match method {
                CountType::Or => process_line(&line, &mut questions),
                CountType::And => {
                    if first {
                        process_line(&line, &mut questions);
                    } else {
                        let mut next = [false; 26];
                        process_line(&line, &mut next);
                        for (i, val) in next.iter().enumerate() {
                            if !val {
                                questions[i] = false;
                            }
                        }
                    }
                },
            };
            first = false;
        }
    }
    count
}

fn process_line(line: &str, questions: &mut [bool]) {
    for c in line.chars() {
        questions[char_to_num(c)] = true;
    }
}

fn count_group(questions: &[bool]) -> i32 {
    questions.iter().fold(0_i32, |acc, q| if *q { acc + 1 } else { acc } )
}

fn char_to_num(c: char) -> usize {
    let num: usize = c.to_digit(36)
        .unwrap()
        .try_into()
        .unwrap();
    num - 10
}

