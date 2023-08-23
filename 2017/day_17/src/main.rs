#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let cycle: usize = data.trim().parse().unwrap();
    let mut positions = [0; 2018];

    // First find the initial insert positions
    let mut last_p = 0;
    for (i, p) in positions.iter_mut().enumerate().skip(1) {
        *p = (last_p + cycle) % i + 1;
        last_p = *p;
    }

    // Then loop through, bumping up the position of any item
    // that has been moved up by a subsequent lower-placed insert
    for i in 1..2018 {
        let (lower, rest) = positions.split_at_mut(i);
        let val = rest[0];
        for p in lower.iter_mut() {
            if *p >= val {
                *p += 1;
            }
        }
    }

    positions.iter().position(|&v| v == last_p + 1).unwrap()
}

fn part_two(data: &str) -> u32 {
    let cycle: u32 = data.trim().parse().unwrap();
    let mut spinlock_size = 0;
    let mut position = 0;
    let mut last = 0;

    // For the small starting values just increment the spinlock size
    // and compute the insert position in steps of one
    while position + cycle > spinlock_size {
        spinlock_size += 1;
        position = (position + cycle) % spinlock_size + 1;
    }

    // When big enough though, we don't have to increment the spinlock in steps
    // of one any more. The modulo logic only applies if the cycle will take
    // us over the current spinlock size
    // So we can batch up our steps into bigger increments, catching each actual
    // modulo operation. As the spinlock gets larger and larger we will be able
    // to skip progressively more steps, which makes this quite a big performance
    // improvement
    while spinlock_size <= 50_000_000 {
        // If we're at position 1, update the last variable to record
        // what step we saw this on
        if position == 1 {
            last = spinlock_size;
        }

        let cycles = (spinlock_size - position) / cycle + 1;

        spinlock_size += cycles;
        position = (position + cycles * (cycle + 1)) % spinlock_size;
    }

    // Return the last step that fell on position 1
    // It's the  only one that can't have been bumped out and so must
    // be next to zero at the end
    last
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(638, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(1_222_153, part_two(data));
    }
}
