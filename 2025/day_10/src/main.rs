#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(trim_prefix_suffix)]

use std::{fmt::Display, num::ParseIntError, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(|l| l.parse::<Light>().unwrap())
        .map(|l| l.find_min_presses())
        .sum()
}

fn part_two(data: &str) -> i32 {
    data.lines()
        .map(|l| l.parse::<Matrix>().unwrap())
        .map(|m| m.min_sum())
        .sum()
}

const COLS: usize = 15;
const ROWS: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Values {
    values: [Option<i32>; COLS],
    cols: usize,
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for val in self.values.iter().take(self.cols) {
            if val.is_none() {
                write!(f, "None ")?;
            } else {
                write!(f, "{:4} ", val.unwrap())?;
            }
        }
        Ok(())
    }
}

impl Values {
    const fn empty() -> Self {
        Self {
            values: [None; COLS],
            cols: COLS,
        }
    }

    fn complete(&self) -> bool {
        self.values.iter().take(self.cols).all(Option::is_some)
    }

    fn iter(&self) -> impl Iterator<Item = &Option<i32>> {
        self.values.iter().take(self.cols)
    }

    const fn add(&mut self, var: usize, val: i32) {
        self.values[var] = Some(val);
    }

    fn sum(&self) -> i32 {
        self.iter().flatten().sum()
    }
}

struct Matrix {
    weights: [[i32; COLS]; ROWS],
    result: [i32; ROWS],
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn min_sum(&self) -> i32 {
        let reduced = self.row_echelon();
        let (count, free) = reduced.get_free_variables();
        self.min_sum_inner(&reduced, &Values::empty(), count, free, i32::MAX)
    }

    fn min_sum_inner(
        &self,
        reduced: &ReducedMatrix,
        known: &Values,
        free_count: usize,
        mut free: [bool; COLS],
        mut best: i32,
    ) -> i32 {
        let Some(propagated_vals) = reduced.back_propagation(known) else {
            return best;
        };

        let current_sum: i32 = propagated_vals.sum();
        if current_sum >= best {
            return best;
        }

        if propagated_vals.complete() {
            return current_sum;
        }

        let (var, bound) = self.get_next_free_variable(&free, &propagated_vals);
        if free_count == 1 {
            if let Some(sum) = Self::min_sum_last(reduced, propagated_vals, var, bound) {
                return sum.min(best);
            }
            return best;
        }
        free[var] = false;

        for val in 0..=bound {
            let mut next = propagated_vals;
            next.add(var, val);

            best = self.min_sum_inner(reduced, &next, free_count - 1, free, best);
        }

        best
    }

    fn min_sum_last(reduced: &ReducedMatrix, known: Values, var: usize, bound: i32) -> Option<i32> {
        let lower =
            (0..=bound).find_map(|val| Self::final_back_propagation(reduced, known, var, val))?;

        let upper = (0..=bound)
            .rev()
            .find_map(|val| Self::final_back_propagation(reduced, known, var, val))?;

        Some(lower.min(upper))
    }

    fn final_back_propagation(
        reduced: &ReducedMatrix,
        mut known: Values,
        var: usize,
        val: i32,
    ) -> Option<i32> {
        known.add(var, val);
        let solution = reduced.back_propagation(&known)?;
        assert!(solution.complete());
        Some(solution.sum())
    }

    fn get_next_free_variable(&self, free: &[bool; COLS], known: &Values) -> (usize, i32) {
        free.iter()
            .enumerate()
            .filter(|&(_, f)| *f)
            .map(|(i, _)| (i, self.get_bound(i, known)))
            .min_by_key(|&(_, bound)| bound)
            .unwrap()
    }

    fn get_bound(&self, variable: usize, known: &Values) -> i32 {
        self.weights
            .iter()
            .zip(self.result.iter())
            .filter(|&(row, _)| row[variable] != 0)
            .map(|(row, res)| {
                res - row
                    .iter()
                    .zip(known.iter())
                    .filter_map(|(val, known)| known.map(|k| *val * k))
                    .sum::<i32>()
            })
            .min()
            .unwrap()
    }

