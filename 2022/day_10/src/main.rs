pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> i32 {
    const CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut cycle = 1_i32;
    let mut x_reg = 1_i32;
    let mut ans = 0;
    for line in data.lines() {
        let (op, val) = line.split_at(4);
        match op {
            "noop" => {
                if CYCLES.contains(&cycle) {
                    ans += cycle * x_reg;
                }
                cycle += 1;
            }
            "addx" => {
                if CYCLES.contains(&cycle) {
                    ans += cycle * x_reg;
                } else if CYCLES.contains(&(cycle + 1)) {
                    ans += (cycle + 1) * x_reg;
                }
                cycle += 2;
                x_reg += val.trim().parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    ans
}

fn part_two(data: &str) -> i32 {
    let mut screen: Screen = [[false; 40]; 6];
    let mut cycle = 0;
    let mut x_reg = 1;
    for line in data.lines() {
        let (op, val) = line.split_at(4);
        match op {
            "noop" => {
                update_screen(cycle, x_reg, &mut screen);
                cycle += 1;
            }
            "addx" => {
                update_screen(cycle, x_reg, &mut screen);
                cycle += 1;
                update_screen(cycle, x_reg, &mut screen);
                cycle += 1;
                x_reg += val.trim().parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    screen.print();
    0
}

fn update_screen(cycle: i32, x_reg: i32, screen: &mut Screen) {
    let row = cycle / 40;
    let col = cycle % 40;
    if (x_reg - 1..=x_reg + 1).contains(&(col)) {
        let row: usize = row.try_into().unwrap();
        let col: usize = col.try_into().unwrap();
        screen[row][col] = true;
    }
}

trait PrintScreen {
    fn print(&self);
}

type Screen = [[bool; 40]; 6];

impl PrintScreen for Screen {
    fn print(&self) {
        for row in self {
            for elem in row {
                if *elem {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(13_140, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
