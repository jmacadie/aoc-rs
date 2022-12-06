pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));

    Ok(())
}

pub fn bench() {
    let data = include_str!("input.txt");
    let _ = part_one(data);
    let _ = part_two(data);
}

fn part_one(data: &str) -> u64 {
    data.lines()
        .map(range_parts)
        .map(|(a, b)| (str_to_range(a), str_to_range(b)))
        .map(contained)
        .fold(0, |acc, v| if v { acc + 1 } else { acc })
}

fn part_two(data: &str) -> u64 {
    data.lines()
        .map(range_parts)
        .map(|(a, b)| (str_to_range(a), str_to_range(b)))
        .map(overlap)
        .fold(0, |acc, v| if v { acc + 1 } else { acc })
}

fn range_parts(line: &str) -> (&str, &str) {
    let mut parts = line.split(',');
    (parts.next().unwrap(), parts.next().unwrap())
}

fn str_to_range(input: &str) -> (u64, u64) {
    let mut x = input.split('-').map(|v| v.parse::<u64>().unwrap());
    (x.next().unwrap(), x.next().unwrap())
}

fn contained((a, b): ((u64, u64), (u64, u64))) -> bool {
    contains(a, b) || contains(b, a)
}

fn contains(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn overlap((a, b): ((u64, u64), (u64, u64))) -> bool {
    overlap_one(a, b) || overlap_one(b, a)
}

fn overlap_one(a: (u64, u64), b: (u64, u64)) -> bool {
    a.1 >= b.0 && a.0 < b.1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt").trim_end_matches('\n');
        assert_eq!(2, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(4, part_two(data));
    }
}
