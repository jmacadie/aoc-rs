pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<5_000>(data));
    println!("Part 2: {}", part_two::<5_000>(data));
}

fn part_one<const N: usize>(data: &str) -> i64 {
    let (mut nums, mut reference) = read_file::<N>(data);
    mix::<N>(&mut nums, &mut reference);
    get_coords::<N>(&nums)
}

fn part_two<const N: usize>(data: &str) -> i64 {
    const KEY: i64 = 811_589_153;
    let (mut nums, mut reference) = read_file::<N>(data);
    for val in nums.iter_mut() {
        *val *= KEY;
    }
    for _ in 0..10 {
        mix::<N>(&mut nums, &mut reference);
    }
    get_coords::<N>(&nums)
}

fn get_coords<const N: usize>(nums: &[i64]) -> i64 {
    let index = find_index(nums, 0);
    let x = nums[(1_000 + index) % N];
    let y = nums[(2_000 + index) % N];
    let z = nums[(3_000 + index) % N];
    x + y + z
}

fn mix<const N: usize>(nums: &mut [i64], reference: &mut [usize]) {
    for i in 0..N {
        let index = find_index(reference, i);
        let move_by = nums[index];
        move_array::<usize, N>(reference, index, move_by);
        move_array::<i64, N>(nums, index, move_by);
    }
}

fn find_index<T: PartialEq>(array: &[T], target: T) -> usize {
    for (i, val) in array.iter().enumerate() {
        if val == &target {
            return i;
        }
    }
    0
}

fn move_array<T: Copy, const N: usize>(array: &mut [T], from: usize, by: i64) {
    let lim = i64::try_from(N).unwrap() - 1;
    let mut to = (i64::try_from(from).unwrap() + by) % lim;
    if to < 0 {
        to += lim;
    }
    if to == 0 {
        to = lim;
    }
    let to = usize::try_from(to).unwrap();
    let moved = array[from];
    if from < to {
        for i in from..to {
            array[i] = array[i + 1];
        }
    } else {
        for i in (to + 1..from + 1).rev() {
            array[i] = array[i - 1];
        }
    }
    array[to] = moved;
}

fn read_file<const N: usize>(data: &str) -> ([i64; N], [usize; N]) {
    let mut nums = [0; N];
    let mut reference = [0; N];
    for (i, line) in data.lines().enumerate() {
        nums[i] = line.parse().unwrap();
        reference[i] = i;
    }
    (nums, reference)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(3, part_one::<7>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(1_623_178_306, part_two::<7>(data));
    }
}
