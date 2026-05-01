use super::reduced::ReducedMatrix;
use super::values::Values;
use super::{Bound, COLS, ROWS};

use std::{fmt::Display, str::FromStr};

pub struct Matrix {
    pub(crate) weights: [[i32; COLS]; ROWS],
    pub(crate) result: [i32; ROWS],
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl Matrix {
    #[must_use]
    pub fn min_sum(&self) -> i32 {
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
        match reduced.get_free_bound(var, &known, bound)? {
            Bound::Upper(u) => {
                let val = Self::final_back_propagation(reduced, known, var, u)?;
                Some(val)
            }
            Bound::Lower(l) => {
                let val = Self::final_back_propagation(reduced, known, var, l)?;
                Some(val)
            }
        }
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
            // Find and move pivot row
            let Some(row_idx) =
                (pivot_row_idx..self.rows).find(|&r| weights[r][pivot_col_idx] != 0)
            else {
                continue;
            };

            if row_idx != pivot_row_idx {
                weights.swap(row_idx, pivot_row_idx);
                result.swap(row_idx, pivot_row_idx);
            }

            // Eliminate this column from all subsequent rows
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

        let matrix = Self {
            weights,
            result,
            rows: self.rows,
            cols: self.cols,
        };
        ReducedMatrix::new(matrix, pivots)
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
