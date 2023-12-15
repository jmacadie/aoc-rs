#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.split("\n\n")
        .map(|pat| Pattern::new(pat).summarise(0))
        .sum()
}

fn part_two(data: &str) -> usize {
    data.split("\n\n")
        .map(|pat| Pattern::new(pat).summarise(1))
        .sum()
}

struct Pattern<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Pattern<'a> {
    fn new(data: &'a str) -> Self {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();

        Self {
            data: data.as_bytes(),
            width,
            height,
        }
    }

    fn row_reflection(&self, error_count: usize) -> Option<usize> {
        (0..self.height - 1)
            .find(|&fold_row| {
                let range = std::cmp::min(fold_row + 1, self.height - fold_row - 1);

                (0..range)
                    .try_fold(0, |outer, offset| {
                        (0..self.width)
                            .try_fold(0, |inner, col| {
                                let lower = (fold_row - offset) * (self.width + 1) + col;
                                let upper = lower + (2 * offset + 1) * (self.width + 1);
                                let errs = usize::from(self.data[lower] != self.data[upper]);

                                if inner + errs > error_count {
                                    Err(0)
                                } else {
                                    Ok(inner + errs)
                                }
                            })
                            .and_then(|inner| {
                                if outer + inner > error_count {
                                    Err(0)
                                } else {
                                    Ok(outer + inner)
                                }
                            })
                    })
                    .map_or(false, |sum| sum == error_count)
            })
            .map(|r| r + 1)
    }

    fn col_reflection(&self, error_count: usize) -> Option<usize> {
        (0..self.width - 1)
            .find(|&fold_col| {
                let range = std::cmp::min(fold_col + 1, self.width - fold_col - 1);

                (0..range)
                    .try_fold(0, |outer, offset| {
                        (0..self.height)
                            .try_fold(0, |inner, row| {
                                let lower = row * (self.width + 1) + (fold_col - offset);
                                let upper = lower + (2 * offset + 1);
                                let errs = usize::from(self.data[lower] != self.data[upper]);

                                if inner + errs > error_count {
                                    Err(0)
                                } else {
                                    Ok(inner + errs)
                                }
                            })
                            .and_then(|inner| {
                                if outer + inner > error_count {
                                    Err(0)
                                } else {
                                    Ok(outer + inner)
                                }
                            })
                    })
                    .map_or(false, |sum| sum == error_count)
            })
            .map(|r| r + 1)
    }

    fn summarise(&self, error_count: usize) -> usize {
        self.row_reflection(error_count)
            .map(|r| r * 100)
            .or_else(|| self.col_reflection(error_count))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(405, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(400, part_two(data));
    }
}
