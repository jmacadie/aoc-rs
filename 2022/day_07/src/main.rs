use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    let sizes = get_sizes(data);
    println!("Part 1: {}", part_one(&sizes));
    println!("Part 2: {}", part_two(&sizes));
}

fn part_one(sizes: &[u64; 200]) -> u64 {
    sizes.iter().filter(|&x| x <= &100_000).sum()
}

fn part_two(sizes: &[u64; 200]) -> u64 {
    const TOTAL_SPACE: u64 = 70_000_000;
    const TARGET_SPACE: u64 = 30_000_000;
    let free_space = TOTAL_SPACE - sizes[0];
    let target = TARGET_SPACE - free_space;
    *sizes
        .iter()
        .filter(|&x| x >= &target)
        .sorted_unstable()
        .next()
        .unwrap()
}

fn get_sizes(data: &str) -> [u64; 200] {
    let mut current_path = [0_usize; 64];
    let mut path_head = 0_usize;
    let mut folder_counter = 0_usize;
    let mut sizes = [0_u64; 200];

    for line in data.lines() {
        let mut parts = line.split(' ');
        match parts.next() {
            Some("$") => {
                // it's a command
                match parts.next() {
                    Some("ls") => (), // ls: ignore
                    Some("cd") => match parts.next() {
                        Some("..") => {
                            // move one level back up the path tree
                            path_head -= 1;
                        }
                        Some(_) => {
                            // add a level to the path tree
                            current_path[path_head] = folder_counter;
                            folder_counter += 1;
                            path_head += 1;
                        }
                        None => unreachable!(), // there's always a command after cd
                    },
                    _ => unreachable!(), // ls and cd are the only commands we get
                }
            }
            Some("dir") => (), // directory listing: ignore
            Some(file_size) => {
                // file listing: extract the size, ignore the file name and add to all super folders up the path
                let file_size = file_size.parse::<u64>().unwrap();
                for i in 0..path_head {
                    sizes[current_path[i]] += file_size;
                }
            }
            _ => unreachable!(), // No other type of input expected
        }
    }
    sizes
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let sizes = get_sizes(data);
        assert_eq!(95_437, part_one(&sizes));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let sizes = get_sizes(data);
        assert_eq!(24_933_642, part_two(&sizes));
    }
}
