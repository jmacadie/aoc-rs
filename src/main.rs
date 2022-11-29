#![warn(clippy::all, clippy::pedantic)]

pub mod common;
pub mod year_2020;

use year_2020::day_07;

fn main() {
    day_07::run();
}
