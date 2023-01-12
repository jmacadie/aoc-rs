#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<100>(data));
    println!("Part 2: {}", part_two::<100>(data));
}

fn part_one<const N: usize>(data: &str) -> u16 {
    let mut grid = read::<N>(data);
    for _ in 0..100 {
        grid = step(&grid);
    }
    count_on(&grid)
}

fn part_two<const N: usize>(data: &str) -> u16 {
    let mut grid = read::<N>(data);
    switch_on_corners::<N>(&mut grid);
    for _ in 0..100 {
        grid = step(&grid);
        switch_on_corners::<N>(&mut grid);
    }
    count_on(&grid)
}

type Grid<const N: usize> = [u128; N];

fn val_at(row: u128, index: usize) -> bool {
    row & 2_u128.pow(u32::try_from(index).unwrap()) != 0
}

fn count_on<const N: usize>(grid: &Grid<N>) -> u16 {
    grid.iter().map(count_on_row).sum()
}

fn count_on_row(row: &u128) -> u16 {
    let mut count = 0;
    let mut row = *row;
    while row > 0 {
        count += u16::try_from(row & 1).unwrap();
        row >>= 1;
    }
    count
}

#[allow(dead_code)]
fn print<const N: usize>(grid: &Grid<N>) {
    for row in grid.iter().take(N) {
        for col in (0..N).rev() {
            if val_at(*row, col) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn read<const N: usize>(data: &str) -> Grid<N> {
    let mut out = [0; N];
    for (val, source) in out.iter_mut().zip(data.lines()) {
        *val = read_line(source);
    }
    out
}

fn read_line(line: &str) -> u128 {
    line.as_bytes().iter().fold(0, |acc, ch| {
        acc * 2
            + match ch {
                b'.' => 0,
                b'#' => 1,
                _ => unreachable!(),
            }
    })
}

fn step<const N: usize>(source: &Grid<N>) -> Grid<N> {
    let mut out = [0; N];
    let mut prev = [0; 130];
    let mut curr = [0; 130];
    let mut next = [0; 130];
    for (row, data) in source.iter().enumerate() {
        for i in 0..N {
            if val_at(*data, i) {
                prev[i] += 1;
                prev[i + 1] += 1;
                prev[i + 2] += 1;
                curr[i] += 1;
                curr[i + 2] += 1;
                next[i] += 1;
                next[i + 1] += 1;
                next[i + 2] += 1;
            }
        }
        if row > 0 {
            add_row(&prev[1..], source[row - 1], &mut out, row);
        }
        prev = curr;
        curr = next;
        next = [0; 130];
    }
    add_row(&prev[1..], source[N - 1], &mut out, N);
    out
}

fn add_row<const N: usize>(counts: &[u8], source: u128, grid: &mut Grid<N>, row: usize) {
    let mut new = 0_u128;
    for (col, &count) in counts.iter().enumerate().take(N) {
        if count == 3 || (count == 2 && val_at(source, col)) {
            new += 2_u128.pow(u32::try_from(col).unwrap());
        }
    }
    grid[row - 1] = new;
}

fn switch_on_corners<const N: usize>(grid: &mut Grid<N>) {
    grid[0] = switch_on_ends::<N>(grid[0]);
    grid[N - 1] = switch_on_ends::<N>(grid[N - 1]);
}

fn switch_on_ends<const N: usize>(row: u128) -> u128 {
    row | 1 | 2_u128.pow(u32::try_from(N - 1).unwrap())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(4, part_one::<6>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(7, part_two::<6>(data));
    }
}
