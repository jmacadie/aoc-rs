#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    data.lines().map(get_dims).map(required_paper).sum()
}

fn part_two(data: &str) -> u32 {
    data.lines().map(get_dims).map(required_ribbon).sum()
}

fn required_ribbon(dimensions: (u32, u32, u32)) -> u32 {
    smallest_perimeter(dimensions) + cubic_volume(dimensions)
}

fn smallest_perimeter(dimensions: (u32, u32, u32)) -> u32 {
    let (length, width, height) = dimensions;
    let side_1 = 2 * (length + width);
    let side_2 = 2 * (width + height);
    let side_3 = 2 * (height + length);
    let min = std::cmp::min(side_1, side_2);
    std::cmp::min(min, side_3)
}

const fn cubic_volume(dimensions: (u32, u32, u32)) -> u32 {
    let (length, width, height) = dimensions;
    length * width * height
}

fn required_paper(dimensions: (u32, u32, u32)) -> u32 {
    let (length, width, height) = dimensions;
    let side_1 = length * width;
    let side_2 = width * height;
    let side_3 = height * length;
    let mut min = std::cmp::min(side_1, side_2);
    min = std::cmp::min(min, side_3);
    2 * side_1 + 2 * side_2 + 2 * side_3 + min
}

fn get_dims(line: &str) -> (u32, u32, u32) {
    let mut parts = line.split('x');
    let (Some(x), Some(y), Some(z), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
            unreachable!();
        };
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    let z = z.parse().unwrap();
    (x, y, z)
}
