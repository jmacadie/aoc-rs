use super::linear::Linear;
use super::values::Values;
use super::{Bound, COLS, Matrix, ROWS};

use crate::utils::gcd::gcd;

use std::{fmt::Display, iter};

pub(crate) struct ReducedMatrix {
    matrix: Matrix,
    pivots: [Option<usize>; ROWS],
}

impl ReducedMatrix {
    pub fn new(matrix: Matrix, pivots: [Option<usize>; ROWS]) -> Self {
        let mut m = Self { matrix, pivots };
        m.normalize_rows();
        m
    }

    pub fn back_propagation(&self, known: &Values) -> Option<Values> {
        let mut determined = *known;
        determined.set_cols(self.matrix.cols);

        for row_idx in (0..self.matrix.rows).rev() {
            let Some(pivot_idx) = self.pivots[row_idx] else {
                continue;
            };

            let row = &self.matrix.weights[row_idx];

            // Pivot is already solved.
            if determined.is_solved(pivot_idx) {
                continue;
            }

            let mut acc = 0;
            let mut solvable = true;

            for (weight, det) in row
                .iter()
                .zip(determined.iter())
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

    fn normalize_rows(&mut self) {
        for (row, res) in self
            .matrix
            .weights
            .iter_mut()
            .zip(self.matrix.result.iter_mut())
        {
            let gcd = row
                .iter()
                .chain(iter::once(&*res))
                .fold(0, |acc, &x| gcd(acc, x));
            if gcd > 0 {
                let gcd = i32::try_from(gcd).unwrap();
                for e in row.iter_mut() {
                    *e /= gcd;
                }
                *res /= gcd;
            }
        }
    }

    pub fn get_free_variables(&self) -> (usize, [bool; COLS]) {
        let mut free_var = [true; COLS];
        self.pivots
            .iter()
            .flatten()
            .copied()
            .for_each(|p| free_var[p] = false);
        free_var
            .iter_mut()
            .skip(self.matrix.cols)
            .for_each(|v| *v = false);
        let count = self.matrix.cols - self.pivots.iter().flatten().count();
        (count, free_var)
    }

    pub fn get_free_bound(&self, free: usize, known: &Values, initial: i32) -> Option<Bound> {
        let expressions: Vec<Linear> = self
            .linear_solution(known, free)
            .iter()
            .take(self.matrix.cols)
            .flatten()
            .copied()
            .filter(|&e| !e.is_const())
            .collect();

        let mut lower = 0;
        let mut upper = initial;

        let is_upper = expressions
            .iter()
            .take(self.matrix.cols)
            .copied()
            .reduce(|acc, x| acc + x)
            .unwrap()
            .is_upper_bound();

        // First constrain by non-negativity
        for expr in &expressions {
            match expr.as_bound() {
                Some(Bound::Lower(v)) => lower = lower.max(v),
                Some(Bound::Upper(v)) => upper = upper.min(v),
                None => {}
            }

            if lower > upper {
                return None;
            }
        }

        // Now constrain by integrality
        let valid = |x| expressions.iter().all(|expr| expr.is_integral_at(x));

        if is_upper {
            let upper = (lower..=upper).rev().find(|&x| valid(x))?;
            Some(Bound::Upper(upper))
        } else {
            let lower = (lower..=upper).find(|&x| valid(x))?;
            Some(Bound::Lower(lower))
        }
    }

    fn linear_solution(&self, known: &Values, free: usize) -> [Option<Linear>; COLS] {
        let mut values = [None; COLS];

        // Existing known values are constants
        for (i, value) in known.iter().enumerate() {
            if let Some(value) = value.as_ref() {
                values[i] = Some(value.into());
            }
        }

        // x = x
        values[free] = Some(Linear::variable());

        // Same bottom-up traversal as back_propagation()
        for row_idx in (0..self.matrix.rows).rev() {
            let Some(pivot) = self.pivots[row_idx] else {
                continue;
            };

            if values[pivot].is_some() {
                continue;
            }

            let row = &self.matrix.weights[row_idx];

            let mut rhs: Linear = self.matrix.result[row_idx].into();

            for col in pivot + 1..self.matrix.cols {
                let weight = row[col];

                if weight == 0 {
                    continue;
                }

                let value =
                    values[col].expect("single free variable should make every RHS value known");

                rhs -= value * weight;
            }

            let raw = rhs / row[pivot];

            // Reduce the raw Linear number to keep the numbers from overflowing
            values[pivot] = Some(raw.reduced());
        }

        values
    }
}

impl Display for ReducedMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)?;
        Ok(())
    }
}
