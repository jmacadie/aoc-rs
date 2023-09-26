#!warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> i32 {
    data.lines().map(|l| l.parse::<i32>().unwrap()).sum()
}

fn part_two(data: &str) -> i32 {
    // Convert the input into numbers and dump into a vector
    let mut nums = Vec::with_capacity(1_000);
    nums.extend(data.lines().map(|l| l.parse::<i32>().unwrap()));
    let full = nums.iter().sum();

    // Search through the array to find the first point with the minimum cycle
    // This will be the end of the first zero sum cycle, and so the first
    // repeated frequency
    // We add the `cycle * full` to it as we find the end point on the initial pass
    // through the array but the answer will be it's value after all the cycles have
    // happened
    nums.iter()
        .enumerate()
        .fold(
            (0, (0, i32::MAX)),
            |(freq, (min_freq, min_cyc)), (idx, &i)| {
                let freq = freq + i;
                if let Some(cycle) = cycle(&nums[..idx + 1], &nums[idx + 1..], full) {
                    if cycle < min_cyc {
                        return (freq, (freq + cycle * full, cycle));
                    }
                }
                (freq, (min_freq, min_cyc))
            },
        )
        .1
         .0
}

// Loop backwards through the original array from a segment point
// Try to detect if there is a sum of numbers across a run from that start
// number to any other number that has a gap sum that is a whole multiple of
// the full sum across an entire cycle (but with opposite sign).
// This is interesting as it means that after some number of cycles, the run
// between those two numbers will add to zero, which means that they will have
// the same frequency
// If there are more than one cycle candidates found, return the one with the
// smallest cycle, since we will see this one first
// We cycle backwards as we are effectively searching from the end-point of the
// sum cycle, towards the start. We do this as we want to find the first cycle
// and this will be 'found' at the end of the cycle
// The skip(1) is because it is trivially obvious that searching the full array
// can never produce a sum that is opposite sign to the sum of the whole array,
// since they are the same thing
fn cycle(left: &[i32], right: &[i32], full: i32) -> Option<i32> {
    left.iter()
        .rev()
        .chain(right.iter().skip(1).rev())
        .enumerate()
        .fold((0, None), |(sum, mut min), (count, i)| {
            let sum = sum - i;
            // Test that sum _will_ be taken down to zero by some number of cycles
            if sum % full == 0 && (sum ^ full) >= 0 {
                // If we've been through the left array then we need to bump the cycle count
                let cycle = if count < left.len() {
                    sum / full
                } else {
                    sum / full + 1
                };
                min = Some(min.map_or_else(|| cycle, |m| std::cmp::min(m, cycle)));
            }
            (sum, min)
        })
        .1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(3, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_two(data));
    }
}
