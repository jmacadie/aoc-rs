#![warn(clippy::all, clippy::pedantic)]

pub mod common;
pub mod year_2020;

use year_2020::day_03;

fn main() {
    day_03::run();
}
