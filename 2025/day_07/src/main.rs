#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let start = data.bytes().position(|b| b == b'S').unwrap();
    let mut beams = Vec::with_capacity(start * 2);
    let mut next_beams = Vec::with_capacity(start * 2);
    next_beams.push(start);

    data.lines().map(str::as_bytes).fold(0, |acc, line| {
        std::mem::swap(&mut beams, &mut next_beams);
        acc + advance_beams(line, &beams, &mut next_beams)
    })
}

fn advance_beams(line: &[u8], beams: &[usize], next_beams: &mut Vec<usize>) -> usize {
    next_beams.clear();

    beams.iter().fold(0, |acc, &b| {
        if line[b] == b'^' {
            add_beam(b - 1, next_beams);
            add_beam(b + 1, next_beams);
            acc + 1
        } else {
            add_beam(b, next_beams);
            acc
        }
    })
}

fn add_beam(beam: usize, beams: &mut Vec<usize>) {
    if beams.last().copied() != Some(beam) {
        beams.push(beam);
    }
}

fn part_two(data: &str) -> usize {
    let start = data.bytes().position(|b| b == b'S').unwrap();
    let mut beams = Vec::with_capacity(start * 2);
    let mut next_beams = Vec::with_capacity(start * 2);
    next_beams.push((start, 1));

    for line in data.lines().map(str::as_bytes) {
        std::mem::swap(&mut beams, &mut next_beams);
        advance_beams_with_count(line, &beams, &mut next_beams);
    }

    next_beams.iter().map(|&(_, count)| count).sum()
}
fn advance_beams_with_count(
    line: &[u8],
    beams: &[(usize, usize)],
    next_beams: &mut Vec<(usize, usize)>,
) {
    next_beams.clear();

    for &(b, c) in beams {
        if line[b] == b'^' {
            add_beam_with_count(b - 1, c, next_beams);
            add_beam_with_count(b + 1, c, next_beams);
        } else {
            add_beam_with_count(b, c, next_beams);
        }
    }
}

fn add_beam_with_count(beam: usize, count: usize, beams: &mut Vec<(usize, usize)>) {
    if let Some(&(last_beam, _)) = beams.last()
        && last_beam == beam
    {
        let (_, last_count) = beams.pop().unwrap();
        beams.push((beam, last_count + count));
    } else {
        beams.push((beam, count));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(21, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(40, part_two(data));
    }
}
