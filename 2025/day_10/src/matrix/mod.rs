const COLS: usize = 15;
const ROWS: usize = 10;

pub mod base;
pub mod linear;
pub mod reduced;
pub mod values;

pub use base::Matrix;

pub(crate) enum Bound {
    Upper(i32),
    Lower(i32),
}
