use std::fmt::Display;

use super::COLS;

#[derive(Debug, Clone, Copy)]
pub struct Values {
    values: [Option<i32>; COLS],
    cols: usize,
}

impl Values {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            values: [None; COLS],
            cols: COLS,
        }
    }

    pub fn complete(&self) -> bool {
        self.values.iter().take(self.cols).all(Option::is_some)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<i32>> {
        self.values.iter().take(self.cols)
    }

    pub const fn add(&mut self, var: usize, val: i32) {
        self.values[var] = Some(val);
    }

    #[must_use]
    pub fn sum(&self) -> i32 {
        self.iter().flatten().sum()
    }

    #[must_use]
    pub const fn is_solved(&self, idx: usize) -> bool {
        self.values[idx].is_some()
    }

    pub const fn set_cols(&mut self, cols: usize) {
        self.cols = cols;
    }
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
