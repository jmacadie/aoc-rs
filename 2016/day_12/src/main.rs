#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let (base, _, f1, f2) = get_magic_numbers(data);
    fibo(base) + f1 * f2
}

fn part_two(data: &str) -> u32 {
    let (base, ext, f1, f2) = get_magic_numbers(data);
    fibo(base + ext) + f1 * f2
}

fn fibo(num: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    for _ in 0..num {
        let c = a;
        a += b;
        b = c;
    }
    a
}

fn get_magic_numbers(input: &str) -> (u32, u32, u32, u32) {
    let mut lines = input.lines();

    // Base Fibo number
    let line = lines.nth(2).unwrap();
    let base = line
        .trim_start_matches("cpy ")
        .trim_end_matches(" d")
        .parse()
        .unwrap();

    // Fibo extension for part 2
    let line = lines.nth(2).unwrap();
    let ext = line
        .trim_start_matches("cpy ")
        .trim_end_matches(" c")
        .parse()
        .unwrap();

    // Other two factors
    let line = lines.nth(10).unwrap();
    let f1 = line
        .trim_start_matches("cpy ")
        .trim_end_matches(" c")
        .parse()
        .unwrap();
    let line = lines.next().unwrap();
    let f2 = line
        .trim_start_matches("cpy ")
        .trim_end_matches(" d")
        .parse()
        .unwrap();

    (base, ext, f1, f2)
}