    fn row_echelon(&self) -> ReducedMatrix {
        // Using Bariess elimination
        let mut weights = self.weights;
        let mut result = self.result;
        let mut pivots = [None; ROWS];

        let mut pivot_row_idx = 0;
        let mut previous_factor = 1;

        for pivot_col_idx in 0..self.cols {
            // Find and move pivot row.
            let Some(row_idx) =
                (pivot_row_idx..self.rows).find(|&r| weights[r][pivot_col_idx] != 0)
            else {
                continue;
            };

            if row_idx != pivot_row_idx {
                weights.swap(row_idx, pivot_row_idx);
                result.swap(row_idx, pivot_row_idx);
            }

            // Eliminate this column from all subsequent rows.
            let pivot_row = weights[pivot_row_idx];
            let pivot_factor = pivot_row[pivot_col_idx];

            for row_idx in pivot_row_idx + 1..self.rows {
                let factor = weights[row_idx][pivot_col_idx];

                for (cell, pvt_cell) in weights[row_idx]
                    .iter_mut()
                    .zip(pivot_row)
                    .take(self.cols)
                    .skip(pivot_col_idx + 1)
                {
                    let gross = *cell * pivot_factor - pvt_cell * factor;
                    assert_eq!(gross.rem_euclid(previous_factor), 0);
                    *cell = gross / previous_factor;
                }

                let gross = result[row_idx] * pivot_factor - result[pivot_row_idx] * factor;
                assert_eq!(gross.rem_euclid(previous_factor), 0);
                result[row_idx] = gross / previous_factor;

                weights[row_idx][pivot_col_idx] = 0;
            }

            pivots[pivot_row_idx] = Some(pivot_col_idx);
            previous_factor = pivot_factor;
            pivot_row_idx += 1;

            if pivot_row_idx == self.rows {
                break;
            }
        }

        ReducedMatrix {
            matrix: Self {
                weights,
                result,
                rows: self.rows,
                cols: self.cols,
            },
            pivots,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.weights.iter().enumerate().take(self.rows) {
            for val in row.iter().take(self.cols) {
                if *val == 0 {
                    write!(f, "  - ")?;
                } else {
                    write!(f, "{val:3} ")?;
                }
            }
            write!(f, "| {}", self.result[idx])?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Matrix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s
            .split_once(' ')
            .ok_or_else(|| format!("Cannot split {s}"))?;
        let (buttons, joltage) = rest
            .rsplit_once(' ')
            .ok_or_else(|| format!("Cannot split off the joltage at the end of {rest}"))?;

        let mut weights = [[0; COLS]; ROWS];

        for (col, button) in buttons.split(' ').enumerate() {
            for num in button.trim_prefix('(').trim_suffix(')').split(',') {
                let row = num
                    .parse::<usize>()
                    .map_err(|_| format!("Cannot parse {num} into a number"))?;
                weights[row][col] = 1;
            }
        }
        let mut result = [0; ROWS];
        for (idx, num) in joltage
            .trim_prefix('{')
            .trim_suffix('}')
            .split(',')
            .enumerate()
        {
            let parsed = num
                .parse::<i32>()
                .map_err(|_| format!("Cannont parse {num} into a number"))?;
            result[idx] = parsed;
        }

        let rows = joltage.split(',').count();
        let cols = buttons.split(' ').count();

        Ok(Self {
            weights,
            result,
            rows,
            cols,
        })
    }
}

struct ReducedMatrix {
    matrix: Matrix,
    pivots: [Option<usize>; ROWS],
}

impl ReducedMatrix {
    fn back_propagation(&self, known: &Values) -> Option<Values> {
        let mut determined = *known;
        determined.cols = self.matrix.cols;

        for row_idx in (0..self.matrix.rows).rev() {
            let Some(pivot_idx) = self.pivots[row_idx] else {
                continue;
            };

            let row = &self.matrix.weights[row_idx];

            // Pivot is already solved.
            if determined.values[pivot_idx].is_some() {
                continue;
            }

            let mut acc = 0;
            let mut solvable = true;

            for (weight, det) in row
                .iter()
                .zip(determined.values.iter())
                .take(self.matrix.cols)
                .skip(pivot_idx + 1)
                .filter(|&(w, _)| *w != 0)
            {
                // Cannot solve row as have unknown beyond the pivot
                // Quit row accumulator and mark to skip row
                let Some(value) = det else {
                    solvable = false;
                    break;
                };
                acc += weight * value;
            }

            // Cannot solve row as have unknown beyond the pivot
            // Skip row
            if !solvable {
                continue;
            }

            let pivot_factor = row[pivot_idx];
            let numerator = self.matrix.result[row_idx] - acc;

            // Invalid state: Must have integer solution
            // Abort entire back propagation
            if numerator.rem_euclid(pivot_factor) != 0 {
                return None;
            }

            let value = numerator / pivot_factor;

            // Invalid state: Must have non-negative solution
            // Abort entire back propagation
            if value < 0 {
                return None;
            }

            determined.add(pivot_idx, value);
        }

        Some(determined)
    }

    fn get_pivot_variables(&self) -> [bool; COLS] {
        let mut pivots = [false; COLS];
        for row in self.matrix.weights {
            for (i, cell) in row.iter().enumerate() {
                if *cell != 0 {
                    pivots[i] = true;
                    break;
                }
            }
        }
        pivots
    }

    fn get_free_variables(&self) -> (usize, [bool; COLS]) {
        let mut free_var = [false; COLS];
        let mut count = 0;
        self.get_pivot_variables()
            .iter()
            .enumerate()
            .take(self.matrix.cols)
            .filter(|&(_, p)| !*p)
            .for_each(|(i, _)| {
                free_var[i] = true;
                count += 1;
            });
        (count, free_var)
    }
}

#[derive(Debug)]
struct Light {
    target: u16,
    buttons: Vec<u16>,
}

impl Light {
    fn find_min_presses(&self) -> usize {
        let mut frontier = Vec::new();
        let mut next_frontier = vec![0];
        let mut counter = 0;
        let mut seen = [false; 1024];
        loop {
            std::mem::swap(&mut frontier, &mut next_frontier);
            counter += 1;
            while let Some(next) = frontier.pop() {
                for b in &self.buttons {
                    // XOR to press button
                    let new = next ^ b;
                    if new == self.target {
                        return counter;
                    }
                    let idx = usize::from(new);
                    if seen[idx] {
                        continue;
                    }
                    seen[idx] = true;
                    next_frontier.push(new);
                }
            }
        }
    }
}

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Target:")?;
        writeln!(f, " {:010b}", self.target)?;
        writeln!(f, "Buttons:")?;
        for b in &self.buttons {
            writeln!(f, " {b:010b}")?;
        }
        Ok(())
    }
}

impl FromStr for Light {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_txt, rest) = s
            .split_once(' ')
            .ok_or_else(|| format!("Cannot split {s}"))?;
        let (buttons_txt, _) = rest
            .rsplit_once(' ')
            .ok_or_else(|| format!("Cannot split off the joltage at the end of {rest}"))?;

        let target = target_txt
            .char_indices()
            .filter(|&(_, c)| c == '#')
            .fold(0u16, |acc, (i, _)| acc | 1 << (i - 1));

        let mut buttons = Vec::new();
        let make_button = |numbers: &str| {
            let mut button: u16 = 0u16;
            for num in numbers.trim_prefix('(').trim_suffix(')').split(',') {
                let parsed = num.parse::<u16>()?;
                button |= 1 << parsed;
            }
            Ok(button)
        };
        for button in buttons_txt.split(' ') {
            let b = make_button(button).map_err(|_: ParseIntError| {
                format!("Cannot format button text into a u16: {button}")
            })?;
            buttons.push(b);
        }

        Ok(Self { target, buttons })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(7, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(33, part_two(data));
    }
}
