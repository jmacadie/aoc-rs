use std::fmt::{self, Display, Formatter};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data, 99));
    println!("Part 2: {}", part_two(data, 99));
}

pub fn bench() {
    let data = include_str!("input.txt");
    let _ = part_one(data, 99);
    let _ = part_two(data, 99);
}

fn part_one(data: &str, size: usize) -> u32 {
    let map = get_map(data);
    let mut visible: Map<bool> = [[false; 99]; 99];
    look_down(&map, &mut visible, size, size);
    look_left(&map, &mut visible, size, size);
    look_up(&map, &mut visible, size, size);
    look_right(&map, &mut visible, size, size);
    count_visible(&visible)
}

fn part_two(data: &str, size: usize) -> u64 {
    let map = get_map(data);
    let mut max_score = 0_u64;
    for row in 0..size {
        for col in 0..size {
            let score = scenic_score(&map, row, col, size, size);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

type Map<T> = [[T; 99]; 99];

struct MapWrap<T>(Map<T>);

impl Display for MapWrap<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.0 {
            for elem in row {
                if elem {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_map(data: &str) -> Map<u8> {
    let mut out: Map<u8> = [[0; 99]; 99];
    for (row, line) in data.lines().enumerate() {
        for (col, ascii_digit) in line.bytes().enumerate() {
            // Add one so the zero hieght trees aren't zero any more
            out[row][col] = ascii_digit - b'0' + 1;
        }
    }
    out
}

fn count_visible(visible: &Map<bool>) -> u32 {
    let mut count = 0;
    for row in visible {
        for &elem in row {
            if elem {
                count += 1;
            }
        }
    }
    count
}

fn scenic_score(map: &Map<u8>, row: usize, col: usize, height: usize, width: usize) -> u64 {
    let curr = map[row][col];
    let (mut up, mut down, mut left, mut right) = (0_u64, 0_u64, 0_u64, 0_u64);

    // Quit out on boundaries
    if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
        return 0;
    }

    // Look up
    for (dist, i) in (0..row).rev().enumerate() {
        if i == 0 || map[i][col] >= curr {
            up = (dist + 1).try_into().unwrap();
            break;
        }
    }

    // Look down
    for (dist, i) in (row + 1..height).enumerate() {
        if i == height - 1 || map[i][col] >= curr {
            down = (dist + 1).try_into().unwrap();
            break;
        }
    }

    // Look left
    for (dist, i) in (0..col).rev().enumerate() {
        if i == 0 || map[row][i] >= curr {
            left = (dist + 1).try_into().unwrap();
            break;
        }
    }

    // Look right
    for (dist, i) in (col + 1..width).enumerate() {
        if i == width - 1 || map[row][i] >= curr {
            right = (dist + 1).try_into().unwrap();
            break;
        }
    }

    up * down * left * right
}

fn look_down(map: &Map<u8>, visible: &mut Map<bool>, height: usize, width: usize) {
    let mut max;
    for col in 0..width {
        max = 0_u8;
        for row in 0..height {
            if map[row][col] > max {
                max = map[row][col];
                visible[row][col] = true;
                if max == 10 {
                    break;
                }
            }
        }
    }
}

fn look_left(map: &Map<u8>, visible: &mut Map<bool>, height: usize, width: usize) {
    let mut max;
    for row in 0..height {
        max = 0_u8;
        for col in (0..width).rev() {
            if map[row][col] > max {
                max = map[row][col];
                visible[row][col] = true;
                if max == 10 {
                    break;
                }
            }
        }
    }
}

fn look_up(map: &Map<u8>, visible: &mut Map<bool>, height: usize, width: usize) {
    let mut max;
    for col in 0..width {
        max = 0_u8;
        for row in (0..height).rev() {
            if map[row][col] > max {
                max = map[row][col];
                visible[row][col] = true;
                if max == 10 {
                    break;
                }
            }
        }
    }
}

fn look_right(map: &Map<u8>, visible: &mut Map<bool>, height: usize, width: usize) {
    let mut max;
    for row in 0..height {
        max = 0_u8;
        for col in 0..width {
            if map[row][col] > max {
                max = map[row][col];
                visible[row][col] = true;
                if max == 10 {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(21, part_one(data, 5));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(8, part_two(data, 5));
    }
}
