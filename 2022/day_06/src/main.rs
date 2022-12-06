pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

pub fn bench() {
    let data = include_str!("input.txt");
    let _ = part_one(data);
    let _ = part_two(data);
}

fn part_one(data: &str) -> usize {
    data.lines().map(|l| find_window(l, 4)).sum()
}

fn part_two(data: &str) -> usize {
    data.lines().map(|l| find_window(l, 14)).sum()
}

fn find_window(line: &str, len: usize) -> usize {
    for (i, window) in line.as_bytes().windows(len).enumerate() {
        if unique(window) {
            return i + len;
        }
    }
    0
}

fn unique(letters: &[u8]) -> bool {
    if let Some((first, rest)) = letters.split_first() {
        if rest.contains(first) {
            return false;
        } else {
            return unique(rest);
        }
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(7 + 5 + 6 + 10 + 11, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(19 + 23 + 23 + 29 + 26, part_two(data));
    }
}
