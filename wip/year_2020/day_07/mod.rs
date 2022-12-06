use std::collections::HashMap;
use std::ops::Mul;

use crate::common::file::read_lines;

const ROOT: &str = "src/year_2020/day_07/";

pub fn run() {
    let test_rules = BagRules::new("test.txt");
    let main_rules = BagRules::new("input.txt");
    run_part_one(&test_rules, &main_rules);
    run_part_two(&test_rules, &main_rules);
}

fn run_part_one(test: &BagRules, main: &BagRules) {
    assert_eq!(4, test.find_all_parents("shiny gold").len());

    let count = main.find_all_parents("shiny gold").len();
    println!(
        "Part 1: {} bags eventually contain at least one shiny gold bag",
        count
    );
}

fn run_part_two(test: &BagRules, main: &BagRules) {
    assert_eq!(32, test.count_all_children("shiny gold"));

    let count = main.count_all_children("shiny gold");
    println!("Part 2: The shiny gold bag contains {} other bags", count);
}

#[derive(Clone, Debug)]
struct BagAmount {
    bag: String,
    amount: i32,
}

impl Mul<i32> for &mut BagAmount {
    type Output = ();

    fn mul(self, rhs: i32) {
        self.amount *= rhs;
    }
}

#[derive(Clone)]
struct BagsContained {
    bags: Vec<BagAmount>,
}

struct BagRules {
    rules: HashMap<String, BagsContained>,
}

impl BagRules {
    fn new(filename: &str) -> Self {
        let file = format!("{}{}", ROOT, filename);
        let lines = read_lines(file).unwrap();
        let mut rules = HashMap::new();
        for line in lines.flatten() {
            let (name, inner_bags) = Self::process_line(&line);
            rules.insert(name, inner_bags);
        }
        BagRules { rules }
    }

    fn process_line(line: &str) -> (String, BagsContained) {
        let mut split = line.split(" contain ");
        let name = split.next().unwrap().trim_end_matches(" bags").to_owned();
        let inner_bags_list = split.next().unwrap().trim_end_matches('.');
        let mut bags = Vec::new();
        if inner_bags_list == "no other bags" {
            return (name, BagsContained { bags });
        }
        for bag in inner_bags_list.split(", ") {
            bags.push(Self::process_bag(bag));
        }
        (name, BagsContained { bags })
    }

    fn process_bag(bag: &str) -> BagAmount {
        let bag = bag.trim_end_matches(" bags");
        let bag = bag.trim_end_matches(" bag");
        let mut parts = bag.split(' ');
        let amount = parts.next().unwrap().parse().unwrap();
        //let name = parts.as_str().to_owned();
        let mut name = String::new();
        for word in parts {
            name.push_str(word);
            name.push(' ');
        }
        let _ = name.pop();
        BagAmount { bag: name, amount }
    }

    fn find_direct_parents(&self, child_bag: &str) -> Vec<String> {
        let mut out = Vec::new();
        for (outer_bag, inner) in &self.rules {
            for inner_bag in &inner.bags {
                if inner_bag.bag == child_bag {
                    out.push(outer_bag.clone());
                }
            }
        }
        out
    }

    fn find_all_parents(&self, source_bag: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut parents = self.find_direct_parents(source_bag);
        for bag in &parents {
            let mut parents_2 = self.find_all_parents(bag);
            out.append(&mut parents_2);
        }
        out.append(&mut parents);
        out.sort_unstable();
        out.dedup();
        out
    }

    fn children(&self, source_bag: &str) -> BagsContained {
        self.rules.get(source_bag).unwrap().clone()
    }

    #[allow(clippy::no_effect)]
    fn find_all_children(&self, source_bag: &str) -> Vec<BagAmount> {
        let mut out = Vec::new();
        let mut children = self.children(source_bag).bags;
        for child_bag in &children {
            let mut children_2 = self.find_all_children(&child_bag.bag);
            for child_bag_2 in &mut children_2 {
                // false positive by clippy here this DOES have an effect
                child_bag_2 * child_bag.amount;
            }
            out.append(&mut children_2);
        }
        out.append(&mut children);
        out
    }

    fn count_all_children(&self, source_bag: &str) -> i32 {
        self.find_all_children(source_bag)
            .iter()
            .fold(0_i32, |acc, ba| acc + ba.amount)
    }
}
