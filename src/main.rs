#![warn(clippy::all, clippy::pedantic)]

pub mod common;
pub mod year_2020;

use year_2020::day_04;

fn main() {
    day_04::run();
}
