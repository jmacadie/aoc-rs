use crate::common::file::read_lines;
use core::ops::Add;

const ROOT: &str = "src/year_2020/day_03/";

struct Map {
    trees: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl Map {
    pub fn new(filename: &str) -> Self {
        let file = format!("{}{}", ROOT, filename);

        let width = Self::calc_width(&file);
        let height = Self::calc_height(&file);

        let mut out = Vec::with_capacity(height);

        let lines = read_lines(file).unwrap();
        for line in lines.flatten() {
            out.push(Self::read_line(&line));
        }
        Map {
            trees: out,
            height,
            width,
        }
    }

    fn read_line(line: &str) -> Vec<bool> {
        line.chars().map(|c| c == '#').collect()
    }

    fn calc_width(file: &str) -> usize {
        read_lines(file)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .count()
    }

    fn calc_height(file: &str) -> usize {
        read_lines(file).unwrap().count()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn val(&self, position: Point) -> bool {
        self.trees[position.y][position.x]
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Journey<'a> {
    map: &'a Map,
    position: Point,
    direction: Point,
    trees_encountered: i32,
}

impl<'a> Journey<'a> {
    pub fn new(map: &'a Map, direction: Point) -> Self {
        let position = Point { x: 0, y: 0 };
        Journey {
            map,
            position,
            direction,
            trees_encountered: 0,
        }
    }
}

impl Journey<'_> {
    fn step(&mut self) {
        let mut position = self.position + self.direction;
        if position.x >= self.map.width() {
            position.x %= self.map.width();
        }
        self.position = position;
        if self.map.val(position) {
            self.trees_encountered += 1;
        }
    }

    pub fn travel(&mut self) {
        while self.position.y < (self.map.height() - 1) {
            self.step();
        }
    }

    pub fn trees_encountered(&self) -> i32 {
        self.trees_encountered
    }
}

pub fn run() {
    let test_map = Map::new("test.txt");
    let main_map = Map::new("input.txt");

    run_part_one(&test_map, &main_map);
    run_part_two(&test_map, &main_map);
}

fn run_part_one(test_map: &Map, main_map: &Map) {
    let direction = Point { x: 3, y: 1 };
    assert_eq!(7, get_trees(test_map, direction));
    println!(
        "Part 1: {} trees encountered",
        get_trees(main_map, direction)
    );
}

fn run_part_two(test_map: &Map, main_map: &Map) {
    let directions = vec![
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];

    assert_eq!(336, all_directions(test_map, &directions));
    println!(
        "Part 2: {} trees encountered product",
        all_directions(main_map, &directions)
    );
}

fn all_directions(map: &Map, directions: &[Point]) -> i32 {
    directions
        .iter()
        .map(|direction| get_trees(map, *direction))
        .product()
}

fn get_trees(map: &Map, direction: Point) -> i32 {
    let mut journey = Journey::new(map, direction);
    journey.travel();
    journey.trees_encountered()
}
